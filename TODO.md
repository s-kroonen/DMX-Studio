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
| DMX-7 | chore | Spike: measure throughput and latency of the DMX4ALL Mini-USB at show load, and update-loop jitter of .NET and Rust on Windows | todo | agentic | 1 day | `spike/DMX-?-dongle-throughput`; real dongle needed; answers ASR-1, ASR-2; .NET jitter decides the Rust + .NET fallback |
| DMX-9 | chore | Spike: 3D room view with picking in Avalonia and in egui, side by side | todo | agentic | 2 days | `spike/DMX-?-3d-view`; Linux desktop and browser, ≥ 30 fps with 50 fixtures; egui result decides the Rust + .NET fallback |
| DMX-8 | chore | Spike: audio loopback capture on Windows and Linux with cpal (Rust) and NAudio plus a Linux library (.NET) | todo | agentic | 1 day | `spike/DMX-?-audio-loopback`; delay from test tone to detected onset |

- **Type**: feature, bug, release, chore, docs. **Status**: todo, doing, review, done, dropped.
- **Time-box**: for research and spike items, how long they may take (`4h`, `2 days`); at the box the
  work stops and hands over, or a person extends it on purpose.
- **Level**: from the work type (AGENTS.md section 2); a bug can be set to `ai-assist` when escalated.
- Branch: `<type>/DMX-<n>-<slug>`; commit scope: `DMX-<n>`.
