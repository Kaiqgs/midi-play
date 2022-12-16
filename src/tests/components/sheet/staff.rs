use crate::{
    components::component::{BuildContext, Component, RenderUtilObject},
    models::{
        render_util::MockRenderUtil,
        sheet::{clef::Clef, staff::Staff},
    },
};

fn setup() -> Staff {
    Staff::new(Clef::new_treble(None), None, BuildContext::default())
}

#[test]
fn draw() {
    let staffcomp = setup();
    let canvas = MockRenderUtil::new();
    let mdraw = RenderUtilObject::new(&canvas);
    //TODO: fix mock
    //canvas.expect_image().return_const(true).once();
    

    let dresult = staffcomp.draw(mdraw);

    assert!(dresult.drawing.image.is_some())
}
