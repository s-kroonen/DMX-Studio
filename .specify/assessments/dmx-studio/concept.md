# Concept: DMX Studio, a rebuild of the DMX Controller

- **Slug**: dmx-studio
- **Created**: 2026-11-26
- **Recommended option**: Option C — Rebuild in vertical slices

## Options

### Option A — Do nothing, or use QLC+ for shows

- **Sketch**: Keep running the earlier controller as it is and stop adding features, or move show
  work to QLC+ (free, cross-platform, supports DMX4ALL interfaces and a fixture library) and keep the
  earlier controller only for 3D aiming.
- **Appetite**: small (days: learn QLC+, patch the fixtures).
- **Trade-offs**: Wins: no build effort, a mature tool with cues, effects and a fixture library
  today. Sacrifices: 3D room aiming and beam safety zones (the reasons the tool exists) stay in the
  frozen earlier controller or are lost; two tools at a show; no path to the owner's own features;
  the earlier controller still breaks when the discontinued interface fails. Fails the goals
  "change without fear" and "beams provably stay out of zones on every path".
- **Rabbit holes**: running two programs against one serial port (only one process can hold it).

### Option B — Adopt dev-standard on the earlier controller and harden it

- **Sketch**: Bring the earlier repository under dev-standard (the adoption path in
  `docs/adopting.md` of the standard), add tests to the untested JavaScript half, then grow
  features in place. The operator sees the same tool, gradually more reliable.
- **Appetite**: medium (weeks to cover the frontend with tests and pass the gates), then ongoing.
- **Trade-offs**: Wins: keeps every working feature from day one; no period without a tool; the
  DMX4ALL driver and IK solver stay as proven code. Sacrifices: keeps the Python server plus
  hand-written JavaScript split the owner wants gone; retrofitting tests and types onto 6,500 lines
  of untyped JavaScript is slow; the native Android client and native host stack stay out of reach;
  the design keeps its six-day origins.
- **Rabbit holes**: testing a Three.js UI written without tests; the gates' stack-layer
  requirements (architecture, complexity, e2e) on code not built for them.

### Option C — Rebuild in vertical slices

- **Sketch**: Build DMX Studio from scratch on a stack chosen in DMX-4, one complete slice at a
  time, each usable on its own. The first release is the smallest tool that runs a safe show:
  DMX4ALL output with a simulator, fixtures and patching, live control of fixtures and groups, room
  model and aiming, safety zones, edit and show modes, venue save and load, basic sound-to-light,
  and control from a browser on the local network. The earlier controller stays in use until the
  first release covers what the owner uses at events; later slices add show building (scenes,
  cues, effects), animations, calibration, the fixture library, the developer API, scripting, the
  MCP server and the Android app. The earlier code is read for facts (DMX4ALL protocol, aiming
  conventions, sound effects), not ported.
- **Appetite**: large (months) for the whole vision; the first release is the budget that matters
  and is itself medium to large. [uncertain: no measure of the owner's available time]
- **Trade-offs**: Wins: meets the maintainability goal from the first line (every part under the
  gates), the native stack the owner asks for, beam safety designed in from the start rather than
  retrofitted, one repository for host, browser and Android. Sacrifices: weeks to months before the
  first release is usable at an event; re-implementing features that already work; risk of never
  reaching parity.
- **Rabbit holes**:
  - Building everything before using anything (big-bang rebuild): the vision lists about 40
    capabilities; only the first slice belongs in the first release.
  - The 3D view and aiming UI on a native stack: the earlier controller relied on Three.js in a
    browser.
  - One codebase for Windows, Linux, browser and Android: a stack that does all four may compromise
    each; the stack decision must keep this bounded.
  - Audio loopback capture differs per platform.
  - Familiarity levels (basic to advanced) multiply UI work; one level first.
  - Serial bandwidth at 38400 baud for fast effects across many channels.

## Recommendation

**Option C — Rebuild in vertical slices.** It is the only option that meets the goals "change
without fear" (every part under the enforced gates) and "beams provably stay out of zones on every
output path" without carrying the Python/JavaScript split the owner rules out. Option B keeps the
tool usable throughout but fights the gates on untyped, untested code and keeps the split. Option A
gives up the product's reason to exist. The slice approach answers C's main risk: the earlier
controller remains the show tool until the first slice replaces it, so no event depends on an
unfinished rebuild. The owner's answer "both" (product fit and owning the tool) also favours C.

## Out of Scope (for the recommended option)

- More than one universe, Art-Net, sACN, network smart lights. (maybe later)
- iOS.
- RDM (Future: no DMX4ALL USB interface offers it).
- Ableton Link in the first release (Next; licence resolved before building). (maybe later)
- User accounts and roles.
- Lasers and AR (maybe later).
- In the first release: scenes, cues, effects engine, animations, calibration solver, light-bar
  zones, fixture creator and import, developer API, scripting, MCP server, Android app, undo and
  shortcut editor, familiarity levels beyond one. All stay in the vision's later tiers.
- Porting code from the earlier controller; it is a source of facts only.

## Assumptions to Validate

- The earlier controller keeps working at events until the first release replaces it.
- The DMX4ALL command protocol behaves the same on the Mini-USB and the NanoDMX for block writes
  and blackout; differences are small and handled in one place.
- A stack exists that covers a Windows and Linux host, a browser client and later Android in one
  repository, with format, lint, type and test gates on every part (DMX-4 decides).
- The first release (the slice above) is small enough to finish in the owner's available time.
  [NEEDS CLARIFICATION: available time per week]
- Basic sound-to-light from the earlier controller (inputs, meters, beat, BPM, tap, six effects
  with their own targets, colour/motion/both) is enough for the first release.
- Beam safety can be checked in one place on the output side and covered by `safety`-tagged tests
  for every output path (constitution VII).
- A browser on the local network is an acceptable first phone client; the Android app can wait.
