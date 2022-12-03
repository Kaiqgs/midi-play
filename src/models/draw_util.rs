use ggez::{graphics::Mesh, mint::Point2};
use mockall::automock;

#[automock]
pub trait DrawUtil{
    fn line(&self, mesh: &Mesh);
}

pub type DrawObject = Box<dyn DrawUtil>;