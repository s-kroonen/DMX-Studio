# Problem Definition: DMX Studio, a rebuild of the DMX Controller

- **Slug**: dmx-studio
- **Created**: 2026-11-26
- **Inputs used**: intake.md, research.md, owner answers after research (2026-11-26)

## Problem Statement

The owner runs moving heads and colour lights at events from one USB DMX interface, and the tool
that does this today, the earlier DMX Controller, proves the right features but cannot be changed
safely: half of it has no tests, it splits into two languages, and its hardware is end of life.
Every change risks a show, and every feature added makes the next one harder, while beam safety
and aiming at real points in a room are the reasons the tool exists at all.

## Affected Users & Stakeholders

- **Users**:
  - **Owner as operator** — runs the lights at events; loses time and confidence when the tool
    breaks or misses a feature during a show. [source: intake.md, research.md "Users & Demand"]
  - **Owner as maintainer** — the only developer; the untested JavaScript half (6,500 lines, no
    tests) and the Python/JavaScript split make changes slow and risky. [source: research.md]
  - **Helper on a phone** — controls lights from the floor or stage. Anticipated, not observed.
    [NEEDS CLARIFICATION: who these helpers are and how often they take part]
  - **Developer writing scripts and add-ons** — anticipated, not observed.
    [NEEDS CLARIFICATION: whether anyone besides the owner writes scripts]
  - **People in the room (audience, performers)** — not users, but exposed to beams; the reason
    beam safety is non-negotiable. [source: CONSTITUTION.md VII]
- **Stakeholders**:
  - **Owner** — decides scope, accepts the stack decision, merges every change, owns every release.
  - **Hardware vendor (DMX4ALL)** — no stake, but the end of life of the Mini-USB interface
    constrains the owner's options. [source: research.md "Data & Constraints"]

## Goals

- The owner can run a full event (aiming, colours, strobe, sound-to-light) with no failure caused
  by the software.
- The owner can add or change a feature without fear of breaking what works: every part is covered
  by the enforced checks of dev-standard.
- Beams stay out of configured no-go zones on every output path, provably.
- Aiming a fixture at a real point in the room is faster than nudging raw pan and tilt.
- The tool keeps working when the current interface fails and a DMX4ALL-protocol replacement is
  bought.
- Someone who knows established lighting tools recognises how to work it.

## Non-Goals

- Competing with professional consoles or replacing QLC+ for other users.
- More than one DMX universe, network DMX output (Art-Net, sACN), or network smart lights
  (Home Assistant bulbs are too slow for show control). [source: intake.md, owner answers]
- iOS. [source: intake.md]
- RDM in the first releases: no DMX4ALL USB interface offers it (moved to Future).
  [source: research.md; owner answer]
- Ableton Link in the first version (moved to Next; licence resolved before building).
  [source: research.md; owner answer]
- User accounts and roles. [source: intake.md]
- Lasers and AR until hardware and need exist. [source: intake.md]

## Success Metrics

- Software-caused failures during events: 0 per event (baseline: unknown —
  [NEEDS CLARIFICATION: incidents at events with the earlier controller]).
- Beams entering a safety zone in automated safety tests: 0 across all output paths (baseline:
  earlier controller checks aims with a ray/box test; coverage across all paths unknown).
- Share of code covered by enforced lint, type and test gates: 100 % of parts (baseline: Python
  part tested, JavaScript part untested — research.md).
- Aim accuracy after calibration: within 10 cm at 5 m (baseline: unknown; proposed target from the
  vision).
- Setup of a venue with 6 fixtures and one safety zone: under 30 minutes (baseline: unknown;
  proposed target).
- Input-to-light latency on the DMX4ALL interface: under 50 ms (baseline: not measured; the
  38400-baud link limits full refreshes to about 0.14 s, so only changed channels can be sent).
- Qualitative: the owner reports adding a feature to the new tool is easier than to the earlier
  controller.

## Cost of Inaction

The earlier controller keeps working on the current dongle. Each change to it stays risky because
of the untested half, so features stall; when the discontinued interface fails, the tool needs
changes in exactly that risky code. The owner keeps a working but frozen tool, or falls back to
general tools (QLC+, FreeStyler) that lack 3D aiming and beam safety zones.

## Open Questions

- [NEEDS CLARIFICATION: how often and where the earlier controller ran at events, and what failed there]
- [NEEDS CLARIFICATION: which capabilities form the first usable release]
- [NEEDS CLARIFICATION: measured latency and refresh rate of the DMX4ALL interface at show load]
- [NEEDS CLARIFICATION: differences between the Mini-USB and the NanoDMX command handling]
- [NEEDS CLARIFICATION: who the phone helpers and script developers are, if anyone besides the owner]
- [NEEDS CLARIFICATION: how a phone pairs with the host and what "control and calibration, no edits" covers]
- [NEEDS CLARIFICATION: timeline scope and show preparation from music]
- [NEEDS CLARIFICATION: concepts of the reference tools per familiarity level]
- [NEEDS CLARIFICATION: Android USB host with PID C850 on real phones]
