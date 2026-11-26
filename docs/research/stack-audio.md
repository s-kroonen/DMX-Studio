# Stack research: audio capture and analysis

- **Concern**: C-8 (drivers: QA-1, QA-3)
- **Issue**: DMX-4
- **Date**: 2026-11-26
- **Question**: Which language (C#/.NET, Rust, Dart) captures microphone, line-in and loopback audio on Windows and Linux with low latency, and analyses band levels, beat and BPM?
- **Time-box**: 4h
- **Status**: answered
- **Sources**: github.com/RustAudio/cpal README and code search in `src/host` (read 2026-11-26); github.com/LSXPrime/SoundFlow README (read 2026-11-26); v1 README "Sound-to-light", github.com/s-kroonen/DMX-Controller (read 2026-11-26)
- **Verified**: `scripts/verify-package`, 2026-11-26: nuget NAudio 3.3.0 (MIT, 2026-10-19); nuget NAudio.Wasapi 22.0.0 (MIT, release date not given); nuget SoundFlow 1.4.2 (licence not stated by the registry; repository MIT, 529 stars, pushed 2026-10-22); nuget PortAudioSharp2 1.0.6 (licence not stated, 2025-10-16); nuget ManagedBass 4.0.2 (licence not stated, 2025-10-03; wraps the proprietary BASS library, ASSUMPTION); nuget FftSharp 2.2.0 (MIT, 2024-11-16); nuget MathNet.Numerics 5.0.0 (MIT, 2022-04-03); crates cpal 0.18.2 (Apache-2.0, 2026-08-16); crates rustfft 6.4.1 (MIT OR Apache-2.0, 2025-09-18); crates realfft 3.5.1 (MIT, 2026-10-26); crates pipewire 0.10.1 (MIT, 2026-08-19); crates libpulse-binding 2.30.1 (MIT OR Apache-2.0, 2025-04-19); crates aubio-rs 0.2.0 (GPL-3.0, 2021-04-30); pub fftea 1.5.0+1 (Apache-2.0, 2024-07-14); pub flutter_audio_capture 1.1.12 (MIT, 2025-12-28); pub record 7.2.0 (BSD-3-Clause, 2026-11-11)
- **Re-check by**: 2027-02-24

## Question

Sound-to-light (basic) is in the first release: input and loopback capture, level and band meters,
beat detection, BPM and tap tempo (vision). Which language does capture and analysis on Windows
and Linux?

## Previous version (remakes)

v1 captured through PyAudioWPatch (WASAPI loopback) on Windows only; Linux capture did not exist.
The analysis (auto-gain, bass/mid/high bands, beat recogniser, BPM, tap tempo) and the six effects
worked at events and are worth keeping as behaviour.

## Options

| Option | Package(s) verified (version, last release, licence, checked on) | Notes |
|---|---|---|
| .NET | NAudio 3.3.0 (Windows WASAPI, loopback); SoundFlow 1.4.2 (miniaudio, cross-platform); FftSharp 2.2.0 (2026-11-26) | Two capture paths: NAudio on Windows, another on Linux |
| Rust | cpal 0.18.2 (WASAPI, ALSA, PulseAudio, PipeWire, JACK); realfft 3.5.1, rustfft 6.4.1 (2026-11-26) | One capture library; beat detection written for the project (aubio-rs is GPL-3.0 and stale) |
| Dart | record 7.2.0, flutter_audio_capture 1.1.12; fftea 1.5.0+1 (2026-11-26) | Microphone capture; no desktop loopback found |

## Criteria and weights (agreed with the owner before scoring)

Agreed 2026-11-26.

| Criterion | Weight | Source (QA-…) |
|---|---|---|
| Loopback capture on Windows (WASAPI) and Linux (PipeWire or PulseAudio) | 3 | QA-3, vision |
| Low latency from sound to beat event | 3 | QA-1 |
| DSP library quality (FFT, onset or beat detection) | 2 | vision |
| Licence fits | 2 | `scripts/check licenses` |

## Scores

Scores 1–5; weighted total out of 50.

| Option | Loopback (3) | Latency (3) | DSP (2) | Licence (2) | Weighted total | Reason |
|---|---|---|---|---|---|---|
| Rust | 4 | 5 | 4 | 5 | **45** | cpal lists WASAPI on Windows and PulseAudio and PipeWire on Linux, with loopback code in `src/host/wasapi/device.rs` and monitor code in the PulseAudio and PipeWire hosts (code presence, not tested); no GC; realfft active; beat detection by hand |
| .NET | 3 | 4 | 3 | 5 | **37** | NAudio covers Windows loopback only; Linux through SoundFlow (miniaudio) or PortAudio, loopback there not verified (ASSUMPTION); FFT libraries last released 2022 and 2024 |
| Dart | 1 | 3 | 3 | 5 | **28** | Packages found capture the microphone; no desktop loopback found (ASSUMPTION) |

## Toolability (dev-standard gates)

Same as [stack-toolability.md](stack-toolability.md). The analysis code (bands, beat, BPM) is pure
computation and testable with recorded samples in every option; capture needs a manual or spike
check per platform.

## Evidence

- cpal backends: Linux ALSA with JACK, PipeWire and PulseAudio optional; Windows WASAPI with ASIO and JACK optional; ALSA development files needed to build on Linux. — [cpal README, read 2026-11-26] (high)
- cpal code search: "loopback" in `src/host/wasapi/device.rs`; "monitor" in `src/host/pulseaudio/stream.rs` and in the PipeWire host. — [github.com/RustAudio/cpal, read 2026-11-26] (medium: code presence, not behaviour)
- SoundFlow: cross-platform (Windows, macOS, Linux, Android, iOS, FreeBSD) on the miniaudio backend, with FFT spectrum analysis and level metering. — [SoundFlow README, read 2026-11-26] (high for the claim)
- aubio-rs is GPL-3.0 with its last release in 2021: not an option under the licence gate. — [verify-package crates, 2026-11-26] (high)
- v1 beat detection and band analysis were written for the project, not taken from a library. — [v1 README "Sound-to-light", read 2026-11-26] (medium)

## Evidence against the leading option

Loopback in cpal is inferred from code, not seen working; on Linux, loopback depends on the sound
server exposing a monitor source, which differs per setup. The .NET path through NAudio is proven on
Windows (v1 used the same WASAPI mechanism through Python). The Rust lead (45 against 37) is mostly
the loopback and latency scores, both unverified.

## Spike needed?

Yes: the existing audio spike (`TODO.md`) captures loopback on Windows and Linux in the leading
language and measures the delay from a test tone to a detected onset.

## Recommendation (confidence: medium)

**Rust with cpal** for capture and realfft for analysis, beat detection written for the project as
in v1, subject to the audio spike. **.NET** works with two capture paths (NAudio on Windows, a
second library on Linux). **Dart** lacks desktop loopback.

## Open questions

- Does cpal capture loopback on Windows and a monitor source on Linux (PipeWire on Fedora)?
- Delay from sound to beat event at 44 Hz output.
- Browser and phone microphone (Next tier): capture in the client and send levels to the host, or stream audio?
