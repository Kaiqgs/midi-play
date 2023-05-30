use super::constants::DEFAULT_FONT;
use ggez::graphics::Text;
use log::info;

pub fn deftext(text: &str, scale: f32) -> Text {
    let mut txt = Text::new(text);
    txt.set_font(DEFAULT_FONT);
    txt.set_scale(scale * 10.0);
    info!("Text scale: {}", scale);
    txt
}
