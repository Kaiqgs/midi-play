use async_trait::async_trait;
use ggez::context::Has;

use crate::models::{
    dialogable::StringDialogable,
    menu::{Menu as MenuModel, StringResult},
    record::Recording,
};

pub struct MenuComponentData {}

pub struct Menu {
    component_data: MenuComponentData,
}

impl Default for Menu {
    fn default() -> Self {
        Self::new()
    }
}

impl Menu {
    pub fn new() -> Self {
        Menu {
            component_data: MenuComponentData {},
        }
    }
}

impl Has<MenuComponentData> for Menu {
    fn retrieve(&self) -> &MenuComponentData {
        &self.component_data
    }
}

// impl Component for Menu {}

#[async_trait]
impl MenuModel for Menu {
    async fn search_midi(&mut self, _dialog: StringDialogable) -> StringResult {
        unimplemented!()
    }

    async fn save_recording(
        &mut self,
        _dialog: StringDialogable,
        _recording: Recording,
    ) -> StringResult {
        unimplemented!()
    }

    fn set_volume(&mut self, _rate: f64) -> f64 {
        unimplemented!()
    }
}
