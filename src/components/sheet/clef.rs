use ggez::graphics::Image;

use crate::{components::component::{Component, Drawing}, models::draw_util::DrawObject};

/// Draws clef symbol;
pub struct Clef {}

impl Component<Image> for Clef {
    fn draw(&self, canvas: DrawObject) -> Drawing<Image> {
        todo!()
    }
}
