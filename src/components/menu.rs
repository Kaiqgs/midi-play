use ggez::{
    graphics::{DrawParam, Text},
    mint::Point2,
};

use crate::models::{
    bit_mode::BitMask, game_mode::GameMode, input::input::MidiPlayInput, menu::Menu,
};

use super::{
    component::Component,
    drawing::{DrawResult, Drawing},
    render_util::deftext,
};

pub const MENU_OPTIONS_SIZE: usize = 4;
pub type MenuArray = [MenuOption; MENU_OPTIONS_SIZE];

pub struct MenuComponentData {
    pub drawing: Drawing,
    pub request_input: Option<MidiPlayInput>,
    pub option_selected: usize,
    pub menu_options: MenuArray,
}

pub struct MenuOption {
    pub text: String,
    pub action: MidiPlayInput,
}

impl MenuOption {
    pub fn new(text: String, action: MidiPlayInput) -> Self {
        MenuOption { text, action }
    }
}

impl MenuComponentData {
    pub fn new() -> Self {
        MenuComponentData {
            drawing: Drawing::default(),
            request_input: None,
            option_selected: 0,
            menu_options: [
                MenuOption::new(
                    "the quick".into(), //"play".into(),
                    MidiPlayInput::ModeChange(GameMode::Library),
                ),
                MenuOption::new(
                    "brown fox".into(), //"settings".into(),
                    MidiPlayInput::ModeChange(GameMode::Library),
                ),
                MenuOption::new(
                    "jumps over".into(), //"editor".into(),
                    MidiPlayInput::ModeChange(GameMode::Library),
                ),
                MenuOption::new(
                    "the lazy dog".into(), //"quit".into(),
                    MidiPlayInput::Quit,
                ),
            ],
        }
    }
}

/**
*
* menu options:
*  - library
*  - settings
*  - quit
*/

impl Component for Menu {
    fn get_name(&self) -> String {
        String::from("[Menu]")
    }

    fn update(&mut self, _reutil: crate::models::render_util::RenderUtil) {
        let n_items = self.component_data.menu_options.len();
        let texts: Vec<Text> = self
            .component_data
            .menu_options
            .iter()
            .enumerate()
            .map(|(i, opt)| {
                deftext(
                    &opt.text.clone(),
                    _reutil.winctx.scale
                        * if i == self.component_data.option_selected {
                            1.2
                        } else {
                            1.0
                        },
                )
            })
            .collect();
        let div = _reutil.winctx.size.y as f32 / (n_items * 2) as f32;
        let mut text_params: Vec<DrawParam> = Vec::new();
        let centerx = _reutil.winctx.size.x as f32 / 2.0;
        let centery = _reutil.winctx.size.y as f32 / 2.0;
        let halfsize = (n_items / 2) as f32 * div;
        for i in 0..n_items {
            // let text = texts[i];
            let y = centery - halfsize + div * i as f32 + div / 2.0;
            let param = DrawParam::new().dest(Point2::from([centerx, y])); //.scale([_reutil.winctx.scale, _reutil.winctx.scale]);
            text_params.push(param);
        }
        self.component_data.drawing.texts_params = text_params;
        self.component_data.drawing.texts = texts;
    }

    fn draw(&self, _reutil: crate::models::render_util::RenderUtil) -> super::drawing::DrawResult {
        DrawResult::Draw(self.component_data.drawing.params)
    }

    fn get_drawing(&self) -> super::drawing::RetrieveDrawing {
        super::drawing::RetrieveDrawing::Ok(super::drawing::DrawingReference::new(
            self.component_data.drawing.clone(),
        ))
    }

    fn request_input(&mut self) -> Option<MidiPlayInput> {
        let input = self.component_data.request_input.clone();
        if self.component_data.request_input.is_some() {
            self.component_data.request_input = None;
        }
        input
    }

    fn next(&self) -> Vec<super::component::ComponentObject> {
        vec![]
    }

    fn next_mut(&mut self) -> Vec<super::component::MutComponentObject> {
        vec![]
    }

    fn get_mask(&self) -> crate::models::bit_mode::BitMask {
        BitMask::default().allow(GameMode::Menu)
    }

    fn handle_input(&mut self, input: MidiPlayInput, reutil: crate::models::render_util::RenderUtil) {
        match input {
            MidiPlayInput::NextOption => {
                self.component_data.option_selected =
                    (self.component_data.option_selected + 1) % MENU_OPTIONS_SIZE
            }
            MidiPlayInput::PreviousOption => {
                self.component_data.option_selected =
                    (self.component_data.option_selected + MENU_OPTIONS_SIZE - 1)
                        % MENU_OPTIONS_SIZE
            }

            MidiPlayInput::SelectOption => {
                self.component_data.request_input = Some(
                    self.component_data.menu_options[self.component_data.option_selected]
                        .action
                        .clone(),
                )
            }
            _ => (),
        }
    }
}
