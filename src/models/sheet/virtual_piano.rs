use crate::components::sheet::virtual_piano::VirtualPianoComponentData;
use crate::models::sheet::staff_system::StaffSystem;
use crate::models::{note::Note, playmeter::QualityMeter};

pub trait VirtualPianoKeyHandler<Q: QualityMeter> {
    fn note_on(&self, comparator: Q);
    fn note_off(&self, comparator: Q);
}

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
