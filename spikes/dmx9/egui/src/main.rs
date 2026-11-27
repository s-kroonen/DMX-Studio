//! Desktop entry of the DMX-9 egui spike.

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default().with_inner_size([1280.0, 800.0]),
        renderer: eframe::Renderer::Glow,
        depth_buffer: 24,
        multisampling: 4,
        ..Default::default()
    };
    eframe::run_native("DMX-9 egui room view", options, Box::new(|cc| Ok(Box::new(dmx9_egui::RoomApp::new(cc)))))
}
