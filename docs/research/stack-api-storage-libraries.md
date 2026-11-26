# Stack research: control surface, storage, maths, packaging and later libraries

- **Concern**: C-7, C-9, C-10, C-11, C-13 (drivers: QA-2, QA-6, QA-7, QA-8, QA-9, ASR-6, ASR-9)
- **Issue**: DMX-4
- **Date**: 2026-11-26
- **Question**: How do C#/.NET and Rust compare on the local API with OpenAPI, live updates, pairing and MCP; on venue storage; on the maths for aiming and calibration; on packaging; and on the libraries later slices need (scripting, MIDI, OSC, Android USB host)?
- **Time-box**: 1 day
- **Status**: answered
- **Sources**: github.com/modelcontextprotocol/csharp-sdk and github.com/modelcontextprotocol/rust-sdk repository metadata (read 2026-11-26); github.com/velopack/velopack README (read 2026-11-26); [stack-toolability.md](stack-toolability.md); [stack-ui.md](stack-ui.md)
- **Verified**: `scripts/verify-package`, 2026-11-26. .NET (nuget): Microsoft.AspNetCore.OpenApi 11.0.0 (MIT, 2026-11-10); Swashbuckle.AspNetCore 11.0.1 (MIT, 2026-11-12); Scalar.AspNetCore 3.3.0 (MIT, 2026-11-26); Microsoft.AspNetCore.SignalR.Client 11.0.0 (MIT, 2026-11-10); ModelContextProtocol and ModelContextProtocol.AspNetCore 2.4.0 (Apache-2.0, 2026-11-07); JsonSchema.Net 9.7.0 (licence not stated, 2026-11-13); YamlDotNet 18.1.0 (MIT, 2026-06-26); Tomlyn 2.11.1 (BSD-2-Clause, 2026-11-08); MathNet.Numerics 5.0.0 (MIT, 2022-04-03); Jint 4.18.1 (BSD-2-Clause, 2026-11-23); NLua 1.7.9 (licence not stated, 2026-05-01); MoonSharp 2.0.0 (licence not stated, 2016-10-14); Microsoft.ClearScript.V8 7.5.1.1 (licence not stated, 2026-09-04); Melanchall.DryWetMidi 8.0.3 (MIT, 2025-12-15); Rug.Osc 1.2.5 (licence not stated, 2014-01-04); CoreOSC 1.0.0 (licence not stated, 2019-12-30); Velopack 1.2.161 (MIT, 2026-09-29); WixToolset.Sdk 7.0.1 (licence not stated, 2026-10-26). Rust (crates): axum 0.8.9 (MIT, 2026-04-14); utoipa 6.0.0 and utoipa-scalar 0.4.0 (MIT OR Apache-2.0, 2026-09-22); aide 0.15.1 (MIT OR Apache-2.0, 2025-08-19); tower-http 0.7.1 (MIT, 2026-08-31); rmcp 3.12.0 (Apache-2.0, 2026-11-26); serde 1.0.229 (MIT OR Apache-2.0, 2026-07-18); serde_json 1.0.154 (MIT OR Apache-2.0, 2026-10-11); schemars 1.2.3 (MIT, 2026-10-12); jsonschema 0.63.2 (MIT, 2026-11-26); toml 1.2.3 (MIT OR Apache-2.0, 2026-11-25); serde_yaml 0.9.34+deprecated (2024-03-25); serde_yml 0.0.13 (MIT OR Apache-2.0, 2026-05-27); nalgebra 0.35.0 (Apache-2.0, 2026-05-24); glam 0.34.3 (MIT OR Apache-2.0, 2026-11-08); levenberg-marquardt 0.15.0 (MIT, 2025-08-03); argmin 0.11.0 (MIT OR Apache-2.0, 2025-09-28); rhai 1.27.0 (MIT OR Apache-2.0, 2026-10-21); mlua 0.12.3 (MIT, 2026-11-22); boa_engine 0.22.0 (Unlicense OR MIT, 2026-08-28); rquickjs 0.14.0 (MIT, 2026-09-18); midir 0.11.2 (MIT, 2026-11-25); rosc 0.11.4 (MIT/Apache-2.0, 2025-03-23); jni 0.22.4 (MIT OR Apache-2.0, 2026-03-16); android-activity 0.6.1 (MIT OR Apache-2.0, 2026-03-24); velopack 1.2.161 (MIT, 2026-09-29); cargo-dist 0.34.0 (MIT OR Apache-2.0, 2026-11-02); cargo-packager 0.11.8 (Apache-2.0 OR MIT, 2025-11-27); cargo-wix 0.3.9 (MIT/Apache-2.0, 2025-03-13)
- **Re-check by**: 2027-02-24

