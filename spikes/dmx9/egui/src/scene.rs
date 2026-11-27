//! The scene shared by the desktop and the browser build: geometry, camera and picking.
//! Plain maths, no toolkit types. Vertices are x, y, z, r, g, b, a.

pub const ROOM_X: f32 = 4.0; // half width
pub const ROOM_Z: f32 = 3.0; // half depth
pub const ROOM_H: f32 = 3.0;
pub const FIXTURES: usize = 50;
const BEAM_SEGMENTS: usize = 16;
const BEAM_HALF_ANGLE: f32 = 0.14; // radians, about 8 degrees

pub type V3 = [f32; 3];

fn sub(a: V3, b: V3) -> V3 {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}
fn add(a: V3, b: V3) -> V3 {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}
fn mul(a: V3, s: f32) -> V3 {
    [a[0] * s, a[1] * s, a[2] * s]
}
fn dot(a: V3, b: V3) -> f32 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
fn cross(a: V3, b: V3) -> V3 {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}
fn norm(a: V3) -> V3 {
    let l = dot(a, a).sqrt();
    if l > 0.0 {
        mul(a, 1.0 / l)
    } else {
        a
    }
}

pub struct Camera {
    pub yaw: f32,
    pub pitch: f32,
    pub dist: f32,
    pub target: V3,
    pub fov_y: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self { yaw: 0.6, pitch: 0.45, dist: 11.0, target: [0.0, 1.2, 0.0], fov_y: 50f32.to_radians() }
    }
}

impl Camera {
    pub fn eye(&self) -> V3 {
        let (sp, cp) = self.pitch.sin_cos();
        let (sy, cy) = self.yaw.sin_cos();
        add(self.target, mul([cp * sy, sp, cp * cy], self.dist))
    }

    fn basis(&self) -> (V3, V3, V3) {
        let f = norm(sub(self.target, self.eye()));
        let r = norm(cross(f, [0.0, 1.0, 0.0]));
        let u = cross(r, f);
        (f, r, u)
    }

    /// Column-major projection × view, for `glUniformMatrix4fv` without transpose.
    pub fn view_proj(&self, aspect: f32) -> [f32; 16] {
        let (f, r, u) = self.basis();
        let e = self.eye();
        let view = [
            r[0], u[0], -f[0], 0.0, r[1], u[1], -f[1], 0.0, r[2], u[2], -f[2], 0.0, -dot(r, e), -dot(u, e), dot(f, e), 1.0,
        ];
        let (near, far) = (0.1, 100.0);
        let t = 1.0 / (self.fov_y / 2.0).tan();
        let proj = [
            t / aspect, 0.0, 0.0, 0.0, 0.0, t, 0.0, 0.0, 0.0, 0.0, (far + near) / (near - far), -1.0, 0.0, 0.0,
            2.0 * far * near / (near - far), 0.0,
        ];
        let mut out = [0.0; 16];
        for c in 0..4 {
            for row in 0..4 {
                out[c * 4 + row] = (0..4).map(|k| proj[k * 4 + row] * view[c * 4 + k]).sum();
            }
        }
        out
    }

    /// Ray through a point of the view, `nx` and `ny` in -1..1 (y up).
    pub fn ray(&self, nx: f32, ny: f32, aspect: f32) -> (V3, V3) {
        let (f, r, u) = self.basis();
        let th = (self.fov_y / 2.0).tan();
        (self.eye(), norm(add(f, add(mul(r, nx * th * aspect), mul(u, ny * th)))))
    }
}

/// Walls are hidden when the camera is outside them, so the room stays visible.
fn wall_visible(eye: V3, axis: usize, side: f32) -> bool {
    let limit = if axis == 0 { ROOM_X } else { ROOM_Z };
    eye[axis] * side < limit
}

/// Nearest hit on the floor or a visible wall.
pub fn pick(cam: &Camera, nx: f32, ny: f32, aspect: f32) -> Option<V3> {
    let (o, d) = cam.ray(nx, ny, aspect);
    let eye = cam.eye();
    let mut best: Option<(f32, V3)> = None;
    let mut consider = |t: f32| {
        if t > 0.0 && best.is_none_or(|(bt, _)| t < bt) {
            let p = add(o, mul(d, t));
            let eps = 1e-3;
            if p[0].abs() <= ROOM_X + eps && p[2].abs() <= ROOM_Z + eps && p[1] >= -eps && p[1] <= ROOM_H + eps {
                best = Some((t, p));
            }
        }
    };
    if d[1].abs() > 1e-6 {
        consider(-o[1] / d[1]);
    }
    for (axis, limit) in [(0usize, ROOM_X), (2usize, ROOM_Z)] {
        for side in [-1.0f32, 1.0] {
            if wall_visible(eye, axis, side) && d[axis].abs() > 1e-6 {
                consider((side * limit - o[axis]) / d[axis]);
            }
        }
    }
    best.map(|(_, p)| p)
}

pub fn fixture_pos(i: usize) -> V3 {
    let (col, row) = (i % 10, i / 10);
    [-3.6 + col as f32 * 0.8, 2.8, -2.4 + row as f32 * 1.2]
}

fn quad(v: &mut Vec<f32>, a: V3, b: V3, c: V3, d: V3, col: [f32; 4]) {
    for p in [a, b, c, a, c, d] {
        v.extend_from_slice(&p);
        v.extend_from_slice(&col);
    }
}

