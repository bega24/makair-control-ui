use conrod_core::color;

use conrod_core::{
    widget::{self, Id as WidgetId},
    Colorable, Positionable, Widget,
};

use crate::display::widget::ControlWidget;

pub struct Config<'a> {
    pub parent: WidgetId,
    pub container: WidgetId,
    pub ids: WidgetId, 
    pub text: &'a str,
    pub width: f64,
    pub height: f64,
}

pub fn render<'a>(master: &mut ControlWidget<'a>, config: Config<'a>) -> f64 {
    // rectangle container (transparent)
    let id = config.ids;
    widget::rectangle::Rectangle::fill_with([config.width, config.height], color::TRANSPARENT)
        .top_left_of(config.parent) // positionner simplement — adapte si tu veux margins
        .set(config.ids, &mut master.ui);

    // texte
    widget::Text::new(config.text)
        
        .mid_top_with_margin_on(config.ids, 5.0)
        .color(color::WHITE)
        .font_size(14)
        .set(config.ids, &mut master.ui);

    config.width
}
