# Stack research: toolability with the dev-standard gates

- **Concern**: C-12 (drivers: QA-9, ASR-10)
- **Issue**: DMX-4
- **Date**: 2026-11-26
- **Question**: Which language options can run every dev-standard gate (format, lint, types, tests, coverage, complexity, fuzzing, dependency audit, SAST, writing rule) on every part of DMX Studio?
- **Time-box**: 4h
- **Status**: answered
- **Sources**: dev-standard `stacks/README.md` and `template/check.config.jinja` at tag v0.3.0 (read 2026-11-26); `docs/process/writing.md` of this repository (read 2026-11-26); opengrep README, github.com/opengrep/opengrep (read 2026-11-26); opengrep-rules tree at commit f1d2b562b414783763fd02a6ed2736eaed622efa, github.com/opengrep/opengrep-rules (read 2026-11-26); rust-clippy `clippy_lints/src/cognitive_complexity.rs`, github.com/rust-lang/rust-clippy (read 2026-11-26); GitHub repository metadata of JetBrains/compose-multiplatform, detekt/detekt, ktlint/ktlint, Kotlin/kotlinx-kover, CodeIntelligenceTesting/jazzer, qt/qtbase, llvm/llvm-project, dotnet/roslyn-analyzers (read 2026-11-26)
- **Verified**: `scripts/verify-package`, 2026-11-26: crates cargo-llvm-cov 0.9.2 (Apache-2.0 OR MIT, released 2026-11-20); crates cargo-fuzz 0.13.2 (MIT OR Apache-2.0, 2026-06-09); crates cargo-deny 0.22.1 (MIT OR Apache-2.0, 2026-11-24); crates cargo-audit 0.23.0 (Apache-2.0 OR MIT, 2026-11-10); crates flutter_rust_bridge 2.13.0 (MIT, 2026-08-23); nuget coverlet.collector 10.1.1 (MIT, 2026-11-15); nuget SharpFuzz 2.3.0 (MIT, 2026-06-16); nuget Microsoft.CodeAnalysis.NetAnalyzers 11.0.100 (MIT, 2026-11-10); pub dart_code_metrics 5.7.6 (licence unknown, last release 2023-07-16); pub flutter_rust_bridge 2.13.0 (MIT, 2026-08-23); pub very_good_analysis 11.0.0 (MIT, 2026-09-03). Maven and C++ tools: `scripts/verify-package` has no Maven or C++ registry; checked through GitHub instead (releases and archive status, 2026-11-26): compose-multiplatform v1.12.2 (2026-10-15, Apache-2.0), detekt v1.23.8 (2025-02-21, Apache-2.0, repository active), ktlint 1.8.0 (2025-11-14, MIT), kotlinx-kover v0.9.11 (2026-09-29, Apache-2.0), jazzer (Apache-2.0, pushed 2026-11-20), llvm-project llvmorg-23.1.6 (2026-11-17), qtbase (mirror, no GitHub releases)
- **Re-check by**: 2027-02-24

## Question

Every part of DMX Studio runs the gates of `scripts/check` (ASR-10). Which language options have
the tools for that, and how much does mixing two languages cost?

## Previous version (remakes)

v1 ran pytest on the Python half and nothing on the 6,500 lines of JavaScript. The untested half
is the main reason for the rebuild (`.specify/assessments/dmx-studio/problem.md`).

## Options

The owner maintains Rust, Dart/Flutter and C#/.NET, and accepts languages without a dev-standard
profile, for example Kotlin or C++ (owner, 2026-11-26). Options that cover host, browser client and
Android with one language are listed beside one mixed option.

| Option | Package(s) verified (version, last release, licence, checked on) | Notes |
|---|---|---|
| A: Rust only | cargo-llvm-cov 0.9.2, cargo-fuzz 0.13.2, cargo-deny 0.22.1, cargo-audit 0.23.0 (2026-11-26) | One language only if a Rust UI toolkit covers desktop, browser and Android (C-4, C-6) |
| B: Dart/Flutter only | very_good_analysis 11.0.0; dart_code_metrics 5.7.6 last released 2023-07-16 (2026-11-26) | Flutter targets Windows, Linux, web and Android from one codebase |
| C: C#/.NET only | coverlet.collector 10.1.1, SharpFuzz 2.3.0, NetAnalyzers 11.0.100 (2026-11-26) | One language only if the UI toolkit covers desktop, browser and Android (C-4) |
| D: Kotlin only | compose-multiplatform v1.12.2, detekt v1.23.8, ktlint 1.8.0, kover v0.9.11, jazzer (GitHub, 2026-11-26) | No dev-standard profile; Compose Multiplatform targets desktop, Android and web |
| E: C++ only | llvm-project 23.1.6 (clang-format, clang-tidy, llvm-cov, libFuzzer); qtbase (GitHub, 2026-11-26) | No dev-standard profile; Qt targets desktop, Android and WebAssembly |
| F: Rust core + Flutter UI | flutter_rust_bridge 2.13.0 on crates and pub (2026-11-26) plus A and B | Two languages, two gate sets, a generated bridge |
| G: Rust engine + .NET host (owner asked, 2026-11-26; fallback) | tonic 0.14.9 (MIT, 2026-10-29); Grpc.Net.Client 2.85.0 (Apache-2.0, 2026-10-29); csbindgen 1.9.8 (MIT, 2026-05-20); interoptopus 0.16.6 (MIT, 2026-10-09); uniffi 0.33.0 (MPL-2.0, 2026-11-25); cargo-ndk 4.1.2 (Apache-2.0 OR MIT, 2025-08-09) plus A and C (2026-11-26) | Rust process owns the interface, safety check, show engine and audio; the .NET host carries the UI, API, browser client and MCP; local IPC (gRPC) between them |

