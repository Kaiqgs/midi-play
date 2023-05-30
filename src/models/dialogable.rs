pub trait Dialogable: Send {
    type Title;
    type Path;
    type Success;
    fn open(&mut self, _title: Self::Title, _default: Self::Path) -> Self::Success {
        self.close()
    }
    fn close(&mut self) -> Self::Success;
}

