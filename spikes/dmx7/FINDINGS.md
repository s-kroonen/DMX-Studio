# Spike DMX-7: DMX4ALL Mini-USB throughput, latency and update-loop jitter

Throwaway spike; findings move to `docs/research/` on the DMX-4 branch.

## Setup

- Interface: DMX4ALL USB-DMX-INTERFACE, FTDI VID 0403 / PID c850, firmware V3.36, 38400 baud,
  International pinout, not in blackout. `N?` answers `047`.
- Host: Fedora Linux 44, `/dev/ttyUSB0`, user in `dialout`.
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

## Open

- Windows: the same runs, with the FTDI latency timer at 16 ms and 1 ms (Device Manager, port
  settings, advanced).
- What `N?` = 47 means for output (slots per DMX frame) and whether it can be raised.
- Pipelined writes (next block before the previous acknowledgement).
- .NET under allocation pressure.
