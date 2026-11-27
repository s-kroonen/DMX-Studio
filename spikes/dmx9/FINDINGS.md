# Spike DMX-9: 3D room view with picking, Avalonia and egui side by side

Throwaway spike; findings move to `docs/research/stack-ui.md` on the DMX-4 branch. The scene is
described in `README.md`: 50 fixtures with moving beams, about 4,200 vertices rebuilt every frame,
surface picking on the CPU.

## Setup

- Linux: Fedora Linux 44, GNOME on Wayland, monitor refresh about 165 Hz (both views are
  vsync-capped there), Firefox for the browser runs.
- egui 0.37.0 with eframe and glow (OpenGL 3.3 desktop, WebGL2 browser); wasm-bindgen 0.2.130.
- Avalonia 12.2.0 with `OpenGlControlBase` (desktop), .NET 10.0.13; browser head with
  Avalonia.Browser and the `wasm-tools` workload.

## Results

| Build | Renders | fps | Frame p99 | Geometry build | GC | Download |
|---|---|---|---|---|---|---|
| egui, Linux desktop | yes | 163–165 | 6.8–7.5 ms | 0.06 ms p99 | — | — |
| Avalonia, Linux desktop | yes | 177–180 | 5.9–6.2 ms | 0.05–0.14 ms p99 | gen0 about 1 per 5 s, 0 in a later run | — |
| egui, Firefox | yes | 164–172 | 7–10 ms | under 1 ms (1 ms timer) | — | 4.9 MB raw, about 1.1 MB compressed |
| Avalonia `OpenGlControlBase`, Firefox | no | — | — | — | — | 19.3 MB raw, 5.3 MB Brotli |
| Avalonia with a WebGL canvas, Firefox | yes | 164 after start | 8–10 ms | 2 ms p99 | gen0 about 1 per s | as above |

Both toolkits clear the target (30 fps with 50 fixtures) on the desktop and in the browser by a
wide margin; the geometry rebuild of all beams costs well under 1 ms per frame natively.

## Findings

- **Avalonia's 3D control does not run in the browser.** The browser platform has no OpenGL
  context factory; the log reads "Unable to initialize OpenGL: current platform doesn't support
  OpenGL interop with the compositor" and the control stays black. The working browser path puts
  an HTML canvas in the layout (`NativeControlHost`) and draws it with WebGL2 from a JavaScript
  module (`Browser/wwwroot/room.js`, about 90 lines), with scene, camera and picking in C#. The 3D
  view is then two implementations (OpenGL on the desktop, WebGL plus JavaScript in the browser),
  the same split as the Blazor and three.js option.
- **Avalonia input around GL content.** On the desktop the GL output is a composition visual that
  pointer hit tests skip: a transparent panel around the control takes the input. In the browser
  Avalonia's input layer captures the pointer on press, so the canvas never sees a drag; input
  goes through Avalonia's pointer events and the canvas has `pointer-events: none`.
- **egui: one implementation.** The same Rust code (scene, glow draw calls, input) runs on the
  desktop and in the browser; the only browser-specific part is the start function. The paint
  callback shares GL state with egui: the clear needs a scissor to the view rectangle, and the
  blend state is handed back after drawing.
- **Browser load.** The egui page loads faster than the Avalonia page (owner, 2026-11-27); the
  Avalonia download is about five times larger compressed. .NET in the browser runs interpreted by
  default (2 ms geometry build against 0.03 ms for egui); AOT compilation is not measured.
- **Browser GC.** The Avalonia browser build collects gen0 about once a second (vertex buffer
  copied to JavaScript each frame, ASSUMPTION as the cause); no frame above 34 ms after start.
- **Window frame on GNOME Wayland.** GNOME leaves the frame to the application. Avalonia runs
  through XWayland and gets GNOME's frame. egui on Wayland draws winit's plain fallback frame;
  winit's Adwaita frame looks close to GNOME but does not match (owner). Two modes in the spike:
  `--frame system` runs egui through XWayland for GNOME's own frame (same trade as Avalonia: no
  native Wayland, scaling through XWayland), `--frame custom` draws title bar, buttons and resize
  edges in egui (about 100 lines; the default egui fonts have no window-control glyphs, so the
  icons are drawn as lines; they look right, owner 2026-11-27). A one-to-one look across platforms is decided after a Windows run.
- **Visual check (owner, 2026-11-27):** room, beams and picking correct in all four working
  builds; drag, click and wheel work in both Avalonia builds after the input fixes.

## Windows

- Windows 11 Pro 10.0.26300 (26H2), NVIDIA GeForce RTX 4060 (driver 32.0.16.1088), monitor
  1920 × 1080 at 60 Hz; the machine also has an AMD Radeon iGPU and a Parsec virtual display
  adapter.
- egui 0.37.0 with glow through the driver's WGL, Rust 1.100.0 (MSVC); Avalonia 12.2.0 through
  ANGLE (`OpenGLES 3.0` reported), .NET 10.0.15.

| Build | Renders | fps | Frame p99 | Geometry build | GC |
|---|---|---|---|---|---|
| egui, system frame | yes | 59.6–60.2 | 16.8–17.8 ms, one window 30.5 ms | 0.03–0.04 ms p99 | — |
| egui, `--frame custom` | yes | 60.0–60.4, one window 41.9 | 16.8–18.8 ms, two windows about 34 ms | 0.03–0.04 ms p99 | — |
| Avalonia, system frame | yes | 60.0, 56.7 in the first 5 s | 16.9–17.6 ms | 0.05–0.07 ms p99, 0.43 ms in the first 5 s | gen0 2 in 55 s |

- Both builds start on Windows: egui on WGL, Avalonia on ANGLE (Direct3D underneath). Both are
  vsync-capped at the 60 Hz monitor (165 Hz on the Linux machine).
- Avalonia's first 5 s hold one 233 ms frame (start-up). The egui custom-frame run holds one
  1570 ms frame in the 41.9 fps window (ASSUMPTION: the minimize check; a minimized window pauses
  rendering).
- Visual check (owner, 2026-11-27): room, beams and picking correct in both builds; drag orbits,
  wheel zooms, a click on the floor or a wall aims the gold fixtures. egui and Avalonia with the
  system frame look like a normal Windows 11 window and snap (screen edge, Win+arrow keys).
- egui `--frame custom` (owner, 2026-11-27): drag moves, double click maximizes and restores, the
  three buttons work, resizing from edges and corners works, Windows snapping works. The drawn
  title bar looks the same as on Linux and not like a Windows 11 title bar; accepted, since the
  aim is one look on every platform, with custom icons later.

## Not measured

- Android, multiple windows, docking, text-heavy panels next to the 3D view.
- Picking fixtures (only surfaces are picked), and the beam look at venue scale.
- Avalonia browser with AOT, and Chromium-based browsers.

## Open

- Whether a frame drawn in egui is wanted on every platform, or the system frame.
