use std::path::Path;

use ggez::{filesystem::Filesystem, GameError};

use crate::models::sheet::SheetTrack;

pub trait SheetTransformer: Send + Sync {
    fn convert(&self) -> SheetTrack;
}

pub trait SheetFromFile {
    fn parse(&mut self, filepath: &Path, fs:&Filesystem) -> Result<SheetTrack, GameError>;
    fn parse_bytes(&mut self, bytes: &[u8]) -> Result<SheetTrack, GameError>;
}
