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
            // egui's built-in fonts have no window-control glyphs, so the icons are drawn as lines.
            if icon_button(ui, Icon::Close).clicked() {
                ui.send_viewport_cmd(ViewportCommand::Close);
            }
            let max = ui.input(|i| i.viewport().maximized.unwrap_or(false));
            if icon_button(ui, if max { Icon::Restore } else { Icon::Maximize }).clicked() {
                ui.send_viewport_cmd(ViewportCommand::Maximized(!max));
            }
            if icon_button(ui, Icon::Minimize).clicked() {
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

#[derive(Clone, Copy)]
enum Icon {
    Minimize,
    Maximize,
    Restore,
    Close,
}

fn icon_button(ui: &mut egui::Ui, icon: Icon) -> egui::Response {
    let (rect, response) = ui.allocate_exact_size(egui::vec2(40.0, 30.0), Sense::click());
    let visuals = ui.style().interact(&response);
    if response.hovered() {
        let fill = if matches!(icon, Icon::Close) { egui::Color32::from_rgb(196, 43, 28) } else { visuals.bg_fill };
        ui.painter().rect_filled(rect.shrink(2.0), 4.0, fill);
    }
    let c = rect.center();
    let s = 5.0;
    let stroke = egui::Stroke::new(1.3, visuals.fg_stroke.color);
    let p = ui.painter();
    match icon {
        Icon::Minimize => {
            p.line_segment([c + egui::vec2(-s, 0.0), c + egui::vec2(s, 0.0)], stroke);
        }
        Icon::Maximize => {
            p.rect_stroke(egui::Rect::from_center_size(c, egui::vec2(2.0 * s, 2.0 * s)), 1.0, stroke, egui::StrokeKind::Middle);
        }
        Icon::Restore => {
            p.rect_stroke(egui::Rect::from_center_size(c + egui::vec2(-1.5, 1.5), egui::vec2(1.6 * s, 1.6 * s)), 1.0, stroke, egui::StrokeKind::Middle);
            p.line_segment([c + egui::vec2(-s + 3.0, -s), c + egui::vec2(s, -s)], stroke);
            p.line_segment([c + egui::vec2(s, -s), c + egui::vec2(s, s - 3.0)], stroke);
        }
        Icon::Close => {
            p.line_segment([c + egui::vec2(-s, -s), c + egui::vec2(s, s)], stroke);
            p.line_segment([c + egui::vec2(-s, s), c + egui::vec2(s, -s)], stroke);
        }
    }
    response
}
