# Spike DMX-9: 3D room view with picking, Avalonia and egui side by side

Throwaway spike; findings in `FINDINGS.md`, then `docs/research/stack-ui.md` on the DMX-4 branch.

The same scene in both toolkits, drawn with OpenGL 3.3 on the desktop and WebGL2 in the browser:

- a room of 8 m × 6 m × 3 m (floor and four walls, the walls nearest the camera hidden);
- 50 fixtures on a grid at 2.8 m, each a box with a translucent beam cone (16 segments, additive);
- the beams sweep on their own; a click on the floor or a wall sets an aim point, and fixtures
  marked as selected (every second one) aim at it: the ray is cast on the CPU, as the real picking
  would be;
- drag to orbit, wheel to zoom;
- a panel with the frame rate, the frame time (p99 over the last second) and the CPU time spent
  building the geometry per frame, plus a log line every 5 s.

The geometry is rebuilt every frame (50 moving beams), as a live view would.

```text
egui/      eframe 0.37 with glow; `cargo run --release`; browser: see FINDINGS.md
avalonia/  Avalonia 12.2 with OpenGlControlBase; Desktop and Browser heads
```
