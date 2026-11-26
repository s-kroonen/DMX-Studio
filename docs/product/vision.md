# Product vision — DMX Studio

> DMX lighting controller with 3D room mapping, safety zones and a show and animation engine.

DMX Studio is a full rebuild of the earlier
[DMX Controller](https://github.com/s-kroonen/DMX-Controller). That project is input for this
page as a list of features that proved useful and as a source of hardware facts. Its stack, code
structure, file formats and API are not carried over: the stack decision (DMX-4) starts from the
requirements below, not from the code.

The [idea assessment](../../.specify/assessments/dmx-studio/decision.md) (DMX-3) holds the
evidence, the problem definition, the options and the verdict: **go**, rebuilding in vertical
slices, with the earlier controller in use until the first release replaces it. This page is the
one-page summary.

## Problem

Affordable DMX software addresses fixtures in raw channel values. Pointing a moving head at a spot
in a real room means nudging pan and tilt until the beam lands, and redoing it for every head and
every venue. Nothing stops a beam from sweeping across a stage edge, a doorway or the audience.
Professional consoles solve show building with presets, cues and effects, but cost money and take
training; music-driven tools are quick but shallow. DMX Studio describes the room in 3D and aims
fixtures at points in it. It blocks beams from no-go zones before any value reaches the hardware.
Live control, cues, effects and sound-to-light run from one place, at the depth the operator
chooses, from a laptop or a phone. Full statement:
[problem definition](../../.specify/assessments/dmx-studio/problem.md).

## Users

| User | Goal | Context (device, place, skill) |
|---|---|---|
| Operator | Run the lights during an event: looks, cues, effects, aiming, sound-to-light | Laptop next to the USB DMX interface; often no internet; hobbyist to semi-professional |
| Venue setup (often the same person) | Patch fixtures, model the room, place and calibrate fixtures, set safety zones | Laptop and phone on site before the event |
| Lighting veteran | Find familiar concepts (programmer, presets, cues, playbacks) and work at full speed | Uses the advanced level, keyboard shortcuts, MIDI controllers |
| Helper on a phone | Change colours, aim a head, run a show from the floor or the stage | Android app, or a browser on any phone, on the local network |
| Developer | Write scripts and add-ons: advanced animations, automation, integrations | Documented API, embedded scripting |
| AI agent | Help with setup and debugging: patching, room model, diagnosing output | Local MCP server on the host machine |

## Familiarity

DMX Studio borrows concepts, not looks, from existing tools, so an experienced operator recognises
them:

| Level | Feels like | Typical concepts |
|---|---|---|
| Basic | SoundSwitch, Lightkey | Music-driven looks, visual setup, few controls |
| Standard | FreeStyler, QLC+ | Floating control windows, fixture library, scenes, virtual console |
| Advanced | grandMA, ChamSys MagicQ, Obsidian ONYX | Programmer, presets, cues and cue lists, playbacks, command line |

The operator picks the level; all levels work on the same show data.

## Scope

Origin: **E** = proven in the earlier controller, **+** = added for DMX Studio. The tiers are a
proposal; the assessment (DMX-3) confirms them and cuts them into releases. Nothing here fixes a
technology, a file format or an architecture.

### Core (first version)

| Area | Capability | Origin |
|---|---|---|
| DMX output | One universe through an interface that speaks the DMX4ALL command protocol (the Mini-USB first, the NanoDMX as replacement); a simulated output for work without hardware; switch without a restart; blackout | E |
| Fixtures | Fixture profiles (channel map, pan/tilt range, fine channels, named ranges, per-role value ranges); bundled profiles for the fixtures in use; patching | E |
| Live control | Colour, dimmer, strobe/shutter, pan/tilt and custom channels for a fixture, a selection or a group, all at the same time | E |
| Groups | Named groups, including a built-in group of all lights | E |
| Room and aiming | Room floor shape and height, fixture position and mounting, aim heads at a point in the room | E |
| Safety | No-go zones; any aim or movement whose beam enters a zone is blocked before output, with the reason shown | E |
| Modes | Edit mode for setup, show mode for running; each mode refuses what it does not allow | E |
| Sound-to-light (basic) | Audio input and loopback capture, level and band meters, beat detection, BPM and tap tempo; VU dimmer, beat flash, colour chase, colour organ, beat strobe, beat movement; each effect targets its own lights or groups; react with colour, motion or both | E |
| Venue config | Save and load the whole venue (room, fixtures, profiles, groups, shows, sound settings); export and import as a file | E |
| Remote clients | Phones and browsers on the local network control the show; they can't edit the setup; calibration is allowed; a host setting turns remote access off | E, + |

### Next

| Area | Capability | Origin |
|---|---|---|
| Show building | Scenes and presets (store a look, recall it); cue lists with fades; an effects engine for colour, dimmer and movement | + |
| Animations | Time-based pan/tilt patterns and keyframed paths through room points, with a time offset per fixture | E |
| Multi-zone fixtures | Light bars: zones with their own colour, dimmer and strobe; zone chases | E |
| Calibration | Aim all heads at one point to compare, offset trims, solving a head's mounting and position from recorded marks | E |
| Fixture library | A library of fixture types; import from other programs' files (for example QLC+ `.qxf`); a fixture editor | E, + |
| Working speed | Undo and redo; common keyboard shortcuts; a shortcut editor | + |
| UI levels | Basic, standard and advanced levels (see Familiarity) | + |
| Mobile app | A native Android app as the main phone client, with the browser client as fallback | + |
| Mobile audio | Sound-to-light from a phone or browser microphone | + |
| Developer API | A documented API with an OpenAPI page; scripts outside the app use it | + |
| Scripting | Scripts inside the app for timing-critical work such as advanced animations | + |
| Agent support | A local MCP server so an AI agent can help with setup and debugging | + |
| Tempo sync | Tempo and beat sync with other apps through Ableton Link; its GPLv2+ licence is settled before building | + |
| Room objects | Reference objects (walls, people, boxes, platforms) and named saved rooms | E |

### Nice to have

| Capability | Origin |
|---|---|
| MIDI controllers (faders, pads) with MIDI learn | + |
| OSC input for show and VJ software and tablet control surfaces | + |
| A timeline that places cues and effects against a music track (scope to be discussed) | + |
| Gyroscope aiming from the phone | + |
| The phone as a calibration and position-measuring tool | + |
| The phone as the USB host for the DMX interface, so a show runs from the phone alone | + |
| Preparing a whole show from music in advance, with a simulation of movement and light | + |
| Advanced music analysis: drops, build-ups, genre and energy, with lights and movement following them | + |
| Scanning the room and the fixtures with the phone (camera and gyroscope) to calibrate each fixture | + |
| A packaged desktop app that also serves phones on the local network | E |

### Future

- Lasers, with laser-specific safety rules. No laser hardware is available yet; safety zones
  already cover moving heads.
- AR room setup.
- More than one universe. The first releases drive one; the design leaves room for more
  (`quality-attributes.md`, QA-8).
- RDM: discover fixtures and read or set their address over the DMX line. No DMX4ALL USB
  interface offers RDM; it waits for RDM-capable hardware.

### Out

A later decision record can move an item back in.

- Network DMX (Art-Net, sACN) and interfaces outside the DMX4ALL command protocol.
- Network smart lights such as Home Assistant RGB bulbs: too slow for show control.
- iOS clients.
- User accounts and roles. The host setting and the control-only rule above limit remote access.
- Sloped walls and ceilings.

## Success

Targets carried from the [problem definition](../../.specify/assessments/dmx-studio/problem.md);
the aim, setup and latency numbers are proposals until measured:

- Zero beams into a safety zone: every safety test passes, in edit mode, show mode, cues, effects,
  animations, sound-to-light and scripts.
- A head aimed at a marked point lands within 10 cm at 5 m after calibration.
- A venue with 6 fixtures (room shape, patch, positions, one safety zone) set up in under
  30 minutes.
- Change on screen to light output within 50 ms on the DMX4ALL interface.
- Runs a full event without internet access.
- A guest phone controls colours and aiming through a browser with nothing installed.
- A developer writes a working script against the API from the OpenAPI page alone.

## Constraints and risks

- **Safety.** Moving heads, and later lasers, can hit eyes. The project is safety-critical
  (`CONSTITUTION.md`, principle VII): safety zones are checked on every output path, including
  cues, effects, scripts and API calls, and safety tests are mandatory.
- **Control surface.** Phones, the API, the MCP server and scripts all reach the lights
  (`CONSTITUTION.md`, principle VIII; [control surface checklist](../security/control-surface.md)).
  Network access is off unless the host turns it on; the MCP server listens on the host only.
- **Maintainability.** Native application stack: the split into a Python server and a hand-written
  JavaScript frontend is not repeated. The stack decision (DMX-4) picks the languages; the
  dev-standard gates (format, lint, types, tests, coverage) must work on every part.
- **Platforms.** Host app on Windows (venue laptop) and Linux (development). Android is the
  mobile target; if a shared codebase for host and Android limits the stack too much, the browser
  client or another route serves the phone. The browser client comes first, the Android app
  follows. Audio loopback capture is platform-specific.
- **One repository.** The host app, the browser client and the Android app live in this
  repository and share the gates.
- **Hardware.** The DMX4ALL Mini-USB interface (FTDI serial, 38400 baud) is the only verified
  output, and DMX4ALL has discontinued it. DMX Studio targets the DMX4ALL command protocol, which
  the current NanoDMX also speaks; differences of the Mini-USB are handled where found. Firmware
  V3.36 ignores the fast single-channel writes, can start in blackout, and has a pinout setting
  (International or Martin) that keeps fixtures dark when wrong. Only one process can hold the
  port. The phone as USB host is unverified (the Android FTDI driver needs the dongle's PID C850
  added).
- **Licences.** Ableton Link is GPLv2+ or proprietary on request; the licence gate
  (`scripts/check licenses`) fails on GPL dependencies, so the licence route is decided before
  Link is built.
- **Offline.** No internet at the venue: every asset ships with the app.
- **Team.** One maintainer, hobby budget. Features land in small steps through the spec-driven
  workflow in `AGENTS.md`.

## Open questions

The [decision](../../.specify/assessments/dmx-studio/decision.md) carries them forward. For the
stack decision (DMX-4): one stack for a Windows and Linux host, the browser client and later
Android with all gates; latency of the DMX4ALL interface at show load. For later slices: timeline
scope, show preparation from music, the scripting language, import formats (for example GDTF),
concepts per familiarity level, Android as USB host.

## Sources in the earlier controller

Read as facts and behaviour, not as design to copy:

| Topic | Source | What it gives |
|---|---|---|
| DMX4ALL wire protocol | [`docs/DMX4ALL_PROTOCOL.md`](https://github.com/s-kroonen/DMX-Controller/blob/main/docs/DMX4ALL_PROTOCOL.md), `backend/app/dmx/dmx4all.py` | Verified commands (connection check, acknowledged block write, blackout, slot count), firmware pitfalls, read-back for verification |
| Saving the config | `backend/app/storage.py`, README "Saved rooms" | What a venue consists of; one data folder; named snapshots; bundled profiles versus user copies |
| Fixture profiles | `backend/app/fixtures/`, README "Fixtures with zones" | Which fields real fixtures need: fine channels, shared dimmer/strobe channels, named ranges, zones |
| Aiming and calibration | `backend/app/room/`, README "Mounting and aiming", "Calibrating the heads" | Mounting conventions (standing, hanging, wall), flip-over reach, out-of-range handling, solving mounting from marks |
| Sound-to-light | `backend/app/audio/`, README "Sound-to-light" | The analysis (bands, beat, BPM, tap) and effect set that worked at events |
| Features and UX | [README](https://github.com/s-kroonen/DMX-Controller/blob/main/README.md) | Edit and show modes, floating windows, phone layout |
