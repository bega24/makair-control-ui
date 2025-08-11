use crate::display::widget::ControlWidget;

use conrod_core::{widget, widget::Id as WidgetId};

pub struct Config {
    pub text: String,   // Le texte à afficher
    pub id: WidgetId,   // L'ID du widget texte
}

pub fn render(master: &mut ControlWidget, config: Config) -> f64 {
    // Dessiner un rectangle transparent (cadre)
    widget::rectangle::Rectangle::fill_with([config.width, config.height], color::TRANSPARENT)
        .top_left_with_margins_on(config.parent, 10.0, 10.0)
        .set(config.ids.0, &mut master.ui);

    // Afficher le texte example
    widget::Text::new(config.text)
        .color(color::WHITE)
        .font_size(14)
        .top_left_with_margins_on(config.parent, 15.0, 15.0)
        .set(config.ids.1, &mut master.ui);

    config.width
}
