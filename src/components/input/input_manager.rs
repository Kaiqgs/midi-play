use crate::{
    components::component::{Component, ComponentObject},
    models::input::input_manager::InputManager,
};

impl Component for InputManager {
    fn get_name(&self) -> String {
        "[Input Manager]".to_string()
    }

    // fn update(&mut self, canvas: crate::models::render_util::RenderUtil) {
    // }

    fn next(&self) -> Vec<ComponentObject> {
        vec![&self.virtual_piano]
    }
}
