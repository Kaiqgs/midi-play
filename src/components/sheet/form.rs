use ggez::graphics::MeshBuilder;

pub(crate) const CENTER_PIECE: u16 = 0b000_101_000;
pub(crate) const BOTTOM_PIECE: u16 = 0b000_000_010;
pub(crate) const TOP_PIECE: u16 = 0b010_000_000;
pub(crate) const UNIT: u16 = 0b010_101_010;
// pub(crate) const UNIT: u16 = CENTER_PIECE + BOTTOM_PIECE + TOP_PIECE;

pub fn draw_format(mb: &mut MeshBuilder, form: u16) {}
