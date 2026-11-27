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
  icons are drawn as lines). A one-to-one look across platforms is decided after a Windows run.
- **Visual check (owner, 2026-11-27):** room, beams and picking correct in all four working
  builds; drag, click and wheel work in both Avalonia builds after the input fixes.

## Not measured

- Windows: both desktop builds, and the frame modes.
- Android, multiple windows, docking, text-heavy panels next to the 3D view.
- Picking fixtures (only surfaces are picked), and the beam look at venue scale.
- Avalonia browser with AOT, and Chromium-based browsers.

## Open

- Windows run of both desktop builds (`egui`: `cargo run --release` and `-- --frame custom`;
  Avalonia: `dotnet run -c Release --project avalonia/Desktop`).
- Whether a frame drawn in egui is wanted on every platform, or the system frame.
