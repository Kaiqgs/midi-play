use std::rc::Rc;

use crate::components::component::{BuildContext, Component, RenderUtilObject};


use crate::models::render_util::MockRenderUtil;
use crate::models::sheet::staff_system::StaffSystem;

fn setup() -> StaffSystem {
    StaffSystem::new(
        None,
        None,
        BuildContext::default(),
    )
}

#[test]
fn draw() {
    let render_util = MockRenderUtil::new();
    //TODO: fix mock;
    //mdraw.expect_staff_block().times(2..5);
    let render_rc: RenderUtilObject = Rc::new(&render_util);
    let sscomponent = setup();
    let dresult = sscomponent.draw(render_rc);
    assert!(dresult.drawing.meshbuilder.is_some());
}
