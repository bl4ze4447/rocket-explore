use egui::{Align2, Color32, FontFamily, FontId, Image, ImageSource, Response, Ui, Vec2};

pub fn harddisk_widget(ui: &mut Ui, selected: bool, text: &str, size_free: &str, size_total: &str) -> Response {
    let desired_space = ui.spacing().interact_size.y * egui::vec2(1.0, 3.0);
    let (rect, mut response) = ui.allocate_at_least(desired_space + Vec2::new(13.0 * (text.len() + size_free.len() + size_total.len()) as f32 , 0.0) / 1.5, egui::Sense::click());

    response.widget_info(|| egui::WidgetInfo::selected(egui::WidgetType::Button, selected, "HardDisk Widget"));

    if ui.is_rect_visible(rect) {
        let visuals = ui.style().interact(&response);
        let rect = rect.expand(visuals.expansion);
        let rounding = 0.15 * rect.height();
        ui.painter().rect(rect, rounding, if selected {
            Color32::from_rgb(100,149,237)
        } else {
            visuals.bg_fill
        }, visuals.bg_stroke);

        let color = if selected { visuals.text_color().gamma_multiply(1.5) } else { visuals.text_color() };

        ui.painter().text(rect.left_center() + Vec2::new(10.0, 0.0), Align2::LEFT_CENTER, text, FontId {
            size: 15.0,
            family: FontFamily::Monospace,
        }, color);
        ui.painter().text(rect.left_bottom() + Vec2::new(10.0, 0.0), Align2::LEFT_BOTTOM, size_free, FontId {
            size: 12.0,
            family: FontFamily::Monospace,
        }, color);
        ui.painter().text(rect.right_bottom() - Vec2::new(10.0, 0.0), Align2::RIGHT_BOTTOM, size_total, FontId {
            size: 12.0,
            family: FontFamily::Monospace,
        }, color);
    }

    response
}