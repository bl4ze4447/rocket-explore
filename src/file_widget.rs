use egui::{Align2, Color32, FontFamily, FontId, Response, Sense, Ui, Vec2};

pub fn file_widget(ui: &mut Ui, selected: bool, text: &str) -> Response {
    let desired_space = ui.spacing().interact_size.y * egui::vec2(1.0, 2.0);
    let (id, rect) = ui.allocate_space(desired_space + Vec2::new(15.0 * text.len() as f32 / 1.5, 0.0));
    let response = ui.interact(rect, id, Sense::click_and_drag());

    response.widget_info(|| egui::WidgetInfo::selected(egui::WidgetType::Button, selected, "File Widget"));

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
    }

    response
}