## Criteria and weights (agreed with the owner before scoring)

Agreed 2026-11-26. The owner lowered the profile weight (languages without a profile are
acceptable) and raised the mixing cost (avoid two languages unless they bring other benefits).

| Criterion | Weight | Source (QA-…) |
|---|---|---|
| Profile proven in dev-standard (stages run end to end) | 1 | QA-9 |
| Gate completeness (coverage, complexity, fuzzing, dependency audit) | 3 | QA-9, ASR-10 |
| SAST coverage (open and custom rules, security canary) | 2 | QA-7, constitution IV |
| Cost of mixing two languages (two profiles, glue code) | 3 | QA-9 |
| Writing rule reads comments (Vale) | 1 | QA-9, `docs/process/writing.md` |

## Scores

Scores 1–5; weighted total out of 50.

| Option | Profile (1) | Gates (3) | SAST (2) | Mixing (3) | Vale (1) | Weighted total | Reason |
|---|---|---|---|---|---|---|---|
| A: Rust only | 4 | 4 | 4 | 5 | 4 | **43** | Profile verified except coverage; all gate tools exist; complexity only through clippy's `restriction` lints; 10 open rules plus custom rules 6/6 on the canary; Vale reads line comments only |
| C: .NET only | 2 | 4 | 4 | 5 | 5 | **42** | Profile rendered only, never run; analyzers, coverlet and SharpFuzz exist; 52 open rules plus custom rules 7/7 |
| D: Kotlin only | 1 | 4 | 3 | 5 | 5 | **39** | No profile; detekt, ktlint, Kover and Jazzer exist; 14 Kotlin and 121 Java open rules, no custom rules or canary |
| E: C++ only | 1 | 3 | 2 | 5 | 5 | **34** | No profile; clang-tidy, llvm-cov and libFuzzer are strong; dependency audit for C++ package managers is weak (ASSUMPTION); 16 C rules, no C++ rule set at the pinned commit |
| B: Dart/Flutter only | 2 | 2 | 1 | 5 | 1 | **26** | Profile rendered only; no maintained complexity tool, no fuzzer known (ASSUMPTION); 0 open Dart rules, no custom rules, no canary; Vale does not read Dart |
| F: Rust + Flutter | 2 | 3 | 3 | 2 | 3 | **26** | Rust side strong, Dart side as in B; two gate sets and a generated bridge; the `stack` question takes one value, so the gates are set by hand in `project.config` |
| G: Rust + .NET | 3 | 4 | 4 | 2 | 4 | **34** | Both sides have full gate sets and custom SAST rules; two toolchains, gates set by hand in `project.config`, a contract between the processes (`CONTRACT_CMD`); the .NET profile is unverified |

## Toolability (dev-standard gates)

| Option | Profile in dev-standard | Custom security rules | Security canary | Quality canary | Measured SAST (open → open + custom) |
|---|---|---|---|---|---|
| Rust | yes; verified 2026-11-24 except coverage | yes (`rust-security.yaml`, axum) | yes | yes | 1/6 → 6/6 |
| .NET | yes; rendered only, unverified | yes (`aspnet-security.yaml`) | yes | no | 2/7 → 7/7 |
| Dart/Flutter | yes; rendered only, no SAST rules | no | no | no | not measured |
| Kotlin | no | no | no | no | not measured |
| C++ | no | no | no | no | not measured |

The profiles define format, lint, typecheck, test, coverage and safety-test commands
(`template/check.config.jinja`, v0.3.0). The playbook gates (`ARCH_CMD`, `COMPLEXITY_CMD`,
`PERF_CMD`, `E2E_CMD`, `CONTRACT_CMD`, `SECURITY_NATIVE_CMD`, `FUZZ_CMD`, `DAST_CMD`) are set by the
project for every option (`stacks/README.md`).

## Evidence

