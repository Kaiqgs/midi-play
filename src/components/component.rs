use ggez::graphics::Drawable;

use crate::models::draw_util::DrawObject;

pub struct Drawing<T>
where
    T: Drawable,
{
    draw: T,
}

impl<T> Drawing<T>
where
    T: Drawable,
{
    pub fn new(draw: T) -> Self {
        Drawing { draw }
    }
}

pub trait Component<T>
where
    T: Drawable,
{
    fn draw(&self, canvas: DrawObject) -> Drawing<T>;
}
