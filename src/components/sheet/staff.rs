use ggez::graphics::Mesh;

use crate::{components::component::{Component, Drawing}, models::draw_util::DrawObject};

use super::clef::Clef;


const LINES: i32 = 5;
const SPACES: i32 = 4;

/// Draws Staff w/ lines & spaces;
pub struct Staff {
    clef: Clef,
}

impl Component<Mesh> for Staff {
    fn draw(&self, canvas: DrawObject) -> Drawing<Mesh> {
        todo!()
    }
}
