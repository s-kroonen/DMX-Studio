# TODO — DMX Studio

The issue tracker for this project. Every change belongs to one item. Agents may add items
(status `todo`) with the ID `DMX-?`; a person gives the number on `dev` (two branches
can't take the same one; CI `todo-ids` checks it). Only a person changes an item to `done` or
removes it.

**Next free number: DMX-7**

| ID | Type | Title | Status | Level | Time-box | Notes |
|----|------|-------|--------|-------|----------|-------|
| DMX-1 | chore | Set up the repository from dev-standard | done | ai-assist | | day 0 |
| DMX-2 | chore | Write the product vision (`docs/product/vision.md`) | done | ai-assist | | |
| DMX-3 | chore | Assess the idea (`/speckit-assess-*`) and research the options | done | ai-assist | | before the stack |
| DMX-4 | chore | Decide the stack (`docs/decisions/0002-choose-the-stack.md`) | todo | ai-assist | | then add the stack layer |
| DMX-5 | chore | Add the stack layer (`docs/process/stack-layer.md`, skill `/stack-layer`) | todo | ai-assist | | after DMX-4 is accepted; again when the code grows |
| DMX-6 | chore | Review the research after the first working build (playbook step 14) | todo | ai-assist | | update the ASRs, retrospective for a remake, concerns that came up, stale notes |
| DMX-? | chore | Research C-1: output to the DMX4ALL interface over USB serial | todo | ai-assist | 1 day | `docs/product/quality-attributes.md`; needs the spike below |
| DMX-? | chore | Spike: measure throughput and latency of the DMX4ALL Mini-USB at show load | todo | agentic | 1 day | `spike/DMX-?-dongle-throughput`; real dongle needed; answers ASR-1, ASR-2 |
| DMX-? | chore | Research C-2: show engine update loop and output frame merging | todo | ai-assist | 4h | |
| DMX-? | chore | Research C-3: safety enforcement on the output side and safe state | todo | ai-assist | 4h | constitution VII |
| DMX-? | chore | Research C-4: desktop UI toolkit for Windows and Linux | todo | ai-assist | 1 day | |
| DMX-? | chore | Research C-5: 3D room view and aiming in the desktop toolkit | todo | ai-assist | 1 day | needs the spike below |
| DMX-? | chore | Spike: 3D room view with picking in the leading toolkit | todo | agentic | 2 days | `spike/DMX-?-3d-view`; after C-4 research |
| DMX-? | chore | Research C-6: browser client and Android app sharing code with the host | todo | ai-assist | 4h | after C-4 |
| DMX-? | chore | Research C-7: control surface (API, OpenAPI, live updates, pairing, MCP) | todo | ai-assist | 4h | constitution VIII |
| DMX-? | chore | Research C-8: audio capture, loopback and beat analysis on Windows and Linux | todo | ai-assist | 1 day | needs the spike below |
| DMX-? | chore | Spike: audio loopback capture on Windows and Linux | todo | agentic | 1 day | `spike/DMX-?-audio-loopback` |
| DMX-? | chore | Research C-9: venue storage format and migrations | todo | ai-assist | 4h | |
| DMX-? | chore | Research C-10: maths libraries for aiming and the calibration solver | todo | ai-assist | 4h | |
| DMX-? | chore | Research C-11: packaging (Windows installer and portable, Linux, Android) | todo | ai-assist | 4h | |
| DMX-? | chore | Research C-12: dev-standard gates for Rust, Flutter and .NET | todo | ai-assist | 4h | dotnet and flutter profiles are unverified in dev-standard |
| DMX-? | chore | Research C-13: libraries for scripting, MIDI, OSC, Ableton Link, Android USB host | todo | ai-assist | 4h | later slices; licence check for Link |

- **Type**: feature, bug, release, chore, docs. **Status**: todo, doing, review, done, dropped.
- **Time-box**: for research and spike items, how long they may take (`4h`, `2 days`); at the box the
  work stops and hands over, or a person extends it on purpose.
- **Level**: from the work type (AGENTS.md section 2); a bug can be set to `ai-assist` when escalated.
- Branch: `<type>/DMX-<n>-<slug>`; commit scope: `DMX-<n>`.
