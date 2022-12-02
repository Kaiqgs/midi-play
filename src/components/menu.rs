use async_trait::async_trait;

use crate::models::{menu::StringDialogable, record::Recording};

use super::{Component, MenuModel, MenuError};
pub struct Menu {}

impl Menu {
    pub fn new() -> Self {
        Menu {}
    }
}

impl Component for Menu {
    fn draw() {
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
