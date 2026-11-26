# Stack research: UI toolkit, 3D room view and remote clients

- **Concern**: C-4, C-5, C-6 (drivers: QA-3, QA-8, QA-9, ASR-5)
- **Issue**: DMX-4
- **Date**: 2026-11-26
- **Question**: Which UI toolkit gives DMX Studio a desktop host on Windows and Linux, a 3D room view with picking, a browser client and later an Android app, in one language?
- **Time-box**: 1 day
- **Status**: answered
- **Sources**: READMEs and repository metadata on github.com, read 2026-11-26: slint-ui/slint (README, LICENSE.md, `examples/wgpu_texture`, `docs/astro/src/content/docs/guide/platforms/web.mdx`), emilk/egui (README, `examples/custom_3d_glow`), iced-rs/iced, DioxusLabs/dioxus, tauri-apps/tauri, AvaloniaUI/Avalonia (README, `src/Avalonia.OpenGL/Controls/OpenGlControlBase.cs`, WebGL code in `src/Browser`), unoplatform/uno, JetBrains/compose-multiplatform, fabmax/kool; [stack-toolability.md](stack-toolability.md)
- **Verified**: `scripts/verify-package`, 2026-11-26: crates slint 1.19.1 (GPL-3.0-only OR LicenseRef-Slint-Royalty-free-2.0 OR LicenseRef-Slint-Software-3.0, released 2026-11-24); crates egui 0.37.0, eframe 0.37.0, egui-wgpu 0.37.0, egui_kittest 0.37.0 (MIT OR Apache-2.0, 2026-10-15); crates egui_dock 0.22.0 (MIT, 2026-11-20); crates iced 0.14.0, iced_test 0.14.0 (MIT, 2025-12-07); crates dioxus 0.7.10, dioxus-desktop 0.7.10 (MIT OR Apache-2.0, 2026-07-30); crates tauri 2.12.7 (Apache-2.0 OR MIT, 2026-11-18); crates leptos 0.8.23 (MIT, 2026-10-31); crates bevy 0.20.1 (MIT OR Apache-2.0, 2026-10-28); crates bevy_egui 0.43.1 (MIT, 2026-10-10); crates wgpu 30.0.2 (MIT OR Apache-2.0, 2026-11-12); crates three-d 0.19.0 (MIT, 2026-04-17); crates i-slint-backend-testing 1.19.1 (Slint triple licence, 2026-11-24); nuget Avalonia, Avalonia.Desktop, Avalonia.Browser, Avalonia.Android, Avalonia.Headless, Dock.Avalonia 12.2.0 (MIT, 2026-11-23); nuget Uno.WinUI 7.0.65 (Apache-2.0, 2026-11-13); nuget Silk.NET 2.23.0, Silk.NET.OpenGL 2.23.0 (MIT, 2026-01-23); nuget HelixToolkit.SharpDX.Core 2.27.3 (MIT, 2025-08-27; DirectX, Windows only); nuget Avalonia.OpenGL 0.7.0 (stale package; OpenGL lives in the Avalonia core repository). Kotlin: compose-multiplatform v1.12.2 (GitHub release 2026-10-15, Apache-2.0); fabmax/kool (GitHub, pushed 2026-11-25, 599 stars)
- **Re-check by**: 2027-02-24

## Question

The host runs on Windows and Linux with many control windows, docking and keyboard shortcuts; the
room is shown and edited in 3D with picking of fixtures and surfaces; phones use a browser client
first and an Android app later; one repository and, unless another benefit outweighs it, one
language (QA-3, QA-9, ASR-5; owner 2026-11-26). JavaScript is accepted as a supporting language
for details inside a web-based UI, such as a 3D view; a backend in one language with a whole
frontend in another is not (owner, 2026-11-26).

## Previous version (remakes)

v1 used vanilla JavaScript and Three.js in a browser (vendored for offline use), floating windows
with remembered positions, a drag gizmo in 3D, and a separate phone layout from the same frontend.
The 3D view and click-to-aim were central; the frontend had no tests.

## Options

Leaders per language from [stack-toolability.md](stack-toolability.md) (Rust, .NET, Kotlin). Dart
and Rust + Flutter are out on toolability.

