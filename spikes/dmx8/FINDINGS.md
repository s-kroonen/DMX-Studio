# Spike DMX-8: audio loopback capture and tone-to-onset delay

Throwaway spike; findings move to `docs/research/stack-audio.md` on the DMX-4 branch.

## Method

Each probe plays a 20 ms 1 kHz burst (amplitude 0.05) every 500 ms on an output device and
captures the same audio back: on Linux from the output's monitor source, on Windows through WASAPI
loopback (the default output opened as a capture device). The delay runs from the moment the burst
is written in the output callback to the capture callback that first sees a sample above 0.01 after
200 ms of quiet. It covers the output buffer, the sound server and the capture buffer: the time an
analysis engine waits for audio that another application plays.

- `rust/`: cpal 0.18.2 (Apache-2.0); hosts PulseAudio (protocol in Rust, no C library) and PipeWire
  (feature `pipewire`, needs `pipewire-devel` and `clang`). Every Linux build needs
  `alsa-lib-devel`.
- `dotnet/`: SoundFlow 1.4.2 (MIT, miniaudio inside; native libraries for Linux and Windows in the
  package). One library for both systems; loopback through `InitializeLoopbackDevice` only on
  WASAPI, on Linux the monitor source is an ordinary capture device.

Usage:

```text
dmx8-probe hosts
dmx8-probe <host> list
dmx8-probe <host> latency <output name part|default> <capture name part> <seconds> [buffer frames]
Dmx8Probe list
Dmx8Probe latency <output name part|default> <capture name part|loopback> <seconds> [period frames]
```

## Linux

Fedora Linux 44, PipeWire 1.6.9 with its PulseAudio server, 48 kHz stereo. Output to a null sink
(`pactl load-module module-null-sink sink_name=dmx8_null`), capture from its monitor: silent, and
the same path as a hardware output's monitor up to the sink. 15 s per run, 29 to 30 bursts; the
first burst of a run is sometimes missed while the streams start.

| Probe, host | Buffer (frames) | Callback blocks out / in | Tone to onset median | p95 | Max |
|---|---|---|---|---|---|
| Rust, PipeWire | default | 1024 / 1024 | 32.3 ms | 41.3 ms | 42.7 ms |
| Rust, PipeWire | 256 | 256 / 256 | 8.2 ms | 10.7 ms | 11.1 ms |
| Rust, PipeWire | 128 | 128 / 128 | 4.5 ms | 5.4 ms | 5.4 ms |
| Rust, PipeWire | 64 | 64 / 64 | 2.7 ms | 2.7 ms | 3.1 ms |
| Rust, PulseAudio | default | 2048 (first 96000) / 8192 | about 400 ms | | |
| Rust, PulseAudio | 256 | 256 / 256 | 18.7 ms | 21.4 ms | 21.4 ms |
| Rust, PulseAudio | 128 | 256 / 128 | 18.7 ms | 21.4 ms | 21.6 ms |
| .NET, SoundFlow | default | 900 / 1200 | 46.2 ms | 50.6 ms | 51.9 ms |
| .NET, SoundFlow | 256 | 256 / 256 | 24.8 ms | 26.8 ms | 26.9 ms |
| .NET, SoundFlow | 128 | 128 / 128 | 21.4 ms | 26.7 ms | 27.6 ms |
| .NET, SoundFlow | 64 | 64 / 64 | 21.3 ms | 26.8 ms | 27.1 ms |

- The native PipeWire host follows the requested buffer down to 64 frames: 2.7 ms, below one DMX
  frame at 44 Hz (22.7 ms).
- Through the PulseAudio protocol (cpal's PulseAudio host and miniaudio) the delay stops falling at
  about 19 to 21 ms, whatever buffer is requested; `pipewire-pulse` sets its own latency.
  ASSUMPTION: the floor is the pipewire-pulse default quantum (1024 frames, 21.3 ms), not measured.
- cpal's PulseAudio host with default buffers starts with a 2 s output fill and 170 ms capture
  blocks; the pairing of bursts fails at that size. An engine sets the buffer size explicitly.
- No garbage collection ran in any .NET run.
- SoundFlow reports the active miniaudio backend as `Oss` while it talks to the PulseAudio server
  (device names "Monitor of ..."): its backend enum looks shifted by one against miniaudio's.
- All results stay within the 44 Hz DMX frame budget once the buffer is set; only cpal with
  PipeWire gets below 10 ms on Linux.

Not measured: the hardware output latency after the sink (what the audience hears), a microphone
input, CPU load while capturing, and SoundFlow with miniaudio's JACK backend through PipeWire.

## Windows

Windows 11 Pro 10.0.26300, WASAPI shared mode, loopback of the default output. .NET 10.0.15
(self-contained win-x64), Rust 1.100.0 (MSVC). 15 s per run, 30 bursts, all 30 paired in every
.NET run. Two default outputs: SteelSeries Sonar - Gaming (virtual mixer, 8 ch 96 kHz) and
Speakers (JBL Quantum 650 Wireless Game, 8 ch 48 kHz, wireless headset).

| Probe, output | Period (frames) | Callback blocks out / in | Tone to onset median | p95 | Max |
|---|---|---|---|---|---|
| .NET, Sonar | default | 480 / 480 | 50.1 ms | 51.2 ms | 51.4 ms |
| .NET, Sonar | 480 | 480 / 480 | 50.3 ms | 51.4 ms | 51.4 ms |
| .NET, Sonar | 256 | 256 / 256 | 27.8 ms | 30.9 ms | 35.7 ms |
| .NET, Sonar | 128 | 128 / 128 | 29.0 ms | 30.3 ms | 30.6 ms |
| .NET, JBL | default | 480 / 480 | 30.0 ms | 30.0 ms | 30.1 ms |
| .NET, JBL | 480 | 480 / 480 | 30.0 ms | 30.7 ms | 30.7 ms |
| .NET, JBL | 256 | 256 / 256 | 28.0 ms | 30.7 ms | 30.7 ms |
| .NET, JBL | 128 | 128 / 128 | 28.7 ms | 30.7 ms | 30.7 ms |

- WASAPI shared mode accepts 256 and 128 frames, but the delay stays at 28 to 30 ms below 480
  frames: the floor lies in the shared-mode engine, not in the callback size.
- The Sonar virtual mixer adds about 20 ms at 480 frames.
- SoundFlow reports the miniaudio backend as `Null` while it uses WASAPI loopback: the same enum
  shift as `Oss` on Linux.
- No garbage collection ran.
- Rust: every run stops before audio starts with `no capture device matches "<name>"`. `find()`
  accepts a capture device only when `default_input_config()` succeeds, and cpal lists no input
  config for WASAPI output devices.

## Open

- Rust on Windows: open the output device as the capture device for loopback.
- Linux: the pipewire-pulse quantum as the cause of the 21 ms floor (`pw-metadata -n settings`).
- Analysis cost per block (band levels, onset and BPM) on top of the capture.
