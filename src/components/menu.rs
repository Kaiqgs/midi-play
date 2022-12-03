use async_trait::async_trait;
use ggez::graphics::Image;

use crate::models::{menu::{StringDialogable, MenuError, Menu as MenuModel}, record::Recording, draw_util::DrawObject};

use super::component::{Component, Drawing};

pub struct Menu {}

impl Menu {
    pub fn new() -> Self {
        Menu {}
    }
}

impl Component<Image> for Menu {
    fn draw(&self, canvas: DrawObject) -> Drawing<Image>{
        unimplemented!()
    }
}

#[async_trait]
impl MenuModel for Menu {
    async fn search_midi(&mut self, dialog: StringDialogable) -> Result<String, MenuError> {
        unimplemented!()
    }

    async fn save_recording(&mut self, dialog: StringDialogable, recording:Recording) -> Result<String, MenuError> {
        unimplemented!()
    }

    fn set_volume(&mut self, rate: f64) -> f64 {
        unimplemented!()
    }
}