## Question

Five concerns that depend on libraries more than on the language core. Dart and MAUI are out
(owner, 2026-11-26), so the comparison is .NET against Rust.

## Previous version (remakes)

- C-7: FastAPI REST and WebSocket; edit/show mode enforced on the server; no authentication or pairing; LAN by a start flag.
- C-9: JSON files per concern in a data folder; named saved rooms; bundled profiles shadowed by user copies.
- C-10: a pure, unit-tested aiming function and a least-squares solver for mounting from marks.
- C-11: a PyInstaller one-file exe for Windows; packaged data in a per-user folder; a second start opens a window onto the running server.
- C-13: none of these libraries existed.

## Options

| Option | Package(s) verified | Notes |
|---|---|---|
| .NET | see **Verified**: ASP.NET Core OpenAPI, Scalar, SignalR, ModelContextProtocol, JsonSchema.Net, Jint, DryWetMidi, Velopack | Most of C-7 ships in ASP.NET Core |
| Rust | see **Verified**: axum, utoipa, tower-http, rmcp, serde, schemars, nalgebra, levenberg-marquardt, rhai, midir, rosc, velopack, cargo-dist | Assembled from crates |

## Criteria and weights (agreed with the owner before scoring)

Agreed 2026-11-26; the owner raised live updates to 3.

| Concern | Criterion | Weight | Source (QA-…) |
|---|---|---|---|
| C-7 | OpenAPI description generated from the code | 3 | vision (developer API) |
| C-7 | Authentication, pairing, origin and host checks in the web framework | 3 | QA-7, ASR-6 |
| C-7 | Live updates (WebSocket or similar) | 3 | owner |
| C-7 | Official or mature MCP server SDK | 2 | vision (agent support) |
| C-9 | Human-readable versioned format with migrations | 3 | QA-9 |
| C-9 | Strict validation on load | 3 | constitution IV |
| C-9 | Library maturity | 2 | QA-9 |
| C-9 | Diff-friendly in git | 1 | QA-9 |
| C-10, C-13 | Linear algebra and non-linear least squares | 3 | QA-2 (aiming), vision (calibration) |
| C-10, C-13 | Embeddable scripting engine with a sandbox | 3 | vision (scripting) |
| C-10, C-13 | Android USB host access | 2 | vision (nice to have) |
| C-10, C-13 | MIDI | 1 | vision (nice to have) |
| C-10, C-13 | OSC | 1 | vision (nice to have) |
| C-11 | Windows installer and portable build from one release | 3 | QA-6, ASR-9 |
| C-11 | Linux package | 2 | QA-6 |
| C-11 | Signed, reproducible CI builds | 2 | dev-standard `release.yml` |
| C-11 | Android packaging (later) | 1 | QA-3 |

## Scores

Scores 1–5; sub-scores follow the criteria order above.

