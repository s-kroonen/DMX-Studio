# Decision: DMX Studio, a rebuild of the DMX Controller

- **Slug**: dmx-studio
- **Decided**: 2026-11-26
- **Verdict**: go
- **Artifacts reviewed**: intake.md | research.md | problem.md | concept.md

## Scorecard

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| Problem validity | strong | The owner operates and maintains the tool; half of the earlier controller is untested and its only verified interface is discontinued (research.md). |
| Evidence strength | adequate | Hardware, library and licence facts are cited; demand rests on one user, which fits a personal tool; event usage and incidents are not recorded. |
| Value vs. inaction | adequate | Inaction leaves a working but frozen tool that needs risky changes once the interface fails; QLC+ lacks 3D aiming and beam safety zones (problem.md "Cost of Inaction"). |
| Feasibility / appetite | adequate | Option C has a bounded first release; the whole vision is large and the owner's available time is unknown, so the first-release budget is the control. |
| Strategic fit | strong | Matches the constitution: spec before code, physical safety (VII) designed in, secure control surface (VIII), small slices (VI). |
| Risk posture | adequate | Main risks are named with mitigations: big-bang rebuild (slices, earlier controller stays in use), hardware end of life (target the DMX4ALL protocol), multi-platform stack (bounded in DMX-4), Link licence (deferred). Serial bandwidth and owner time remain open. |

## Verdict & Rationale

**Go**, with Option C (rebuild in vertical slices). The problem is real and owned by the person who
decides; the evidence is adequate for a personal tool and the strongest counter-evidence (a mature
free alternative, discontinued hardware, scope size) is answered by the concept: QLC+ lacks the
product's core, the target is the DMX4ALL protocol rather than one device, and the first release is
a single slice while the earlier controller stays the show tool. Feasibility is adequate, not
strong: the owner's available time is unknown, so the first release is the budget to watch.

## If needs-clarification

Not applicable.

## If go — Handoff to `/speckit-specify`

dev-standard order: a go goes to the stack choice first (`/stack-concerns` → `/stack-research` →
`/stack-decide`, item DMX-4); `/speckit-specify` follows for each slice once the stack decision is
accepted (`scripts/check` fails a plan without an accepted ADR 0002).

- **Problem**: the owner's lighting controller proves the right features but cannot be changed safely (untested half, two-language split, discontinued interface), while beam safety and aiming at real room points are its reason to exist.
- **Chosen approach**: Option C — rebuild from scratch in vertical slices on a native stack chosen in DMX-4; the earlier controller is a source of facts and stays in use until the first release replaces it.
- **In scope (first release)**: DMX4ALL-protocol output with a simulator, fixtures and patching, live control of fixtures and groups, room model and aiming, safety zones on every output path, edit and show modes, venue save and load, basic sound-to-light (inputs, meters, beat, BPM, tap, six effects with own targets, colour/motion/both), browser control on the local network (control and calibration, no edits, off by host setting).
- **Out of scope**: more universes, Art-Net, sACN, network smart lights, iOS, RDM (Future), Ableton Link in the first release (Next), accounts and roles, lasers, AR, porting earlier code; all later tiers of the vision.
- **Success metrics**: 0 software-caused failures per event; 0 beams into safety zones in safety tests across all output paths; 100 % of parts under lint, type and test gates; aim within 10 cm at 5 m after calibration; 6-fixture venue set up in under 30 minutes; input-to-light under 50 ms; adding a feature easier than in the earlier controller (qualitative).
- **Carried-forward open questions**:
  - Owner's available time per week (sizes the first release).
  - Event usage and incidents with the earlier controller.
  - Measured latency and refresh rate of the DMX4ALL interface at show load; Mini-USB versus NanoDMX differences.
  - Phone pairing and the exact meaning of "control and calibration, no edits".
  - A stack covering Windows and Linux host, browser and later Android in one repository with all gates (DMX-4).
  - Timeline scope, show preparation from music, scripting language, import formats (GDTF), concepts per familiarity level, Android USB host with PID C850 (later slices).
