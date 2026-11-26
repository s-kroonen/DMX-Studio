# Idea Intake: DMX Studio, a rebuild of the DMX Controller

- **Slug**: dmx-studio
- **Created**: 2026-11-26
- **Source**: repo path `docs/product/vision.md`; https://github.com/s-kroonen/DMX-Controller (README.md, docs/DMX4ALL_PROTOCOL.md); pasted text (owner's messages during DMX-2)
- **Type**: new-capability

## Idea (as captured)

Source of the earlier controller: https://github.com/s-kroonen/DMX-Controller, host `github.com`,
URL Trust Policy branch `allowlisted`, read as a git clone (read-only).
Title: "DMX Controller". Excerpt from its README:

> A self-built, freestyler-style DMX lighting controller with real 3D room mapping: aim moving
> heads/lasers at points in your actual room instead of juggling raw pan/tilt values, with safety
> zones, groups, overlaid RGB/strobe/pan-tilt control windows, a fixture creator, and a keyframed
> animation engine -- served as a web app from a Python backend that talks to a DMX4ALL Mini-USB
> interface.

The owner's request (pasted text, 2026-11-25):

> we need to start building a new version of my dmx controller [...] completly decople the stack
> from the existing code we only want high level features and only use code as a source like the
> dmx4all dongle interface or how the config is saved but right now we dont want to lock anything
> down

The owner's additions (pasted text, 2026-11-26), verbatim:

> i want to add some more that are new like a deeper mobile client and native stack so no python
> with a javascript anymore it has to be maintainable which is what the spec kit is for and the
> advanced rules. for now we can keep the output simple its just a single donge with one universe
> and while i do have some other options like for instance a home assistant with rgb lights this
> is no where near fast enough to be usable. then other features like the scenes and presets are
> good midi control is a nice to have and i dont know much about it. then what is osc. cue lists
> with fades is a good idea as well as effects. we have good sound control base that i definitly
> want to take to the new version but mobile and browser mic is a new thing as well as the ableton
> link which is somthing worht looking into. timeline is also somthing worth discussing in more
> detail. roles are a bit overkill but we can limit what the phone can do like no edits just
> control and calibration and dissble it outright if the server settings do so, gryo aim is a cool
> gimic as well as a calibration utility or position calibration. undo redo and common shortcuts
> with shortcut editor are a good should have. rdm is also a good addition with a library of
> fixtures and the posibility to import from other programs via a file. lasers are a future plan
> we dont have hardware for that but safety zones were a thing in the old version. then for ease
> of use and debugging we can add a local mcp server for agents to assist with the setup and also
> a api with a openapi page or docs so a dev can make their own additions and scripts to run for
> instance advanced animations or execute things. then as a last thing the new version should be
> based loosely of already existing hardware and programs so a veteran of the trade can recognise
> certain features. as nice to haves a scanning tool for mobile setup so based on lighting and gyro
> we can calibrate each fixture but thats a hard thing to do as well as maybe ar solutions. then a
> way to make whole shows of a setup from music input before hand a simulation with movment and
> lighting. also a movement and light enging of sound thats advanced so it detects beat drops music
> genre and other so lights adjust to it.

Answers given by the owner to clarifying questions (2026-11-26):

- Phone client: "a nice to have addition is phone as a usb link to the dmx dongle. my prefrence
  goes to a native app with a browser fallback but details should be decide later"; browser
  first, the app follows; app and studio in the same repository.
- Platforms: Windows host, Linux host; Android phone as a nice to have, "if going multi platform
  limits us too much we will work around it with a usb forward in the browser or a other way to
  get the phone to run shows, apple not needed".
- Scripting: both an external API and scripts inside the app.
- Reference programs: grandMA / ChamSys, SoundSwitch / Lightkey, FreeStyler, QLC+, Obsidian ONYX;
  "diffrent styles for the app from basic to advanced based on these options".
- Sound: "at least in its basic form should be in the first version and the old code has good
  fetures for this for lights groups beats and effects".

The full feature list, tiers, constraints and open questions are in `docs/product/vision.md`.

## Restated

A rebuild of the owner's DMX lighting controller on a native, maintainable stack: one-universe DMX
output through a USB interface, 3D room-based aiming with safety zones, live control, sound-to-light,
and show building (scenes, cues, effects), with a browser and later an Android client, a developer
API, scripting and a local MCP server. The earlier controller supplies proven features and hardware
facts, not its design.

## Origin & Context

- **Raised by**: the project owner (s-kroonen), sole maintainer and operator.
- **Trigger**: the earlier controller (Python backend with a vanilla JavaScript frontend, 48
  commits, last change 2026-09-22) proved the features but is hard to maintain; the owner wants a
  rebuild under the dev-standard process (spec-driven, enforced checks) before choosing a stack.

## First-Glance Unknowns

- [NEEDS CLARIFICATION: which features form the first usable release, given the size of the list]
- [NEEDS CLARIFICATION: does the DMX4ALL Mini-USB interface (firmware V3.36) support RDM]
- [NEEDS CLARIFICATION: can an Android phone act as USB host for the FTDI-based DMX4ALL interface]
- [NEEDS CLARIFICATION: licence of the Ableton Link SDK and whether it passes the licence gate]
- [NEEDS CLARIFICATION: timeline scope: full editor against a track, or cue timing only]
- [NEEDS CLARIFICATION: show preparation from music: automatic, assisted, or authored with a preview]
- [NEEDS CLARIFICATION: embedded scripting: which language and what a script may touch]
- [NEEDS CLARIFICATION: fixture import formats besides QLC+ `.qxf` (for example GDTF), and which programs]
- [NEEDS CLARIFICATION: which concepts from grandMA, ChamSys, ONYX, QLC+, FreeStyler, SoundSwitch and Lightkey map to the basic, standard and advanced levels]
- [NEEDS CLARIFICATION: how a phone pairs with the host and what "control and calibration, no edits" covers exactly]
- [NEEDS CLARIFICATION: whether one codebase can serve the Windows and Linux host and Android without limiting the stack]
- [NEEDS CLARIFICATION: latency budget from input to DMX output at 38400 baud]
