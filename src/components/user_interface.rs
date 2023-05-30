use crate::models::user_interface::UserInterface;

use super::component::Component;

impl Component for UserInterface{
    fn get_name(&self) -> String {
        String::from("[User Interface]")
    }

    fn next(&self) -> Vec<super::component::ComponentObject> {
        vec![&self.menu, &self.track_library]
    }

    fn next_mut(&mut self) -> Vec<super::component::MutComponentObject> {
        vec![&mut self.menu, &mut self.track_library]
    }

    // fn update(&mut self, reutil: crate::models::render_util::RenderUtil) {
    //     ()
    // }

    // fn draw(&self, reutil: crate::models::render_util::RenderUtil) -> super::drawing::DrawResult {
    //     super::drawing::DrawResult::Skip
    // }

    // fn handle_input(&mut self, input: crate::models::input::input::MidiPlayInput) {
    //     ()
    // }

    // fn request_input(&mut self) -> Option<crate::models::input::input::MidiPlayInput> {
    //     None
    // }

    // fn get_new_drawing(&self) -> super::drawing::Drawing {
    //     super::drawing::Drawing::default()
    // }

    // fn get_drawing(&self) -> super::drawing::RetrieveDrawing {
    //     super::drawing::RetrieveDrawing::Ok(super::drawing::DrawingReference::new(self.get_new_drawing()))
    // }

    // fn get_mask(&self) -> crate::models::bit_mode::BitMask {
    //     crate::models::bit_mode::BitMask::new(crate::models::bit_mode::BitmaskSetup::All)
    // }

}
