# Spike DMX-7: DMX4ALL Mini-USB throughput, latency and update-loop jitter

Throwaway spike; findings move to `docs/research/` on the DMX-4 branch.

## Setup

- Interface: DMX4ALL USB-DMX-INTERFACE, FTDI VID 0403 / PID c850, firmware V3.36, 38400 baud,
  International pinout, not in blackout. `N?` answers `047`.
- Host: Fedora Linux 44, `/dev/ttyUSB0`, user in `dialout`.
- Host: Windows 11 Pro 10.0.26300, `COM3`, FTDI driver 2.12.28.0; Rust 1.100.0 (MSVC), .NET
  10.0.12 (self-contained win-x64 single file).
- Probes: `rust/` (serial2 0.2.39) and `dotnet/` (.NET 10.0.13, System.IO.Ports 11.0.0).
- Test traffic at channel 100 and up; the visual sweep drives Moving Head 1 at address 1.

## Linux, FTDI latency timer 16 ms (default), Rust

| Block | Write + ack (median) |
|---|---|
| 1 to 39 channels | 16 ms |
| 64 channels | 32 ms |
| 128 channels | 48 ms |
| 255 channels | 80 ms |

Every acknowledgement lands on a 16 ms boundary: the FTDI chip holds the reply until its latency
timer runs out. At 44 Hz with 39 channels: 41.8 Hz achieved, half the updates overran.

## Linux, latency timer 1 ms, Rust

Write + ack is wire time plus about 1 ms: 1 channel 2.0 ms, 39 channels 12.0 ms, 64 channels
18.9 ms, 128 channels 35.9 ms, 255 channels 68.9 ms.

| Target | Channels per update | Achieved | Overruns | Wake jitter max | Write + ack median |
|---|---|---|---|---|---|
| 44 Hz | 39 | 44.0 Hz | 0 | 0.00 ms | 12.2 ms |
| 44 Hz | 47 | 44.0 Hz | 0 | 0.42 ms | 14.3 ms |
| 44 Hz | 64 | 44.0 Hz | 0 | 0.00 ms | 18.7 ms |
| 44 Hz | 128 | 27.9 Hz | all | 0.00 ms | 35.9 ms |
| 20 Hz | 255 | 14.5 Hz | all | 0.00 ms | 68.9 ms |

The 38400-baud line is the limit: about 80 changed channels per update at 44 Hz; a full universe
(512 channels) takes about 140 ms, about 7 Hz.

Visual sweep, Moving Head 1, 44 Hz, 16-bit pan: smooth (owner, 2026-11-26). Tilt at mid pointed
straight up, while the v1 config marks the head as hanging (pitch 180°): mounting or tilt direction
in the profile to check during calibration.

## Linux, latency timer 1 ms, .NET

| Target | Wait | Channels | Achieved | Overruns | Wake jitter p99 / max | Write + ack median / max | GC gen0/1/2 |
|---|---|---|---|---|---|---|---|
| 44 Hz | spin | 39 | 44.0 Hz | 0 | 0.00 / 0.80 ms | 13.0 / 17.0 ms | 0/0/0 |
| 44 Hz | sleep | 39 | 44.0 Hz | 0 | 1.07 / 1.07 ms | 12.8 / 15.4 ms | 0/0/0 |
| 44 Hz | spin | 64 | 44.0 Hz | 3 | 0.00 / 0.37 ms | 19.4 / 23.1 ms | 0/0/0 |
| 44 Hz | sleep | 64 | 44.0 Hz | 2 | 0.96 / 1.02 ms | 18.8 / 23.1 ms | 0/0/0 |
| 44 Hz, 30 s | spin | 47 | 44.0 Hz | 0 | 0.00 / 0.57 ms | 15.1 / 17.9 ms | 0/0/0 |

No garbage collection ran; the loop allocates little, so this is optimistic for an engine.

