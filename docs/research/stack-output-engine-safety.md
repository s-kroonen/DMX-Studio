# Stack research: DMX output, show engine and safety enforcement

- **Concern**: C-1, C-2, C-3 (drivers: QA-1, QA-2, QA-4, QA-8, ASR-1, ASR-2, ASR-3, ASR-4, ASR-7)
- **Issue**: DMX-4
- **Date**: 2026-11-26
- **Question**: Which language (C#/.NET, Rust, Dart) best drives the DMX4ALL interface at up to 44 Hz with low jitter, merges show sources into one frame, and enforces one safety check with a safe state on failure?
- **Time-box**: 1 day
- **Status**: answered
- **Sources**: github.com/dotnet/runtime `src/libraries/System.IO.Ports/src/System/IO/Ports` (Unix and Windows sources, read 2026-11-26); github.com/serialport/serialport-rs README (read 2026-11-26); github.com/alexheretic/spin-sleep README (read 2026-11-26); v1 `docs/DMX4ALL_PROTOCOL.md`, github.com/s-kroonen/DMX-Controller (read 2026-11-26); [stack-toolability.md](stack-toolability.md)
- **Verified**: `scripts/verify-package`, 2026-11-26: nuget System.IO.Ports 11.0.0 (MIT, 2026-11-10); nuget RJCP.SerialPortStream 3.0.5 (MS-PL, 2026-03-26); nuget System.Management 11.0.0 (MIT, 2026-11-10); crates serialport 4.10.1 (MPL-2.0, 2026-09-08); crates tokio-serial 5.5.0 (MIT, 2026-06-15); crates serial2 0.2.39 (BSD-2-Clause OR Apache-2.0, 2026-10-26); pub libserialport 0.3.0+1 (LGPL-3.0, 2022-08-27); pub flutter_libserialport 0.6.0 (MIT, 2025-08-01); nuget Microsoft.Extensions.TimeProvider.Testing 11.0.1 (MIT, 2026-11-21); nuget TngTech.ArchUnitNET 0.13.5 (Apache-2.0, 2026-10-26); nuget NetArchTest.Rules 1.3.2 (licence not stated, last release 2021-05-23); crates spin_sleep 1.3.3 (Apache-2.0, 2025-09-06); crates tokio 1.54.1 (MIT, 2026-10-21); crates mock_instant 0.6.1 (0BSD, 2026-06-08); pub fake_async 1.3.3 (Apache-2.0, 2025-01-28)
- **Re-check by**: 2027-02-24

## Question

The output sends changed channels to the DMX4ALL interface at a configurable rate up to 44 Hz and
lowers the rate when acknowledgements fall behind (ASR-1). Live control, cues, effects and sound
merge into one frame (ASR-7). Every value passes one safety check on the output side, and a
failure leads to blackout (ASR-3, ASR-4). Which language does this best?

## Previous version (remakes)

v1 used Python with pyserial: acknowledged block writes of changed runs, a slow full refresh of
the low channels, blackout release on connect, a read-back endpoint. The aim path ran a ray/box
test against safety zones. Throughput, jitter and failure behaviour were never measured; whether
every output path passed the safety check was never proven.

## Options

| Option | Package(s) verified (version, last release, licence, checked on) | Notes |
|---|---|---|
| .NET | System.IO.Ports 11.0.0; System.Management 11.0.0; TimeProvider.Testing 11.0.1; ArchUnitNET 0.13.5 (2026-11-26) | Serial from Microsoft on Windows and Unix |
| Rust | serialport 4.10.1 (MPL-2.0) or serial2 0.2.39; spin_sleep 1.3.3; tokio 1.54.1; mock_instant 0.6.1 (2026-11-26) | No garbage collector; crate graph as boundary |
| Dart | flutter_libserialport 0.6.0 over libserialport (LGPL-3.0); fake_async 1.3.3 (2026-11-26) | Only while Flutter stays in the comparison |

## Criteria and weights (agreed with the owner before scoring)

Agreed 2026-11-26.

| Concern | Criterion | Weight | Source (QA-…) |
|---|---|---|---|
| C-1 | Serial library maturity on Windows and Linux | 3 | QA-4 |
| C-1 | Control over timing and acknowledgements | 3 | QA-1, ASR-1 |
| C-1 | Port discovery and reconnect (FTDI VID/PID, hot-plug) | 2 | QA-4 |
| C-1 | Testable without hardware (simulator, fake port) | 2 | ASR-10 |
| C-2 | Timer precision and jitter on Windows and Linux | 3 | QA-1, ASR-1 |
| C-2 | Predictable latency (no long garbage-collection pauses) | 3 | QA-1, ASR-2 |
| C-2 | Concurrency model for UI, engine, audio and network | 2 | QA-8 |
| C-2 | Testability of timing (fake clock) | 2 | ASR-10 |
| C-3 | Compile-time guarantees that no other path reaches the hardware | 3 | QA-2, ASR-3 |
| C-3 | Failure handling (crash of a show source, lost port, watchdog) | 3 | QA-2, ASR-4 |
| C-3 | Safety tests in the gates (`scripts/check safety`) | 2 | constitution VII |
| C-3 | Architecture test support (`ARCH_CMD`) | 2 | ASR-3 |

## Scores

Scores 1–5; each concern out of 50.

| Option | C-1 | C-2 | C-3 | Reason |
|---|---|---|---|---|
| .NET | **41** (5, 4, 3, 4) | **38** (3, 3, 5, 5) | **42** (4, 4, 4, 5) | System.IO.Ports from Microsoft with Unix sources; `GetPortNames` gives names only, VID/PID needs WMI on Windows and sysfs on Linux (ASSUMPTION); default Windows timer resolution and GC pauses need care (ASSUMPTION); FakeTimeProvider; assembly boundaries and ArchUnitNET |
| Rust | **43** (4, 5, 4, 4) | **43** (4, 5, 4, 4) | **45** (5, 4, 5, 4) | serialport-rs asks for maintainers, especially for Windows (README), serial2 as alternative; blocking I/O with full control; no GC; spin_sleep for accurate waits; a crate that is the only owner of the port, with a checked-frame type, makes other paths a compile error; safety tests verified in the profile |
| Dart | **29** (2, 3, 4, 3) | **29** (2, 3, 3, 4) | **28** (2, 4, 3, 2) | Serial through a C library (LGPL-3.0, last release 2022) over FFI; event-loop timers (ASSUMPTION: millisecond precision); isolates isolate crashes; library privacy is the only boundary; no architecture test tool found |

Sub-scores in brackets follow the criteria order of the table above.

## Toolability (dev-standard gates)

Rust: `SAFETY_TEST_CMD` filters tests named `safety` (verified 2026-11-24). .NET: `Category=Safety`
(profile unverified). Dart: `--tags safety` (profile unverified). `ARCH_CMD`: ArchUnitNET for .NET;
for Rust the workspace crate graph does most of the work, with `cargo deny` bans for dependencies
(ASSUMPTION: no dedicated architecture test needed).

## Evidence

- The DMX4ALL interface "generates the DMX512 signal itself and holds the last value of every slot". After a crash of the process that holds the port, the lights stay in their last state. — [v1 `docs/DMX4ALL_PROTOCOL.md`, read 2026-11-26] (high)
- Blackout is a device command (`B1`); any process that opens the port can send it. — [same] (high)
- At 38400 baud 8N1 about 3,840 bytes per second pass; at 44 Hz that is about 87 bytes per update, including the 4-byte block header and the acknowledgement. — ASSUMPTION: calculated, not measured (medium)
- System.IO.Ports ships Unix sources (`SerialPort.Unix.cs`, `SerialStream.Unix.cs`) next to the Windows ones. — [github.com/dotnet/runtime, read 2026-11-26] (high)
- serialport-rs provides "blocking I/O interface and port enumeration on POSIX and Windows"; Linux enumeration relies on libudev; the README asks for maintainers, especially for Windows. — [serialport-rs README, read 2026-11-26] (high)
- spin_sleep exists because "`thread::sleep` isn't always very accurate, and this accuracy varies on platform and state". — [spin-sleep README, read 2026-11-26] (high)
- The default Windows timer resolution is about 15.6 ms unless raised, which is coarser than a 22.7 ms update period needs. — ASSUMPTION (medium)
- NetArchTest.Rules had its last release in 2021; ArchUnitNET is active. — [verify-package nuget, 2026-11-26] (high)

## Evidence against the leading option

Rust leads all three concerns, but by 2 to 7 points, inside the error of judged scores. Its serial
library asks for Windows maintainers, and the venue runs Windows. The compile-time guarantee for
C-3 holds only if the architecture keeps the port inside one crate; .NET reaches most of it with
assembly boundaries and ArchUnitNET. Rust's UI score (63, egui) is lower than Avalonia's (69), so
an all-Rust stack trades UI comfort for engine strength; a mix (Rust engine, other UI) costs the
second language the owner wants to avoid.

## Spike needed?

Yes: the existing dongle spike (`TODO.md`) measures throughput, acknowledgement latency and
achievable update rate on the real Mini-USB. It answers ASR-1 and ASR-2 for every language,
because the limit sits in the interface. A second question for the stack layer, not the language:
a supervisor that blacks out the interface when the main process dies.

## Recommendation (confidence: medium)

**Rust** is the strongest choice for the output, the engine and safety enforcement (43, 43, 45).
**.NET** is adequate for all three (41, 38, 42) with more care for timer resolution and GC.
**Dart** is weak for all three. Whatever the language, ASR-4 needs a supervisor outside the main
process, because the interface holds its last values: the safety design adds a watchdog that
reopens the port and sends blackout when the main process stops.

## Open questions

- Achievable update rate and acknowledgement latency of the Mini-USB (dongle spike).
- Owner decision on ASR-4: a supervisor process that sends blackout, and its reaction time (QA-2).
- Does the NanoDMX hold last values the same way, and does it offer a timeout or watchdog?
- serialport-rs on Windows versus serial2: which one in the spike.
