//! A window frame drawn by egui (`--frame custom`): the same title bar, buttons and resize edges on
//! every platform. Throwaway code.

use eframe::egui::{self, Align, Layout, ResizeDirection, Sense, ViewportCommand};

/// Title bar with drag to move, double click to maximize, and minimize, maximize and close buttons.
pub fn title_bar(ui: &mut egui::Ui, title: &str) {
    egui::Panel::top("title_bar").exact_size(34.0).show(ui, |ui| {
        let rect = ui.max_rect();
        let bar = ui.interact(rect, ui.scope_id().with("drag"), Sense::click_and_drag());
        if bar.double_clicked() {
            let max = ui.input(|i| i.viewport().maximized.unwrap_or(false));
            ui.send_viewport_cmd(ViewportCommand::Maximized(!max));
        } else if bar.drag_started_by(egui::PointerButton::Primary) {
            ui.send_viewport_cmd(ViewportCommand::StartDrag);
        }
        ui.painter().text(rect.center(), egui::Align2::CENTER_CENTER, title, egui::FontId::proportional(15.0), ui.visuals().strong_text_color());
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            let button = |ui: &mut egui::Ui, text: &str| ui.add(egui::Button::new(egui::RichText::new(text).size(16.0)).frame(false).min_size(egui::vec2(32.0, 28.0)));
            if button(ui, "✕").clicked() {
                ui.send_viewport_cmd(ViewportCommand::Close);
            }
            let max = ui.input(|i| i.viewport().maximized.unwrap_or(false));
            if button(ui, if max { "🗗" } else { "🗖" }).clicked() {
                ui.send_viewport_cmd(ViewportCommand::Maximized(!max));
            }
            if button(ui, "🗕").clicked() {
                ui.send_viewport_cmd(ViewportCommand::Minimized(true));
            }
        });
    });
}

/// Without a system frame the window has no resize borders: a 6 px band along each edge starts a
/// resize through the compositor.
pub fn resize_edges(ui: &mut egui::Ui) {
    let rect = ui.ctx().content_rect();
    let Some(pos) = ui.input(|i| i.pointer.hover_pos()) else { return };
    let band = 6.0;
    let (l, r) = (pos.x - rect.left() < band, rect.right() - pos.x < band);
    let (t, b) = (pos.y - rect.top() < band, rect.bottom() - pos.y < band);
    let dir = match (l, r, t, b) {
        (true, _, true, _) => ResizeDirection::NorthWest,
        (_, true, true, _) => ResizeDirection::NorthEast,
        (true, _, _, true) => ResizeDirection::SouthWest,
        (_, true, _, true) => ResizeDirection::SouthEast,
        (true, ..) => ResizeDirection::West,
        (_, true, ..) => ResizeDirection::East,
        (_, _, true, _) => ResizeDirection::North,
        (_, _, _, true) => ResizeDirection::South,
        _ => return,
    };
    let icon = match dir {
        ResizeDirection::North | ResizeDirection::South => egui::CursorIcon::ResizeVertical,
        ResizeDirection::East | ResizeDirection::West => egui::CursorIcon::ResizeHorizontal,
        ResizeDirection::NorthWest | ResizeDirection::SouthEast => egui::CursorIcon::ResizeNwSe,
        _ => egui::CursorIcon::ResizeNeSw,
    };
    ui.ctx().set_cursor_icon(icon);
    if ui.input(|i| i.pointer.primary_pressed()) {
        ui.send_viewport_cmd(ViewportCommand::BeginResize(dir));
    }
}
