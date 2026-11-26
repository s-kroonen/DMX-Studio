# Idea Research: DMX Studio, a rebuild of the DMX Controller

- **Slug**: dmx-studio
- **Created**: 2026-11-26
- **Evidence confidence (overall)**: medium — hardware and library facts are cited; demand rests on one user; market facts on commercial tools are assumptions.

## Users & Demand

- The only confirmed user is the owner, who operates the lights at events and maintains the software alone. — [source: intake.md, owner statements] (confidence: high)
- The owner built and used the earlier controller: 48 commits between 2026-09-17 and 2026-09-22, about 6,400 lines of Python application code, 4,000 lines of Python tests (305 test functions), 6,500 lines of own JavaScript with no JavaScript tests. — [source: github.com/s-kroonen/DMX-Controller, git history and line count] (confidence: high)
- Hardware use is partly verified: the DMX4ALL handshake, block writes and blackout release are checked on the real dongle; "all three fixtures show red", "Beamz stays out of auto mode" and smooth 16-bit pan/tilt are unchecked. — [source: DMX-Controller docs/DMX4ALL_PROTOCOL.md, "Hardware validation checklist"] (confidence: high)
- The README states bundled fixture profiles are "unverified against real fixtures except where noted". — [source: DMX-Controller README, "Known limitations"] (confidence: high)
- Stated wants (vision, intake) outnumber observed use: sound-to-light, calibration and room aiming are described in detail, but no record shows how often or at which events the earlier controller ran. — [ASSUMPTION] (confidence: medium)
- Other users (helpers on phones, developers, lighting veterans) are anticipated, not observed. — [ASSUMPTION] (confidence: low)

## Prior Art

