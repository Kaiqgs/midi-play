

use async_trait::async_trait;

use super::dialogable::StringDialogable;
use crate::models::record::Recording;

pub type StringResult = Result<String, String>;

#[async_trait]
pub trait Menu {
    async fn search_midi(&mut self, dialog: StringDialogable) -> StringResult;
    async fn save_recording(
        &mut self,
        dialog: StringDialogable,
        recording: Recording,
    ) -> StringResult;
    fn set_volume(&mut self, rate: f64) -> f64;
}
