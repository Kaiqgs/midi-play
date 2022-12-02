use super::{clef::Clef, Component};

const LINES: i32 = 5;
const SPACES: i32 = 4;

/// Draws Staff w/ lines & spaces;
pub struct Staff {
    clef: Clef,
}

impl Component for Staff {
    fn draw() {
        todo!()
    }
}