- **The earlier DMX Controller** (internal): proved 3D aiming with an IK solver, safety zones, sound-to-light, calibration from marks, a phone layout and edit/show modes in about six days of work. The JavaScript half has no tests; the owner names maintainability as the reason for the rebuild. — [source: DMX-Controller README and repository] (confidence: high)
- **QLC+**: open-source (Apache-2.0), cross-platform (Linux, Windows 10+, macOS 10.12+, Raspberry Pi), Qt-based, active (last push 2026-11-24), 1,593 stars. Supports DMX4ALL USB-DMX STAGE-PROFI MK2 and NanoDMX through its `dmxusb` plugin, using the same `C?` handshake as the earlier controller. — [source: github.com/mcallegari/qlcplus README, resources/docs/html_en_EN/dmxusbplugin.html, plugins/dmxusb/src/stageprofi.cpp] (confidence: high)
- **QLC+ fixture definitions (`.qxf`)**: a library of fixture files ships in the QLC+ repository (`resources/fixtures`, 146 entries at the top level). The earlier controller already imports `.qxf`. — [source: github.com/mcallegari/qlcplus; DMX-Controller README] (confidence: high)
- **GDTF / MVR**: fixture description (GDTF, DIN SPEC 15800:2022-02) and venue/rig exchange (MVR, DIN SPEC 15801:2023-12) formats, manufacturer-supported, specification maintained in the open (repository has no licence file; default meshes free to use). — [source: github.com/mvrdevelopment/spec README] (confidence: high)
- **Open Lighting Architecture (OLA)**: open-source lighting framework with RDM support across many plugins; the repository mentions DMX4ALL only in the RDM manufacturer names list, not as a device plugin. — [source: github.com/OpenLightingProject/ola, code search] (confidence: medium)
- **FreeStyler, SoundSwitch, Lightkey, grandMA onPC, ChamSys MagicQ, Obsidian ONYX**: the reference tools the owner named. Not researched in this pass; their concepts are input for the familiarity levels. — [ASSUMPTION: owner's familiarity] (confidence: low)

## Market & Context

- Free alternatives exist for the core of the idea (QLC+, FreeStyler); none known to the owner combines 3D room aiming with beam safety zones and sound-to-light. — [ASSUMPTION; not verified against each product] (confidence: low)
- Cost of doing nothing: the earlier controller keeps working on the current dongle, but grows harder to change (untested JavaScript, Python and JavaScript split, Windows-specific audio capture). — [source: DMX-Controller repository; owner statement] (confidence: medium)
- The project is a personal tool first; no commercial goal is stated. — [source: intake.md] (confidence: high)

## Data & Constraints

- **DMX4ALL Mini-USB-DMX-Interface (Art. Nr. 99-0412) is discontinued** and listed under products that "can no longer be supported"; the product page carries no specifications. — [source: https://www.dmx4all.de/produkte/mini-usb-dmx-interface/, https://www.dmx4all.de/produktportfolio/nicht-mehr-unterstuetzte-produkte/] (confidence: high)
- Current DMX4ALL USB interfaces: NanoDMX USB Interface (512 channels, "USB-VCP mit DMX4ALL commands", fast mode 44 frames/s, slow mode 33 frames/s) and DMX STAGE-PROFI 1.1 RDM (512 channels out, 5–44 fps, USB-VCP, RDM over Art-Net only). — [source: https://www.dmx4all.de/produkte/nanodmx-usb-interface/, https://www.dmx4all.de/produkte/dmx-stage-profi-1-1-rdm/] (confidence: high)
- **RDM over USB on DMX4ALL hardware: not found.** The Mini-USB page says nothing on RDM; the NanoDMX page does not mention RDM; the STAGE-PROFI offers RDM over Art-Net. — [source: same pages] (confidence: medium; the Mini-USB manual PDF was not read)
- DMX4ALL wire protocol on the owner's dongle: FTDI VID 0403 / PID C850, 38400 baud 8N1, acknowledged block writes (`FF <start> <count> <data>`, up to 255 values), the device holds the last value of each slot, `N?` reports the number of output slots (example reply 224). — [source: DMX-Controller docs/DMX4ALL_PROTOCOL.md] (confidence: high)
- Throughput budget: at 38400 baud 8N1, about 3,840 bytes/s; a full 512-channel refresh in 255-value blocks takes about 0.14 s, so only changed channels can be sent at show rates. — [ASSUMPTION: derived from the baud rate; not measured] (confidence: medium)
- **Android as USB host**: `usb-serial-for-android` (MIT, active, 5,732 stars) drives FTDI FT232R/FT232H/FT2232H/FT4232H/FT230X/FT231X/FT234XD over USB host mode (OTG), "working reliably since Android 4.2". Its default table lists FTDI PIDs 6001, 6010, 6011, 6014, 6015, not C850; a custom probe table adds other PIDs. — [source: github.com/mik3y/usb-serial-for-android README and UsbId.java] (confidence: medium; not tried with the DMX4ALL dongle)
- **Ableton Link licence**: dual-licensed, GPLv2-or-later or a proprietary licence on request from Ableton; C++ library; Windows (MSVC 17 2022), Linux (Clang 13 or GCC 10) and macOS supported. The dev-standard licence gate fails on a GPL production dependency without an exception. — [source: github.com/Ableton/link LICENSE.md and README; dev-standard README "Verified"] (confidence: high)
- Safety: the project is safety-critical (constitution VII) and exposes a control surface (constitution VIII); both apply to every output path, including scripts, API and MCP. — [source: CONSTITUTION.md] (confidence: high)
- Platforms: Windows (venue) and Linux (development) hosts are required; Android is optional; iOS is out. — [source: owner answers in intake.md] (confidence: high)

## Evidence Against the Idea

- **Hardware end of life**: the only verified interface is discontinued and unsupported by its manufacturer. A rebuild that depends on it inherits that risk; the NanoDMX speaks the same command set and is the closest current replacement (same vendor command protocol, per QLC+ and the NanoDMX page), but nothing confirms the Mini-USB code path works unchanged on it. — [source: dmx4all.de pages above]
- **RDM is not reachable on the current hardware**: no DMX4ALL USB interface lists RDM over USB; RDM in the scope needs other hardware or Art-Net, which the vision rules out. — [source: dmx4all.de pages above]
- **Scope size versus one maintainer**: the vision lists about 40 capabilities across five tiers; the earlier controller covered about half of them in a burst of work and left the frontend untested. A rebuild risks a long period without a working controller. — [source: docs/product/vision.md; DMX-Controller repository] (confidence: medium)
- **Mature free alternative**: QLC+ is cross-platform, open source, active, supports DMX4ALL interfaces and a fixture library. Contributing features (3D aiming, safety zones) to it, or using it next to a smaller tool, is an option the assessment has to weigh. — [source: github.com/mcallegari/qlcplus]
- **Licence friction**: Ableton Link under GPLv2+ conflicts with the licence gate unless the project accepts GPL or obtains the proprietary licence. — [source: github.com/Ableton/link]
- **Low serial bandwidth**: 38400 baud limits full refreshes and fast effects across many channels. — [ASSUMPTION: derived, not measured]

## Gaps & Open Questions

- [NEEDS CLARIFICATION: is a replacement interface (for example the NanoDMX) acceptable, and should the rebuild be verified on it]
- [NEEDS CLARIFICATION: RDM: drop it, defer it to other hardware, or accept Art-Net for RDM only]
- [NEEDS CLARIFICATION: does the Mini-USB manual state RDM support or output refresh rate (PDF not read)]
- [NEEDS CLARIFICATION: measured latency and refresh rate of the dongle at show load]
- [NEEDS CLARIFICATION: Ableton Link: accept GPL for the project, request the proprietary licence, or leave Link out]
- [NEEDS CLARIFICATION: why not build on or contribute to QLC+]
- [NEEDS CLARIFICATION: how often and where the earlier controller ran at real events, and what failed there]
- [NEEDS CLARIFICATION: concepts of the reference tools (grandMA, ChamSys, ONYX, SoundSwitch, Lightkey, FreeStyler) per familiarity level]
- [NEEDS CLARIFICATION: Android USB host with PID C850 on real phones]

## Sources

- https://github.com/s-kroonen/DMX-Controller (host: github.com, policy: allowlisted; read as a git clone)
- https://github.com/mcallegari/qlcplus (host: github.com, policy: allowlisted; GitHub API)
- https://github.com/mvrdevelopment/spec (host: github.com, policy: allowlisted; GitHub API)
- https://github.com/OpenLightingProject/ola (host: github.com, policy: allowlisted; GitHub API and code search)
- https://github.com/mik3y/usb-serial-for-android (host: github.com, policy: allowlisted; GitHub API)
- https://github.com/Ableton/link (host: github.com, policy: allowlisted; GitHub API)
- https://www.dmx4all.de/en/ (host: www.dmx4all.de, policy: confirmed-by-user)
- https://www.dmx4all.de/produktportfolio/usb-dmx-interfaces/ (host: www.dmx4all.de, policy: confirmed-by-user)
- https://www.dmx4all.de/produktportfolio/nicht-mehr-unterstuetzte-produkte/ (host: www.dmx4all.de, policy: confirmed-by-user)
- https://www.dmx4all.de/produkte/mini-usb-dmx-interface/ (host: www.dmx4all.de, policy: confirmed-by-user)
- https://www.dmx4all.de/produkte/dmx-stage-profi-1-1-rdm/ (host: www.dmx4all.de, policy: confirmed-by-user)
- https://www.dmx4all.de/produkte/nanodmx-usb-interface/ (host: www.dmx4all.de, policy: confirmed-by-user)
