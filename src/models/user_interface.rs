use super::{menu::Menu, build_context::BuildContext, track_library::TrackLibrary};

pub struct UserInterface{
    pub menu: Menu,
    pub track_library: TrackLibrary,
}



impl UserInterface {
    pub fn new(bctx:BuildContext) -> Self {
        let menu = Menu::new(bctx.clone(), None);
        let track_library = TrackLibrary::new(bctx.clone());
        Self {
            menu,
            track_library
        }
    }
}
