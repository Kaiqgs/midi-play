use crate::components::sheet::virtual_piano::VirtualPianoComponentData;
use crate::models::note::Note;
use crate::models::sheet::staff_system::StaffSystem;

pub struct VirtualPiano {
    pub component_data: VirtualPianoComponentData,
    pub notes: Vec<Note>,
}

impl VirtualPiano {
    pub fn new() -> VirtualPiano {
        let notes = StaffSystem::notes();
        VirtualPiano {
            component_data: VirtualPianoComponentData::new(),
            notes,
        }
    }
}
