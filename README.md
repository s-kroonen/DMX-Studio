# DMX Studio

DMX lighting controller with 3D room mapping, safety zones and a show and animation engine.

**Stage:** day 0, no code yet. Stack: not decided ([decision record](docs/decisions/0002-choose-the-stack.md)).

## Contents

- [About](#about)
- [Getting started](#getting-started)
- [How the project works](#how-the-project-works)
- [Checks](#checks)
- [How AI is used](#how-ai-is-used)
- [Releases](#releases)

## About

<What it does, main features, where it runs. Filled in from [the product vision](docs/product/vision.md).>

## Getting started

Requirements: Git, Bash (Windows: Git Bash or WSL), [lefthook](https://lefthook.dev),
[uv](https://docs.astral.sh/uv/) (for Spec Kit); Docker optional.
Runtime requirements follow once the stack is decided.

```bash
git clone <repository url>
cd dmx-studio
scripts/setup          # git hooks, pinned security scanners, rule sets
scripts/check all      # what CI runs
```

## How the project works

| Document | What |
|---|---|
| [CONSTITUTION.md](CONSTITUTION.md) | The principles; every plan is checked against them |
| [AGENTS.md](AGENTS.md) | Working rules: work types, AI levels, rules for people and AI tools |
| [docs/process/writing.md](docs/process/writing.md) | The writing rule for docs, comments and PR text |
| [TODO.md](TODO.md) | Issues `DMX-<n>` |
| [docs/process/workflow.md](docs/process/workflow.md) | Idea → decision → feature / bug / release |
| [docs/decisions/](docs/decisions/) | Decision records |

Commits are [Conventional Commits](https://www.conventionalcommits.org/) with the issue as scope:
`feat(DMX-12): add fixture groups`.

## Checks

Every check runs through `scripts/check`: the git hooks, the AI agent and CI call the same stages.

| Command | What |
|---|---|
| `scripts/check fast` | constitution in sync, format, lint, typecheck, secrets in staged files |
| `scripts/check test` | tests |
| `scripts/check research` | research notes dated and verified; decisions cite evidence (in `fast`) |
| `scripts/check stack-layer` | after the first code: every playbook command in `project.config` decided |
| `scripts/check licenses` | forbidden and restricted licences (trivy) |
| `scripts/check docs` | markdownlint, relative links and the writing rule (Vale) |
| `scripts/check pr-text` | the PR or MR body (env `PR_BODY`) against the writing rule and the watermark ban |
| `scripts/verify-package` | does a package exist, its licence and last release (registry API) |
| `scripts/check secrets` | secrets in the full history (gitleaks) |
| `scripts/check deps` | fixable high/critical vulnerabilities in production dependencies (trivy) |
| `scripts/check sast` | static security analysis (opengrep) |
| `scripts/check config` | Dockerfile / IaC misconfiguration (trivy) |
| `scripts/check canary` | the scanners still find the planted bugs in `tests/security-canary/` |
| `scripts/check standard-version` | is a newer dev-standard out? prints its upgrade notes (never fails) |
| `scripts/update-standard <DMX-n>` | new version of dev-standard on its own branch, with a report |
| `scripts/check all` | all of the above |

## How AI is used

AI tools (mainly Claude Code, with [GitHub Spec Kit](https://github.github.com/spec-kit/) for specs, idea assessment and bug analysis) work here at a level that depends on the work:

- **Features (ai-assist):** AI helps write specs, plans, code and tests on request. A person reviews and edits every change and makes every commit.
- **Bug fixes (agentic):** an agent finds the cause, fixes it with a regression test on its own branch and opens a pull request; a person reviews and merges. Bugs touching design, security, data or hardware safety go to a person.
- **Standard updates (ai-assist):** a person runs `scripts/update-standard`, with AI help for conflicts.
- **Releases (agentic):** an agent computes the version from the commits and prepares the changelog on a release branch; a person merges and tags.

Commits, pull requests, code and docs carry no AI watermark (no `Co-Authored-By` trailer naming a
tool, no session link, no "generated with" line); the role of AI is described here and in
[AGENTS.md](AGENTS.md), once. Agent work stays traceable through its branch (`claude/`, `ccr-`, the
agentic work types), the pull request label `agent` and the bot author of Action commits. The rules
every AI tool follows are in [AGENTS.md](AGENTS.md); the files that check and constrain them are
listed in [`.guardrails`](.guardrails).

## Releases

Versions follow [Semantic Versioning](https://semver.org/), computed from the commit types
(`scripts/next-version`). See [CHANGELOG.md](CHANGELOG.md).

## Updating the standard

Set up from dev-standard (`_src_path` in `.copier-answers.yml`). A new version arrives with
`scripts/update-standard DMX-<n>`: standard files are replaced, shared files are merged, project files
stay as they are. `project.config` holds this project's settings; `check.config` belongs to the
standard. Details: [`docs/process/updating.md`](docs/process/updating.md).