fn cube(v: &mut Vec<f32>, c: V3, h: f32, col: [f32; 4]) {
    let p = |x: f32, y: f32, z: f32| [c[0] + x * h, c[1] + y * h, c[2] + z * h];
    let shade = |s: f32| [col[0] * s, col[1] * s, col[2] * s, col[3]];
    quad(v, p(-1., 1., -1.), p(1., 1., -1.), p(1., 1., 1.), p(-1., 1., 1.), shade(1.0));
    quad(v, p(-1., -1., -1.), p(-1., -1., 1.), p(1., -1., 1.), p(1., -1., -1.), shade(0.5));
    quad(v, p(-1., -1., 1.), p(-1., 1., 1.), p(1., 1., 1.), p(1., -1., 1.), shade(0.8));
    quad(v, p(-1., -1., -1.), p(1., -1., -1.), p(1., 1., -1.), p(-1., 1., -1.), shade(0.7));
    quad(v, p(1., -1., -1.), p(1., -1., 1.), p(1., 1., 1.), p(1., 1., -1.), shade(0.9));
    quad(v, p(-1., -1., -1.), p(-1., 1., -1.), p(-1., 1., 1.), p(-1., -1., 1.), shade(0.6));
}

/// Length of a beam from `o` along `d` until it leaves the room.
fn beam_length(o: V3, d: V3) -> f32 {
    let mut t = 20.0f32;
    for (axis, limit) in [(0usize, ROOM_X), (1, ROOM_H), (2, ROOM_Z)] {
        if d[axis] > 1e-6 {
            t = t.min((limit - o[axis]) / d[axis]);
        } else if d[axis] < -1e-6 {
            let low = if axis == 1 { 0.0 } else { -limit };
            t = t.min((low - o[axis]) / d[axis]);
        }
    }
    t.max(0.0)
}

fn cone(v: &mut Vec<f32>, o: V3, d: V3, col: [f32; 4]) {
    let len = beam_length(o, d);
    let radius = len * BEAM_HALF_ANGLE.tan();
    let helper = if d[1].abs() < 0.9 { [0.0, 1.0, 0.0] } else { [1.0, 0.0, 0.0] };
    let a = norm(cross(d, helper));
    let b = cross(d, a);
    let end = add(o, mul(d, len));
    let rim = |k: usize| {
        let (s, c) = (k as f32 / BEAM_SEGMENTS as f32 * std::f32::consts::TAU).sin_cos();
        add(end, add(mul(a, c * radius), mul(b, s * radius)))
    };
    let tip = [col[0], col[1], col[2], col[3]];
    let edge = [col[0] * 0.3, col[1] * 0.3, col[2] * 0.3, col[3]];
    for k in 0..BEAM_SEGMENTS {
        for (p, c) in [(o, tip), (rim(k), edge), (rim(k + 1), edge)] {
            v.extend_from_slice(&p);
            v.extend_from_slice(&c);
        }
    }
}

pub fn is_selected(i: usize) -> bool {
    i % 2 == 0
}

/// Fills `v` with the opaque part, then the beams; returns the vertex count of the opaque part.
pub fn build(v: &mut Vec<f32>, cam: &Camera, time: f32, aim: Option<V3>) -> usize {
    v.clear();
    let eye = cam.eye();
    let (x, z, h) = (ROOM_X, ROOM_Z, ROOM_H);
    quad(v, [-x, 0., -z], [-x, 0., z], [x, 0., z], [x, 0., -z], [0.25, 0.25, 0.28, 1.0]);
    let wall = [0.35, 0.33, 0.30, 1.0];
    if wall_visible(eye, 0, -1.0) {
        quad(v, [-x, 0., -z], [-x, h, -z], [-x, h, z], [-x, 0., z], wall);
    }
    if wall_visible(eye, 0, 1.0) {
        quad(v, [x, 0., -z], [x, 0., z], [x, h, z], [x, h, -z], wall);
    }
    let wall2 = [0.30, 0.30, 0.33, 1.0];
    if wall_visible(eye, 2, -1.0) {
        quad(v, [-x, 0., -z], [x, 0., -z], [x, h, -z], [-x, h, -z], wall2);
    }
    if wall_visible(eye, 2, 1.0) {
        quad(v, [-x, 0., z], [-x, h, z], [x, h, z], [x, 0., z], wall2);
    }
    for i in 0..FIXTURES {
        let col = if is_selected(i) { [0.9, 0.7, 0.2, 1.0] } else { [0.6, 0.6, 0.6, 1.0] };
        cube(v, fixture_pos(i), 0.1, col);
    }
    if let Some(p) = aim {
        cube(v, p, 0.06, [1.0, 0.1, 0.1, 1.0]);
    }
    let opaque = v.len() / 7;
    for i in 0..FIXTURES {
        let o = fixture_pos(i);
        let d = match aim {
            Some(p) if is_selected(i) => norm(sub(p, o)),
            _ => {
                let pan = time * 0.5 + i as f32 * 0.3;
                let tilt = 0.5 + 0.3 * (time * 0.7 + i as f32).sin();
                [tilt.sin() * pan.cos(), -tilt.cos(), tilt.sin() * pan.sin()]
            }
        };
        let hue = i as f32 / FIXTURES as f32;
        let col = [0.5 + 0.5 * (hue * 6.28).cos(), 0.5 + 0.5 * (hue * 6.28 + 2.1).cos(), 0.5 + 0.5 * (hue * 6.28 + 4.2).cos(), 0.18];
        cone(v, o, d, col);
    }
    opaque
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn centre_ray_hits_the_floor_below_the_target() {
        let cam = Camera { yaw: 0.0, pitch: 1.2, dist: 5.0, target: [0.0, 0.0, 0.0], ..Default::default() };
        let p = pick(&cam, 0.0, 0.0, 1.0).expect("hit");
        assert!(p[0].abs() < 1e-3 && p[1].abs() < 1e-3 && p[2].abs() < 1e-3, "{p:?}");
    }
}
