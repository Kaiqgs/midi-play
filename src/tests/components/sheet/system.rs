use crate::{components::{sheet::SheetTrackComponent, component::Component}, models::draw_util::MockDrawUtil};

fn setup() -> SheetTrackComponent {
    SheetTrackComponent::new()
}

#[test]
fn draw(){
    let mut mdraw = MockDrawUtil::new();
    mdraw.expect_line().times(88);
    let bdraw = Box::new(mdraw);
    let stcomponent = setup();
    stcomponent.draw(bdraw);
}