- The `stack` question takes one value (`none`, `python`, `node`, `rust`, `dotnet`, `flutter`); a mixed option sets its gate commands by hand in `project.config`, which is sourced after `check.config`. — [dev-standard `copier.yml`, README "Updating a project", read 2026-11-26] (high)
- Rust profile: `cargo fmt`, `cargo clippy -D warnings`, `cargo test`, `cargo llvm-cov`; safety tests by name filter `safety`. Verified 2026-11-24 except coverage. — [dev-standard `template/check.config.jinja` and `stacks/README.md` at v0.3.0, read 2026-11-26] (high)
- .NET profile: `dotnet format`, `dotnet build -warnaserror`, `dotnet test`, coverlet; safety tests by `Category=Safety`. Rendered only, unverified (no SDK in the dev-standard test run). — [same] (high)
- Flutter profile: `dart format`, `flutter analyze --fatal-infos`, `flutter test`; `BUILD_CMD` builds for the web. Rendered only; no SAST rules for Dart. — [same] (high)
- Open SAST rules at the pinned opengrep-rules commit: Python 334, JavaScript 173, Java 121, C# 52, TypeScript 30, Scala 27, C 16, Kotlin 14, Rust 10, Dart 0. — [opengrep-rules tree at f1d2b56, counted 2026-11-26] (high)
- opengrep parses Dart, C#, Rust, Kotlin, C and C++. — [opengrep README, read 2026-11-26] (high)
- Clippy's `cognitive_complexity` sits in the `restriction` group; its documentation says it does not measure cognitive complexity and points to `excessive_nesting` and `too_many_lines`. — [rust-clippy `cognitive_complexity.rs`, read 2026-11-26] (high)
- `dart_code_metrics` had its last release on 2023-07-16; no maintained free complexity tool for Dart was found. — [verify-package pub, 2026-11-26] (medium)
- `dotnet/roslyn-analyzers` is archived; the NetAnalyzers package is still released (11.0.100, 2026-11-10). — [GitHub, verify-package nuget, 2026-11-26] (high)
- Vale reads comments of Rust (line comments only), C#, Kotlin, Java, C and C++; not Dart. — [`docs/process/writing.md`, read 2026-11-26] (high)
- No Dart fuzzer comparable to cargo-fuzz, SharpFuzz or Jazzer is known. — ASSUMPTION (low)
- trivy and osv-scanner cover Cargo, NuGet, pub and Maven lockfiles better than C++ package managers. — ASSUMPTION (medium)

## Evidence against the leading option

Rust leads by one point over .NET, inside the error of the scores. Its single-language score
depends on a Rust UI toolkit that covers desktop, browser and Android with a 3D view (C-4, C-5,
C-6): if none does, Rust turns into option F, which scores 26. Rust coverage (`cargo llvm-cov`) is
the one profile stage dev-standard did not verify. The custom Rust rules target an axum web
server; a desktop app with a local API uses only part of them. Complexity has no proper gate in
Rust: `too_many_lines` and `excessive_nesting` are proxies.

## Spike needed?

No for toolability itself. The decisive question moves to C-4 (UI toolkit): whether Rust, .NET or
Kotlin covers all three clients in one language.

## Recommendation (confidence: medium)

Toolability rules out **Dart/Flutter** as the language for the core and the **Rust + Flutter** mix:
Dart has no SAST rules, no maintained complexity tool and no writing-rule coverage, and the mix
adds a second gate set. **Rust (43) and .NET (42) tie** on toolability; **Kotlin (39)** is close
without a profile. C-4 and C-6 decide between them: the option whose UI toolkit covers desktop,
browser and Android in one language keeps its score.

## Fallback: option G (Rust engine + .NET host)

Taking the better side per concern from the other notes gives G about 448 of 515 points, against
446 for Rust only and 436 for .NET only (estimate; the per-concern scores are in
[stack-ui.md](stack-ui.md), [stack-output-engine-safety.md](stack-output-engine-safety.md),
[stack-audio.md](stack-audio.md) and [stack-api-storage-libraries.md](stack-api-storage-libraries.md);
G loses about 9 points here and some on the control surface and packaging). The mixing cost eats
nearly all of the gain and repeats for every feature that crosses the boundary. The process
boundary matches ASR-11 (a supervisor outside the main process). G is the fallback when both
single-language options fail on their weak side: .NET cannot hold the update rate on Windows or
capture loopback on Linux, and egui cannot carry the 3D view and desktop UX. The spikes measure
both (`TODO.md`). Agreed with the owner, 2026-11-26.

## Open questions

- JavaScript is accepted as a supporting language for details of a web-based UI, such as a 3D view
  (owner, 2026-11-26). Such a module adds the node gates (prettier, eslint, a type check through
  TypeScript or JSDoc) for its folder; 173 open JavaScript rules cover it. Which gates apply to it
  is part of the stack layer (DMX-5).

- Which of Rust, .NET and Kotlin has a UI toolkit for desktop, browser and Android with a 3D view (C-4, C-5, C-6)?
- Does `cargo llvm-cov` run in CI with the Rust profile (unverified in dev-standard)?
- dev-standard follow-ups: `scripts/verify-package` lacks Maven and C++ registries; the `.NET` profile is unverified; no profile supports two languages.
