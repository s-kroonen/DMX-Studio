# Quality attributes and stack concerns

The requirements that decide the stack, each measurable. Filled by `/stack-concerns`; the stack
decision cites them by ID. Sources: [vision](vision.md), the
[assessment](../../.specify/assessments/dmx-studio/decision.md), `CONSTITUTION.md`, the earlier
[DMX Controller](https://github.com/s-kroonen/DMX-Controller) (v1), and owner answers of
2026-11-26.

## Quality attributes

| ID | Attribute | Target (measurable) | Source | Status |
|---|---|---|---|---|
| QA-1 | Timing / real-time | DMX output at up to 44 updates per second, the rate a setting; the output lowers its rate when the interface cannot keep up; change on screen to light within 50 ms at the configured rate | owner 2026-11-26; vision "Success" | 50 ms proposed, not measured |
| QA-2 | Safety | Every value sent to hardware passes the limit and safety-zone check in one place on the output side; on an error, a lost connection or a stopped process the output goes to blackout within 1 s (ASR-11); 0 beams into a zone in safety tests | constitution VII; problem.md; owner 2026-11-26 | |
| QA-3 | Platforms | Host: Windows [NEEDS CLARIFICATION: 10 and 11?] at the venue, Linux (Fedora) for development; browser client on current Chromium and Firefox; Android app later; no iOS | owner answers in intake.md | Windows versions open |
| QA-4 | Hardware interfaces | DMX4ALL command protocol over USB virtual COM port (FTDI, 38400 baud on the Mini-USB): acknowledged block writes, blackout release on connect, read-back; Mini-USB first, NanoDMX as replacement; one process holds the port | research.md; v1 `docs/DMX4ALL_PROTOCOL.md` | measured partly in v1 |
| QA-5 | Offline / network | Runs a full event without internet; every asset ships with the app; the network is the local network only | vision | |
| QA-6 | Packaging / install | Windows: an installer and a portable build that runs without admin rights; Linux: a package for development use; Android: an installable app later | owner 2026-11-26 | Linux format open |
| QA-7 | Security | Control surface listens on localhost by default; network access is an explicit host setting; clients pair explicitly; remote clients control and calibrate, never edit setup; the MCP server listens on the host only; see `docs/security/control-surface.md` | constitution VIII; vision | |
| QA-8 | Performance / scale | One full universe (512 channels) in use at the full output rate; the data model and output engine leave room for more universes without redesign; UI frame rate of the 3D view [NEEDS CLARIFICATION: e.g. ≥ 30 fps with 50 fixtures] | owner 2026-11-26 | UI target open |
| QA-9 | Maintainability | One maintainer; languages the owner maintains: Rust, Dart/Flutter, C#/.NET; one repository for host, browser client and Android; format, lint, type, test and coverage gates of dev-standard on every part; code readable by AI agents through the same gates | owner 2026-11-26; vision "Constraints" | |

## Architecturally significant requirements (ASRs)

The quality attributes (or parts of them) that would be expensive to change later: they decide the
architecture and the stack. Filled by `/stack-concerns`, checked against what was measured after the
first working build (`docs/process/stack-layer.md`, step 14), and cited by decision records.

| ID | Requirement (a testable statement) | Quality attribute | Source | Measured (date, how) | Gate |
|---|---|---|---|---|---|
| ASR-1 | The output engine sends a DMX update at the configured rate (default 44 Hz) with jitter ≤ [NEEDS CLARIFICATION] ms, sends only changed channels, and lowers the rate automatically when the interface's acknowledgements fall behind | QA-1, QA-4 | owner 2026-11-26 | not yet | `PERF_CMD` |
| ASR-2 | A control change on the host reaches the interface within 50 ms at the configured rate | QA-1 | vision "Success" (proposed) | not yet | `PERF_CMD` |
| ASR-3 | No value reaches the interface without passing the safety check in one module on the output side; cues, effects, sound, scripts, API and MCP have no other path to the hardware | QA-2 | constitution VII | not yet | `scripts/check safety`, `ARCH_CMD` |
| ASR-4 | On an output error, a lost interface, a crash of a show source or a stop of the process, the output goes to blackout within the QA-2 reaction time | QA-2 | constitution VII | not yet | `scripts/check safety` |
| ASR-5 | The host builds and runs from one repository on Windows and Linux; the browser client is served by the host; an Android app can be added in the same repository without a second copy of the show logic | QA-3, QA-9 | owner answers in intake.md | not yet | `BUILD_CMD`, `E2E_CMD` |
| ASR-6 | The control surface is off the network by default, requires pairing and authentication for every state-changing call, and checks origin and host on browser requests | QA-7 | constitution VIII | not yet | `docs/security/control-surface.md`, `DAST_CMD` |
| ASR-7 | 512 channels change every update at the full output rate within [NEEDS CLARIFICATION: CPU budget] on the venue laptop; channel addressing carries a universe number from the start | QA-8 | owner 2026-11-26 | not yet | `PERF_CMD` |
| ASR-8 | The app starts and runs a show with no network connection at all | QA-5 | vision | not yet | `E2E_CMD` |
| ASR-9 | A Windows installer and a portable build come from the same release; the portable build runs without admin rights | QA-6 | owner 2026-11-26 | not yet | `BUILD_CMD` |
| ASR-10 | Every part of the code (host, browser client, later Android) runs the format, lint, type, test and coverage gates in `scripts/check` | QA-9 | constitution III; vision | not yet | `scripts/check all` |
| ASR-11 | A supervisor process, separate from the main process, watches it; when the main process stops or stops answering, the supervisor opens the interface and sends blackout within the QA-2 reaction time. The interface holds its last values, so the main process alone cannot meet ASR-4 | QA-2 | [stack-output-engine-safety.md](../research/stack-output-engine-safety.md); owner 2026-11-26 | not yet | `scripts/check safety` |

## Concerns

| ID | Concern | Drivers | Reversibility | Needs a spike? | Research note | Spike |
|---|---|---|---|---|---|---|
| C-1 | Output to the DMX4ALL interface over USB serial: throughput, latency, rate control, port ownership on Windows and Linux | QA-1, QA-4, ASR-1, ASR-2 | two-way (behind an output boundary) | yes: measure the Mini-USB | [stack-output-engine-safety.md](../research/stack-output-engine-safety.md) | `spike/DMX-?-dongle-throughput` |
| C-2 | Show engine: the update loop, merging live control, cues, effects and sound into one output frame, timing and jitter | QA-1, QA-8, ASR-1, ASR-7 | one-way (core language) | no | [stack-output-engine-safety.md](../research/stack-output-engine-safety.md) | |
| C-3 | Safety enforcement: one check on the output side, safe state on failure, testability of safety rules | QA-2, ASR-3, ASR-4 | one-way (architecture) | no | [stack-output-engine-safety.md](../research/stack-output-engine-safety.md) | |
| C-4 | Desktop UI toolkit for the host on Windows and Linux: many control windows, keyboard shortcuts, UI levels | QA-3, QA-9, ASR-5 | one-way | no | [stack-ui.md](../research/stack-ui.md) | |
| C-5 | 3D room view and aiming in that toolkit: room shape, fixtures, beams, picking a point | QA-3, QA-8 | one-way | yes: 3D view in the leading toolkit | [stack-ui.md](../research/stack-ui.md) | `spike/DMX-?-3d-view` |
| C-6 | Remote clients: browser client served by the host, later the Android app, sharing code with the host in one repository | QA-3, QA-9, ASR-5 | one-way | maybe: after C-4 | [stack-ui.md](../research/stack-ui.md) | |
| C-7 | Control surface: local API with an OpenAPI description, live updates to clients, pairing and authentication, origin checks, a host-only MCP server | QA-7, ASR-6 | two-way (protocol), one-way (language libraries) | no | [stack-api-storage-libraries.md](../research/stack-api-storage-libraries.md) | |
| C-8 | Audio capture and analysis: input devices and loopback on Windows and Linux, band levels, beat and BPM detection, latency | QA-1, QA-3 | two-way (behind an audio boundary) | yes: loopback on both systems | [stack-audio.md](../research/stack-audio.md) | `spike/DMX-?-audio-loopback` |
| C-9 | Venue storage: a human-readable, versioned file for room, fixtures, profiles, groups and shows; export and import; migrations | QA-9 | two-way (with migrations) | no | [stack-api-storage-libraries.md](../research/stack-api-storage-libraries.md) | |
| C-10 | Maths: aiming (pan/tilt from a 3D point) and the calibration solver (non-linear least squares) | QA-2, QA-8 | two-way | no | [stack-api-storage-libraries.md](../research/stack-api-storage-libraries.md) | |
| C-11 | Packaging and release: Windows installer and portable build, Linux package, Android app later, from one release | QA-6, ASR-9 | two-way | no | [stack-api-storage-libraries.md](../research/stack-api-storage-libraries.md) | |
| C-12 | Toolability: dev-standard profiles and gates for the candidate languages (format, lint, types, tests, coverage, SAST rules, quality canary) | QA-9, ASR-10 | one-way | no | [stack-toolability.md](../research/stack-toolability.md) | |
| C-13 | Later slices that depend on libraries: embedded scripting, MIDI, OSC, Ableton Link (licence), Android USB host | QA-9 | two-way if kept behind boundaries | no (checked per slice) | [stack-api-storage-libraries.md](../research/stack-api-storage-libraries.md) | |

## Previous version per concern

What v1 (the earlier DMX Controller) did and what the project learned. Source: the v1 repository
and its README, read 2026-11-26 (`research.md` of the assessment).

| Concern | v1 | Learned |
|---|---|---|
| C-1 | Python and pyserial; acknowledged block writes of changed runs, slow full refresh of low channels, blackout release on connect, read-back endpoint | The fast single-channel writes of firmware V3.36 corrupt the buffer; the Martin pinout keeps fixtures dark; only one process can hold the port; throughput at show load was never measured |
| C-2 | Show engine and animation engine in Python, driven by the API | Live control, patterns, paths and sound worked together; the shared dimmer/strobe channel needed per-role value ranges |
| C-3 | Ray/box test of every computed aim before output; positions clamped to the room on save | Safety lived in the aiming path; whether every output path passed it was not proven |
| C-4 | Vanilla JavaScript UI in a browser, floating windows, packaged with pywebview and PyInstaller on Windows | Worked, but 6,500 lines without tests; the split between Python and JavaScript slowed changes |
| C-5 | Three.js, vendored for offline use; drag gizmo, picking surfaces to aim | The 3D view and picking were central to the product; mobile reused the desktop's fixture models |
| C-6 | A separate phone layout in the same frontend, chosen by screen size | One backend for desktop and phone worked; browser microphone and gyroscope aiming were deferred |
| C-7 | FastAPI REST and WebSocket, edit/show mode enforced on the server (HTTP 409); LAN binding by a start flag | Mode enforcement on the server worked; no authentication or pairing existed |
| C-8 | WASAPI loopback through PyAudioWPatch on Windows only; bands, beat, BPM, tap tempo | The effect set worked at events; capture was Windows-only |
| C-9 | JSON files per concern in a data folder; named saved rooms; bundled profiles shadowed by user copies | Snapshots and export/import were useful; shadowed bundled profiles needed a rebuild rule |
| C-10 | Pure, unit-tested IK function; least-squares solver for mounting from marks | Fine channels and flip-over reach mattered; the solver needed 3+ marks, 5+ for position |
| C-11 | PyInstaller one-file exe for Windows; run script for development | Packaged data needed a per-user data folder; a second start must not open the port twice |
| C-12 | pytest for Python; nothing for JavaScript | The untested half is the main reason for the rebuild |
| C-13 | None of these existed | — |

A full retrospective (`docs/research/retrospective-template.md`) follows the first working build
(DMX-6).
