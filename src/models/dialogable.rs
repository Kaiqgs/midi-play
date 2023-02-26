use ggez::glam::bool;
use mockall::automock;

use super::menu::StringResult;

#[automock(
    type Title = String;
    type Path = String;
    type Success = StringResult;
)]
pub trait Dialogable: Send {
    type Title;
    type Path;
    type Success;
    fn open(&mut self, title: Self::Title, default: Self::Path) -> Self::Success {
        self.close()
    }
    fn close(&mut self) -> Self::Success;
}

pub type StringDialogable =
    Box<dyn Dialogable<Title = String, Path = String, Success = StringResult>>;
