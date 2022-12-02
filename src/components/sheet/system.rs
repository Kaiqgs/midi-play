use {super::staff::Staff, super::Component};

/// Draw systems of Staffs;
pub struct System {
    staffs: Vec<Staff>,
}

impl System {}

impl Component for System {
    fn draw() {
        todo!()
    }
}
