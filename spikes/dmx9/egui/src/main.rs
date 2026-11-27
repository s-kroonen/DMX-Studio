//! Desktop entry of the DMX-9 egui spike.
//!
//! `--frame system` (default): the system draws the window frame; on Linux the window runs through
//! XWayland, because GNOME on Wayland leaves the frame to the application.
//! `--frame custom`: no system frame; egui draws the title bar, buttons and resize edges.

fn main() -> eframe::Result {
    let custom = std::env::args().collect::<Vec<_>>().windows(2).any(|w| w[0] == "--frame" && w[1] == "custom");
    let mut options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default().with_inner_size([1280.0, 800.0]).with_decorations(!custom),
        renderer: eframe::Renderer::Glow,
        depth_buffer: 24,
        multisampling: 4,
        ..Default::default()
    };
    #[cfg(target_os = "linux")]
    if !custom {
        options.event_loop_builder = Some(Box::new(|builder| {
            use winit::platform::x11::EventLoopBuilderExtX11 as _;
            builder.with_x11();
        }));
    }
    eframe::run_native(
        "DMX-9 egui room view",
        options,
        Box::new(move |cc| {
            let mut app = dmx9_egui::RoomApp::new(cc);
            app.custom_frame = custom;
            Ok(Box::new(app))
        }),
    )
}
