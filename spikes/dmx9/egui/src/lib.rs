//! Spike DMX-9: the room view in egui, drawn with glow (OpenGL 3.3 / WebGL2). Throwaway code.

pub mod frame;
pub mod scene;

use eframe::egui::mutex::Mutex;
use eframe::{egui, egui_glow, glow};
use scene::Camera;
use std::sync::Arc;
use web_time::Instant;

pub fn say(msg: &str) {
    #[cfg(target_arch = "wasm32")]
    log::info!("{msg}");
    #[cfg(not(target_arch = "wasm32"))]
    println!("{msg}");
}

struct Renderer {
    program: glow::Program,
    vao: glow::VertexArray,
    vbo: glow::Buffer,
    mvp: Option<glow::UniformLocation>,
}

impl Renderer {
    fn new(gl: &glow::Context) -> Self {
        use glow::HasContext as _;
        let version = if cfg!(target_arch = "wasm32") { "#version 300 es" } else { "#version 330" };
        let vs = "layout(location = 0) in vec3 a_pos;
                  layout(location = 1) in vec4 a_col;
                  uniform mat4 u_mvp;
                  out vec4 v_col;
                  void main() { v_col = a_col; gl_Position = u_mvp * vec4(a_pos, 1.0); }";
        let fs = "precision mediump float;
                  in vec4 v_col;
                  out vec4 o_col;
                  void main() { o_col = v_col; }";
        unsafe {
            let program = gl.create_program().expect("program");
            let mut shaders = Vec::new();
            for (kind, src) in [(glow::VERTEX_SHADER, vs), (glow::FRAGMENT_SHADER, fs)] {
                let s = gl.create_shader(kind).expect("shader");
                gl.shader_source(s, &format!("{version}\n{src}"));
                gl.compile_shader(s);
                assert!(gl.get_shader_compile_status(s), "{}", gl.get_shader_info_log(s));
                gl.attach_shader(program, s);
                shaders.push(s);
            }
            gl.link_program(program);
            assert!(gl.get_program_link_status(program), "{}", gl.get_program_info_log(program));
            for s in shaders {
                gl.detach_shader(program, s);
                gl.delete_shader(s);
            }
            let vao = gl.create_vertex_array().expect("vao");
            let vbo = gl.create_buffer().expect("vbo");
            gl.bind_vertex_array(Some(vao));
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
            gl.enable_vertex_attrib_array(0);
            gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, 28, 0);
            gl.enable_vertex_attrib_array(1);
            gl.vertex_attrib_pointer_f32(1, 4, glow::FLOAT, false, 28, 12);
            gl.bind_vertex_array(None);
            let mvp = gl.get_uniform_location(program, "u_mvp");
            Self { program, vao, vbo, mvp }
        }
    }

    fn paint(&self, gl: &glow::Context, verts: &[f32], opaque: usize, mvp: &[f32; 16]) {
        use glow::HasContext as _;
        unsafe {
            let bytes = std::slice::from_raw_parts(verts.as_ptr() as *const u8, std::mem::size_of_val(verts));
            gl.clear_color(0.05, 0.05, 0.07, 1.0);
            gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
            gl.use_program(Some(self.program));
            gl.uniform_matrix_4_f32_slice(self.mvp.as_ref(), false, mvp);
            gl.bind_vertex_array(Some(self.vao));
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.vbo));
            gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, bytes, glow::STREAM_DRAW);
            gl.enable(glow::DEPTH_TEST);
            gl.depth_func(glow::LEQUAL);
            gl.disable(glow::BLEND);
            gl.draw_arrays(glow::TRIANGLES, 0, opaque as i32);
            gl.enable(glow::BLEND);
            gl.blend_func(glow::SRC_ALPHA, glow::ONE);
            gl.depth_mask(false);
            gl.draw_arrays(glow::TRIANGLES, opaque as i32, (verts.len() / 7 - opaque) as i32);
            // Hand the state back as egui expects it.
            gl.depth_mask(true);
            gl.disable(glow::DEPTH_TEST);
            gl.blend_func_separate(glow::ONE, glow::ONE_MINUS_SRC_ALPHA, glow::ONE_MINUS_DST_ALPHA, glow::ONE);
            gl.bind_vertex_array(None);
        }
    }
}

struct Frame {
    verts: Vec<f32>,
    opaque: usize,
}

pub struct RoomApp {
    /// Draw the window frame in egui (`--frame custom`) instead of leaving it to the system.
    pub custom_frame: bool,
    renderer: Arc<Mutex<Renderer>>,
    frame: Arc<Mutex<Frame>>,
    cam: Camera,
    aim: Option<scene::V3>,
    start: Instant,
    last: Instant,
    frame_ms: Vec<f64>,
    build_ms: Vec<f64>,
    window_start: Instant,
    shown: String,
    log_start: Instant,
    log_frames: Vec<f64>,
    log_build: Vec<f64>,
}

fn p99(v: &mut [f64]) -> f64 {
    if v.is_empty() {
        return 0.0;
    }
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    v[((v.len() - 1) as f64 * 0.99).round() as usize]
}

