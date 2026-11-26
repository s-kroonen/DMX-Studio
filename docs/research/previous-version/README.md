# Previous version: data

Data exported from the earlier [DMX Controller](https://github.com/s-kroonen/DMX-Controller) (v1),
kept as a source for research, spikes and the storage design. DMX Studio does not read these files
as they are; a decision on importing v1 configs belongs to the storage spec (C-9 in
`docs/product/quality-attributes.md`).

| File | Exported | Contents |
|---|---|---|
| `venue-config-2026-11-26.json` | 2026-11-26 from v1 (config export, format version 1) | Room 2.6 × 3.5 × 2 m with a 6-point floor shape; 3 fixtures: Moving Head 1 at address 1 and Moving Head 2 at address 15 (`generic-rgbw-moving-head-14ch`), Beamz at address 29 (`beamz-mhl108-mkii-11ch`), all hanging (pitch 180°); group "All lights"; no safety zones, animations or sound functions; 8 bundled fixture profiles |

The fixtures use DMX channels 1 to 39. Spikes that drive the real interface write test traffic
above channel 39 unless a test needs a fixture to move.
