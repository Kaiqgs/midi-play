use ggez::{
    graphics::{Mesh, MeshBuilder},
    mint::Point2,
};

use crate::{
    components::component::{Component, Drawing},
    models::{sheet::system::StaffSystem, draw_util::{DrawObject}},
};

/// Draw systems of Staffs;
pub struct System {
    system: StaffSystem,
    position: Point2<u32>,
    size: Point2<u32>,
    mesh_builder: MeshBuilder,
}

impl System {
    pub fn new(system: StaffSystem) -> Self {
        System {
            system,
            position: Point2::from([0, 0]),
            size: Point2::from([0, 0]),
            mesh_builder: MeshBuilder::new(),
        }
    }
}

impl Component<Mesh> for System {
    fn draw(&self, canvas: DrawObject) -> Drawing<Mesh> {
        todo!()
    }
}