| Option | Package(s) verified (version, last release, licence, checked on) | Notes |
|---|---|---|
| R1: Rust, egui/eframe | egui, eframe, egui-wgpu, egui_kittest 0.37.0; egui_dock 0.22.0; three-d 0.19.0 (2026-11-26) | eframe targets Web, Linux, Mac, Windows and Android (README); immediate-mode UI |
| R2: Rust, Slint | slint 1.19.1; i-slint-backend-testing 1.19.1 (2026-11-26) | Desktop, Android, iOS, embedded; web guide in the docs; triple licence |
| R3: Rust, Dioxus or Tauri with a Rust web UI | dioxus 0.7.10; tauri 2.12.7; leptos 0.8.23 (2026-11-26) | UI in a webview (HTML and CSS); web, desktop, Android |
| R4: Rust, iced | iced, iced_test 0.14.0 (2026-11-26) | Windows, macOS, Linux and the Web (README); no Android |
| N1: .NET, Avalonia | Avalonia, Avalonia.Desktop/Browser/Android/Headless, Dock.Avalonia 12.2.0; Silk.NET 2.23.0 (2026-11-26) | Desktop, browser (WebAssembly) and Android packages from one release |
| N2: .NET, Uno Platform | Uno.WinUI 7.0.65 (2026-11-26) | Mobile, WebAssembly, desktop through Skia (README) |
| K1: Kotlin, Compose Multiplatform | compose-multiplatform v1.12.2; kool engine (GitHub, 2026-11-26) | Android, desktop; web in Beta (README) |
| F1: Dart, Flutter (owner asked to compare, 2026-11-26) | flutter_scene 0.24.6 (MIT, 2026-11-24); three_js 0.3.0 (MIT, 2026-05-02); flutter_angle 0.4.2 (MIT, 2026-08-10) (2026-11-26) | Official targets Windows, Linux, web, Android; out on toolability (26/50), compared here on UI only |
| N4: .NET Blazor UI with three.js for 3D | Microsoft.AspNetCore.Components.WebView.Maui 11.0.10 (MIT, 2026-11-11); Photino.Blazor 4.0.13 and Photino.NET 4.0.16 (Apache-2.0, last release 2025-01-23); bunit 2.15.2 (MIT, 2026-11-10); Microsoft.Playwright 1.65.1 (MIT, 2026-11-18); npm three 0.187.0, @types/three 0.187.0 (MIT, 2026-10-30) (2026-11-26) | One Razor UI served by the host to the browser; a desktop window through Photino or MAUI Blazor Hybrid; 3D in JavaScript, as in v1 |
| N3: .NET MAUI (owner asked to compare, 2026-11-26) | Microsoft.Maui.Controls 11.0.10 (MIT, 2026-11-11); Microsoft.Maui.Platforms.Linux.Gtk4 (prerelease only, registry gives no stable version); SkiaSharp.Views.Maui.Controls 4.157.0 (MIT, 2026-11-18); Microsoft.AspNetCore.Components.WebAssembly 11.0.0 (MIT, 2026-11-10); HelixToolkit.Maui and Evergine.MAUI not found (2026-11-26) | Android, iOS, Windows, macOS (README); Linux through an experimental backend; web through a separate Blazor UI |

## Criteria and weights (agreed with the owner before scoring)

Agreed 2026-11-26; the owner raised desktop UX to 3.

| Criterion | Weight | Source (QA-…) |
|---|---|---|
| One language covers Windows, Linux, browser and Android | 3 | QA-3, QA-9, ASR-5 |
| 3D room view with picking (in the toolkit or an embeddable engine) | 3 | QA-8, vision |
| Desktop UX: many floating windows, docking, shortcuts | 3 | vision "Familiarity", owner |
| Maturity and maintenance (releases, users, docs) | 2 | QA-9 |
| Licence fits (no copyleft, no paid runtime licence) | 2 | `scripts/check licenses` |
| UI testability (headless tests for the gates, e2e) | 2 | ASR-10, constitution III |

## Scores

Scores 1–5; weighted total out of 75.

