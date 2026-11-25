# TODO — DMX Studio

The issue tracker for this project. Every change belongs to one item. Agents may add items
(status `todo`) with the ID `DMX-?`; a person gives the number on `dev` (two branches
can't take the same one; CI `todo-ids` checks it). Only a person changes an item to `done` or
removes it.

**Next free number: DMX-7**

| ID | Type | Title | Status | Level | Time-box | Notes |
|----|------|-------|--------|-------|----------|-------|
| DMX-1 | chore | Set up the repository from dev-standard | done | ai-assist | | day 0 |
| DMX-2 | chore | Write the product vision (`docs/product/vision.md`) | todo | ai-assist | | |
| DMX-3 | chore | Assess the idea (`/speckit-assess-*`) and research the options | todo | ai-assist | | before the stack |
| DMX-4 | chore | Decide the stack (`docs/decisions/0002-choose-the-stack.md`) | todo | ai-assist | | then add the stack layer |
| DMX-5 | chore | Add the stack layer (`docs/process/stack-layer.md`, skill `/stack-layer`) | todo | ai-assist | | after DMX-4 is accepted; again when the code grows |
| DMX-6 | chore | Review the research after the first working build (playbook step 14) | todo | ai-assist | | update the ASRs, retrospective for a remake, concerns that came up, stale notes |

- **Type**: feature, bug, release, chore, docs. **Status**: todo, doing, review, done, dropped.
- **Time-box**: for research and spike items, how long they may take (`4h`, `2 days`); at the box the
  work stops and hands over, or a person extends it on purpose.
- **Level**: from the work type (AGENTS.md section 2); a bug can be set to `ai-assist` when escalated.
- Branch: `<type>/DMX-<n>-<slug>`; commit scope: `DMX-<n>`.
