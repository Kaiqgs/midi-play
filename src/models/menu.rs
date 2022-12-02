use std::{error::Error, fmt};

use async_trait::async_trait;
use ggez::glam::bool;
use mockall::automock;

use super::record::Recording;

#[derive(Debug)]
pub struct MenuError {
    details: String,
}

impl fmt::Display for MenuError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for MenuError {
    fn description(&self) -> &str {
        &self.details
    }
}

#[automock(
    type Title = String;
    type Path = String;
    type Success = (String, bool);
)]
pub trait Dialogable: Send {
    type Title;
    type Path;
    type Success;
    fn open(&mut self, title: Self::Title, default: Self::Path) -> Self::Success {
        self.close()
    }
    fn close(&mut self) -> Self::Success;
}

pub type StringDialogable = Box<dyn Dialogable<Title = String, Path = String, Success = (String, bool)>>;

#[async_trait]
pub trait Menu {
    async fn search_midi(&mut self, dialog: StringDialogable) -> Result<String, MenuError>;
    async fn save_recording(
        &mut self,
        dialog: StringDialogable,
        recording: Recording,
    ) -> Result<String, MenuError>;
    fn set_volume(&mut self, rate: f64) -> f64;
}