| Option | One language (3) | 3D (3) | Desktop UX (3) | Maturity (2) | Licence (2) | Testability (2) | Weighted total | Reason |
|---|---|---|---|---|---|---|---|---|
| N1: Avalonia | 5 | 3 | 5 | 5 | 5 | 5 | **69** | All four targets from one release; docking (Dock.Avalonia), multi-window, XAML; headless test package; 3D only through the OpenGL control plus a library such as Silk.NET, no scene graph |
| R1: egui/eframe | 4 | 5 | 4 | 3 | 5 | 4 | **63** | Same UI on desktop and in the browser (WebAssembly); 3D through wgpu or glow paint callbacks, `custom_3d_glow` example, three-d and Bevy integrations; built-in floating windows, egui_dock; 0.x releases with breaking changes; Android less proven (ASSUMPTION) |
| N2: Uno | 5 | 2 | 4 | 4 | 5 | 3 | **57** | All targets; Skia rendering, no 3D path found; testability not verified (ASSUMPTION) |
| K1: Compose | 4 | 3 | 3 | 4 | 5 | 4 | **56** | Web in Beta; 3D through a small engine (kool, 599 stars); no docking found |
| R2: Slint | 4 | 3 | 3 | 4 | 3 | 4 | **52** | Stable 1.x; 3D as a wgpu texture (`examples/wgpu_texture`); no docking found; royalty-free licence has conditions, GPLv3 otherwise |
| R3: Dioxus / Tauri | 4 | 4 | 3 | 3 | 5 | 2 | **53** | Webview UI; 3D through three.js in the webview (JavaScript accepted as a supporting language); headless testing not found (ASSUMPTION) |
| N4: Blazor + three.js | 4 | 5 | 3 | 3 | 5 | 5 | **62** | One Razor UI for desktop window, browser and Android (MAUI Blazor Hybrid); three.js is the 3D approach v1 proved; floating windows and docking built by hand as in v1 (ASSUMPTION: no docking library verified); desktop shell weak: Photino last released 2025-01-23, MAUI on Linux experimental; bUnit and Playwright for tests |
| R4: iced | 3 | 3 | 2 | 3 | 5 | 3 | **46** | No Android; last release 2025-12-07; no floating windows or docking found (ASSUMPTION) |
| F1: Flutter | 5 | 4 | 3 | 5 | 5 | 5 | **66** | Official Windows, Linux, web and Android targets; 3D through flutter_scene on every target (WebGL2 on the web), but on Flutter GPU, a preview API (ASSUMPTION), and one main maintainer; desktop multi-window and docking not verified (ASSUMPTION); widget tests run headless (ASSUMPTION) |
| N3: MAUI (XAML) | 2 | 2 | 3 | 3 | 5 | 3 | **43** | Linux only through an experimental backend "not officially supported by Microsoft"; the web needs a second UI in Blazor (3D there through three.js); no 3D package for MAUI XAML found, SkiaSharp is 2D. With the UI in Blazor Hybrid instead of XAML it becomes N4 |

## Toolability (dev-standard gates)

Carried from [stack-toolability.md](stack-toolability.md): .NET 42 and Rust 43 out of 50. The .NET
profile is rendered only and unverified in dev-standard; Avalonia.Headless gives the UI tests a
place in `TEST_CMD`. egui_kittest does the same for egui.

## Evidence

