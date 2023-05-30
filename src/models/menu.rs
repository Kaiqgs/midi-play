use ggez::graphics::FontData;

use crate::components::menu::MenuComponentData;

use super::build_context::BuildContext;

pub struct Menu {
    pub component_data: MenuComponentData,
}

impl Menu {
    pub fn new(bctx: BuildContext, component_data: Option<MenuComponentData>) -> Self {
        Menu {
            component_data: component_data.unwrap_or(MenuComponentData::new()),
        }
    }
}