Visual sweep from .NET, Moving Head 1, 44 Hz, 16-bit pan, green: smooth, no visible difference from
the Rust sweep (owner, 2026-11-26). Write + ack median 6.4 ms, max 11.8 ms (Rust: 5.6 ms, max 6.4 ms).

## Windows, latency timer 16 ms (default)

Write + ack matches Linux: 16 ms up to 39 channels, 32 ms at 64, 48 ms at 128, 80 ms at 255.

| Probe | Wait | Channels | Achieved | Overruns | Wake jitter p99 / max | Write + ack median / max |
|---|---|---|---|---|---|---|
| Rust | spin | 39 | 41.7 Hz | 220 of 440 | 0.00 / 0.01 ms | 24.7 / 26.0 ms |
| Rust | spin | 64 | 31.3 Hz | all | 0.00 / 0.00 ms | 32.0 / 32.1 ms |
| .NET | sleep | 39 | 40.0 Hz | 220 of 440 | 15.3 / 16.1 ms | 16.0 / 27.5 ms |
| .NET | spin | 39 | 41.4 Hz | 220 of 440 | 13.5 / 14.0 ms | 16.0 / 27.9 ms |
| .NET | spin | 64 | 31.3 Hz | all | 0.00 / 0.27 ms | 32.0 / 32.5 ms |
| .NET, 30 s | sleep | 47 | 38.8 Hz | 660 of 1320 | 15.2 / 15.8 ms | 16.0 / 30.1 ms |

The latency timer limits both probes, as on Linux. `N?` answers `162` in this run (after the
.NET runs wrote 64 channels from channel 100, last 0-based index 162) and `047` after a replug.

## Windows, latency timer 1 ms

Write + ack is wire time plus about 1 ms, as on Linux: 1 channel 2.0 ms, 39 channels 12.0 ms,
64 channels 19.0 ms, 128 channels 36.0 ms, 255 channels 69.0 ms.

| Probe | Wait | Channels | Achieved | Overruns | Wake jitter p99 / max | Write + ack median / max |
|---|---|---|---|---|---|---|
| Rust | spin | 39 | 44.0 Hz | 0 | 0.00 / 0.01 ms | 12.2 / 13.0 ms |
| Rust | spin | 64 | 44.0 Hz | 0 | 0.00 / 0.09 ms | 19.1 / 19.7 ms |
| .NET | sleep | 39 | 42.8 Hz | 215 of 440 | 13.1 / 14.1 ms | 12.1 / 13.6 ms |
| .NET | spin | 39 | 42.8 Hz | 216 of 440 | 13.3 / 13.3 ms | 12.0 / 13.1 ms |
| .NET | spin | 64 | 42.9 Hz | 219 of 440 | 6.3 / 12.3 ms | 19.0 / 20.3 ms |
| .NET, 30 s | sleep | 47 | 42.8 Hz | 652 of 1320 | 10.9 / 16.6 ms | 14.8 / 15.7 ms |

Rust matches Linux. .NET misses about half its deadlines in both wait modes, with no GC and with
write + ack equal to Rust: the wake comes late, not the write. The lateness reaches the 15.6 ms
default Windows timer tick; `Thread.Sleep` follows that tick, while Rust's `thread::sleep` uses a
high-resolution waitable timer on Windows.

Visual sweeps, Moving Head 1, 44 Hz (owner, 2026-11-27): smooth at both latency settings, no
visible difference between Rust and .NET. Write + ack median 13.4 ms at 16 ms, 5.9 ms at 1 ms
(Rust 5.88, .NET 5.96).

## Open

- .NET on Windows: rerun with `timeBeginPeriod(1)` or a high-resolution waitable timer to confirm
  the timer tick as the cause of the missed deadlines.
- Latency timer 1 ms is a per-device driver setting: set it at install, from the application
  (FTDI D2XX `FT_SetLatencyTimer`), or check it at startup.
- What `N?` means: `047` after power-up, `162` after a 64-channel block from channel 100. Test:
  replug, `info`, one block at a known high channel, `info`.
- Pipelined writes (next block before the previous acknowledgement).
- .NET under allocation pressure.