- Avalonia publishes Avalonia.Desktop, Avalonia.Browser and Avalonia.Android at the same version (12.2.0, 2026-11-23), with Avalonia.Headless for tests and Dock.Avalonia for docking. — [verify-package nuget, 2026-11-26] (high)
- Avalonia's core contains `OpenGlControlBase` (`src/Avalonia.OpenGL/Controls/`) and its browser backend contains WebGL code (9 code hits in `src/Browser`). — [github.com/AvaloniaUI/Avalonia, code search, 2026-11-26] (medium: presence of code, not a working 3D scene)
- HelixToolkit.SharpDX targets DirectX and so Windows only; it does not cover Linux. — [verify-package nuget; package name, 2026-11-26] (medium)
- eframe "supports writing apps for Web, Linux, Mac, Windows, and Android"; the web demo runs in any browser with WebAssembly and WebGL. — [github.com/emilk/egui README, 2026-11-26] (high)
- egui ships a `custom_3d_glow` example of a 3D scene inside the UI; bevy_egui and three-d integrate with egui. — [github.com/emilk/egui, verify-package crates, 2026-11-26] (high)
- Slint renders a wgpu scene as a texture (`examples/wgpu_texture`); its royalty-free licence covers proprietary desktop, mobile and web apps, excludes embedded. — [github.com/slint-ui/slint, 2026-11-26] (high)
- iced lists Windows, macOS, Linux and the Web; no Android. — [github.com/iced-rs/iced README, 2026-11-26] (high)
- Compose Multiplatform lists Android, desktop and iOS, with Web in Beta. — [github.com/JetBrains/compose-multiplatform README, 2026-11-26] (high)
- Uno renders desktop through Skia and targets WebAssembly and mobile. — [github.com/unoplatform/uno README, 2026-11-26] (high)
- flutter_scene: "A complete 3D game engine for Flutter, built on Flutter GPU"; platforms "iOS, Android, macOS, Windows, Linux, and the web, where a built-in WebGL2 backend stands in for Impeller"; 1,047 stars, pushed 2026-11-26. — [github.com/bdero/flutter_scene README, 2026-11-26] (high for the claim, medium for maturity)
- three_js (Dart port of three.js) lists Android, Windows, web (WebGL2, with a script loaded from a CDN) and Linux (Ubuntu tested); 91 stars. — [github.com/Knightro63/three_js README, 2026-11-26] (high)
- .NET MAUI's README names Android, iOS, Windows and macOS. — [github.com/dotnet/maui README, 2026-11-26] (high)
- The MAUI Linux GTK4 backend comes from `dotnet/maui-labs` (243 stars): "This backend is experimental and will change between releases. It is not officially supported by Microsoft." It needs .NET 10, GTK 4 and WebKitGTK 6 for Blazor; the page lists no supported controls and says nothing on custom drawing, OpenGL, multiple windows or shortcuts. — [`learn.microsoft.com/en-us/dotnet/maui/developer-tools/platform-backends/linux-gtk4`, updated 2026-10-13, read 2026-11-26] (high)
- A MAUI host plus a Blazor web client means two UI codebases (XAML and Razor) unless the host uses Blazor Hybrid; 3D in Blazor runs through JavaScript interop to WebGL. — ASSUMPTION (medium)
- Photino wraps the system webview ("WebKit-based browser control for Windows, macOS and Linux"), Windows 10+ and Linux (tested with Ubuntu 18.04+); photino.NET 1,361 stars, last push 2026-03-26; last NuGet release 2025-01-23. — [github.com/tryphotino/photino.NET README, verify-package nuget, 2026-11-26] (high)
- three.js: 119,467 stars, pushed 2026-11-26; npm three 0.187.0 (MIT, 2026-10-30). — [github.com/mrdoob/three.js, verify-package npm, 2026-11-26] (high)
- A browser client built from the same Avalonia or egui UI downloads as a WebAssembly bundle of several megabytes; over the local network this is acceptable. — ASSUMPTION (medium)

## Evidence against the leading option

Avalonia has no 3D scene graph: the room view, beams and picking are built on an OpenGL control with
a low-level library (Silk.NET), and the same code has to run through WebGL in the browser client.
v1's 3D view took Three.js for granted; that convenience is gone. egui is the stronger 3D option
(wgpu, three-d and Bevy integrations) and nearly as strong on one-language coverage. If the 3D spike
shows the Avalonia path is slow to build or fails in the browser, R1 (Rust, egui) leads instead.
The .NET profile of dev-standard has never run end to end, so the first gate setup is unproven.
Avalonia's immediate competitor score gap (69 against 63) rests on desktop UX and maturity, both
judged from documentation, not use.

## Spike needed?

Yes. The existing spike item "3D room view with picking in the leading toolkit" (`TODO.md`) takes
this form: Avalonia with an OpenGL control and Silk.NET draws a room polygon with walls, three
fixtures with beams, and picks a point on a surface, on Linux desktop and in the browser
(Avalonia.Browser). The same scene in egui with three-d or wgpu is the fallback measure. Time-box:
2 days. Pass: both targets render and pick at ≥ 30 fps with 50 fixtures on the development machine.

## Recommendation (confidence: medium)

**N1: .NET with Avalonia** (69) for host, browser client and Android, in one language, subject to
the 3D spike. Close behind: **F1: Flutter** (66, but 26/50 on toolability), **R1: Rust with egui**
(63) and **N4: Blazor with three.js** (62, the 3D approach v1 proved, weak desktop shell). MAUI
with XAML (43) is out: an experimental Linux backend, a second web UI and no 3D path. The owner
dropped Dart/Flutter and MAUI (2026-11-26): Dart is weak on every concern beyond the UI
([stack-output-engine-safety.md](stack-output-engine-safety.md),
[stack-audio.md](stack-audio.md), [stack-toolability.md](stack-toolability.md)). The spike
decides between the remaining leaders.

## Open questions

- Does a 3D room scene with picking run in Avalonia on Linux and in the browser (spike)?
- Does the dev-standard .NET profile pass end to end (playbook step 13)?
- Android: Avalonia.Android maturity for the later app; not needed for the first release.
- Size and start time of the browser client on a phone over the local network.