impl RoomApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let gl = cc.gl.as_ref().expect("glow backend");
        let now = Instant::now();
        Self {
            custom_frame: false,
            renderer: Arc::new(Mutex::new(Renderer::new(gl))),
            frame: Arc::new(Mutex::new(Frame { verts: Vec::new(), opaque: 0 })),
            cam: Camera::default(),
            aim: None,
            start: now,
            last: now,
            frame_ms: Vec::new(),
            build_ms: Vec::new(),
            window_start: now,
            shown: String::new(),
            log_start: now,
            log_frames: Vec::new(),
            log_build: Vec::new(),
        }
    }

    fn stats(&mut self) {
        let now = Instant::now();
        let dt = (now - self.last).as_secs_f64() * 1000.0;
        self.last = now;
        self.frame_ms.push(dt);
        self.log_frames.push(dt);
        if (now - self.window_start).as_secs_f64() >= 1.0 {
            let n = self.frame_ms.len();
            let secs = (now - self.window_start).as_secs_f64();
            let build_mean = self.build_ms.iter().sum::<f64>() / self.build_ms.len().max(1) as f64;
            self.shown = format!(
                "{:.1} fps, frame p99 {:.1} ms, geometry build {:.2} ms per frame, {} vertices",
                n as f64 / secs,
                p99(&mut self.frame_ms),
                build_mean,
                self.frame.lock().verts.len() / 7
            );
            self.frame_ms.clear();
            self.build_ms.clear();
            self.window_start = now;
        }
        if (now - self.log_start).as_secs_f64() >= 5.0 {
            let secs = (now - self.log_start).as_secs_f64();
            let n = self.log_frames.len();
            let max = self.log_frames.iter().cloned().fold(0.0, f64::max);
            let build_max = self.log_build.iter().cloned().fold(0.0, f64::max);
            say(&format!(
                "egui 5 s: {:.1} fps, frame p99 {:.1} ms, max {:.1} ms, build p99 {:.2} ms, max {:.2} ms",
                n as f64 / secs,
                p99(&mut self.log_frames),
                max,
                p99(&mut self.log_build),
                build_max
            ));
            self.log_frames.clear();
            self.log_build.clear();
            self.log_start = now;
        }
    }
}

impl eframe::App for RoomApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.stats();
        if self.custom_frame {
            frame::title_bar(ui, "DMX-9 egui room view");
            frame::resize_edges(ui);
        }
        egui::CentralPanel::default().show(ui, |ui| {
            ui.label(format!("egui 0.37 with glow: {}", self.shown));
            ui.label(match self.aim {
                Some(p) => format!("aim point {:.2} {:.2} {:.2}; drag to orbit, wheel to zoom, click to aim", p[0], p[1], p[2]),
                None => "drag to orbit, wheel to zoom, click on the floor or a wall to aim the selected fixtures".into(),
            });
            let size = ui.available_size();
            let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click_and_drag());
            let aspect = rect.width() / rect.height().max(1.0);
            let drag = response.drag_motion();
            self.cam.yaw -= drag.x * 0.008;
            self.cam.pitch = (self.cam.pitch + drag.y * 0.008).clamp(0.05, 1.5);
            if response.hovered() {
                let scroll = ui.input(|i| i.smooth_scroll_delta.y);
                self.cam.dist = (self.cam.dist * (1.0 - scroll * 0.002)).clamp(3.0, 30.0);
            }
            if response.clicked() {
                if let Some(pos) = response.interact_pointer_pos() {
                    let nx = (pos.x - rect.left()) / rect.width() * 2.0 - 1.0;
                    let ny = 1.0 - (pos.y - rect.top()) / rect.height() * 2.0;
                    self.aim = scene::pick(&self.cam, nx, ny, aspect).or(self.aim);
                }
            }
            let t0 = Instant::now();
            let mvp = self.cam.view_proj(aspect);
            {
                let mut f = self.frame.lock();
                let Frame { verts, opaque } = &mut *f;
                *opaque = scene::build(verts, &self.cam, self.start.elapsed().as_secs_f32(), self.aim);
            }
            let build = t0.elapsed().as_secs_f64() * 1000.0;
            self.build_ms.push(build);
            self.log_build.push(build);
            let renderer = Arc::clone(&self.renderer);
            let frame = Arc::clone(&self.frame);
            ui.painter().add(egui::PaintCallback {
                rect,
                callback: Arc::new(egui_glow::CallbackFn::new(move |info, painter| {
                    use glow::HasContext as _;
                    // Keep the clear inside the view, so the labels above it stay.
                    let clip = info.viewport_in_pixels();
                    unsafe {
                        painter.gl().enable(glow::SCISSOR_TEST);
                        painter.gl().scissor(clip.left_px, clip.from_bottom_px, clip.width_px, clip.height_px);
                    }
                    let f = frame.lock();
                    renderer.lock().paint(painter.gl(), &f.verts, f.opaque, &mvp);
                })),
            });
        });
        ui.ctx().request_repaint();
    }
}

#[cfg(target_arch = "wasm32")]
mod web {
    use eframe::wasm_bindgen::{self, prelude::*};
    use eframe::wasm_bindgen::JsCast as _;

    #[wasm_bindgen(start)]
    pub fn start() {
        eframe::WebLogger::init(log::LevelFilter::Info).ok();
        wasm_bindgen_futures::spawn_local(async {
            let canvas = web_sys::window()
                .and_then(|w| w.document())
                .and_then(|d| d.get_element_by_id("room"))
                .expect("canvas #room")
                .dyn_into::<web_sys::HtmlCanvasElement>()
                .expect("canvas");
            let options = eframe::WebOptions { depth_buffer: 24, ..Default::default() };
            let runner = eframe::WebRunner::new();
            runner
                .start(canvas, options, Box::new(|cc| Ok(Box::new(super::RoomApp::new(cc)))))
                .await
                .expect("start");
            std::mem::forget(runner);
        });
    }
}