| Concern (max) | .NET | Rust | Reason |
|---|---|---|---|
| C-7 (55) | **55** (5, 5, 5, 5) | **43** (4, 3, 4, 5) | ASP.NET Core ships OpenAPI generation, authentication, host filtering, CORS and SignalR; the official C# MCP SDK has an ASP.NET Core package. Rust assembles them: utoipa for OpenAPI, tower-http for CORS, authentication and pairing from parts (ASSUMPTION: no single framework package); rmcp is the official Rust MCP SDK |
| C-9 (45) | **38** (4, 4, 5, 4) | **41** (4, 5, 5, 4) | Both write JSON, TOML or YAML and need hand-written migrations; serde's typed, strict deserialisation (unknown fields rejected) plus schemars and jsonschema give the stricter load; serde_yaml is deprecated, so YAML is weaker in Rust |
| C-10, C-13 (50) | **38** (3, 4, 5, 5, 2) | **44** (5, 5, 3, 4, 4) | .NET: System.Numerics vectors built in, MathNet last released 2022; Jint active; .NET for Android reaches the Android USB API directly (ASSUMPTION); DryWetMidi; the OSC packages date from 2014 and 2019. Rust: nalgebra, glam, levenberg-marquardt and argmin; rhai is built for sandboxed embedding (ASSUMPTION from its purpose); Android USB through JNI; midir; rosc |
| C-11 (40) | **36** (5, 4, 4, 5) | **36** (5, 5, 4, 3) | Velopack (MIT, both languages) builds an installer, updates and "self-updating portable package" for Windows, macOS and Linux; Rust adds cargo-dist and cargo-packager for Linux packages (ASSUMPTION: deb or AppImage); Android packaging is native to .NET and less proven for Rust UI toolkits |

## Toolability (dev-standard gates)

As in [stack-toolability.md](stack-toolability.md). The dev-standard custom rules target ASP.NET
(C#) and axum (Rust), the two web frameworks in this note; both measured 6/6 and 7/7 on the canary.
WiX licence terms are not stated by the registry; Velopack covers the same need under MIT.

## Evidence

- The official MCP SDKs: `modelcontextprotocol/csharp-sdk` (4,656 stars, pushed 2026-11-24) and `modelcontextprotocol/rust-sdk` (4,148 stars, pushed 2026-11-26). — [GitHub, read 2026-11-26] (high)
- Velopack turns compiler output into an installer, updates, delta packages and a self-updating portable package, supports Windows, macOS and Linux, and has APIs for C#, C++, JS and Rust; MIT, 2,492 stars. — [velopack README, read 2026-11-26] (high)
- serde_yaml is published as `0.9.34+deprecated`. — [verify-package crates, 2026-11-26] (high)
- MathNet.Numerics had its last release on 2022-04-03; Rust's nalgebra, glam and levenberg-marquardt are released in 2025 and 2026. — [verify-package, 2026-11-26] (high)
- The .NET OSC packages found had their last releases in 2014 and 2019; rosc in 2025. — [verify-package, 2026-11-26] (high)
- MoonSharp had its last release in 2016; Jint, NLua and ClearScript are active. — [verify-package nuget, 2026-11-26] (high)

## Evidence against the leading option

The note splits: .NET wins C-7 clearly (55 against 43), Rust wins C-9 and C-10/C-13 narrowly, C-11
ties. The control surface carries constitution VIII, so the .NET lead there counts more than the
weights show: authentication, origin checks and pairing come from one maintained framework
instead of assembled crates. The Rust lead on C-10/C-13 rests on nice-to-have items (OSC, MIDI)
and on scripting, which comes in a later slice.

## Spike needed?

No. The library facts answer these concerns; the spikes in `TODO.md` stay on the dongle, the 3D
view and audio.

## Recommendation (confidence: medium)

No single winner: **.NET for the control surface** (ASP.NET Core with OpenAPI, SignalR and the
official MCP SDK), **Rust for maths and later libraries**, a tie on packaging (Velopack serves
both). The stack decision weighs this against the other notes.

## Open questions

- Pairing flow (one-time code on the host) in either framework: built from parts in both; detail for the control-surface spec.
- Venue file format (JSON or TOML) and its migration rule: a decision for the first storage spec, not the stack.
- Scripting language choice (Lua, JavaScript, Rhai) belongs to the scripting slice.
