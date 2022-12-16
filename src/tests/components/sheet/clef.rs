use crate::{
    components::{component::{Component, RenderUtilObject}},
    models::{sheet::clef::Clef, render_util::MockRenderUtil},
};

fn setup() -> Clef {
    Clef::new_treble(None)
}

#[test]
fn draw() {
    let mdraw = MockRenderUtil::new();
    //TODO: fix mock;
    //mdraw.expect_image().return_const(true).once();
    let mbox = RenderUtilObject::new(&mdraw);
    let stcomponent = setup();
    let dresult = stcomponent.draw(mbox);
    assert!(dresult.drawing.image.is_some());
}
