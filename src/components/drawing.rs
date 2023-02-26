use std::{
    cell::{Cell, RefCell},
    sync::{Mutex, RwLock},
};

use ggez::graphics::{Color, DrawParam, Image, Mesh, MeshBuilder, Text};
use log::debug;

pub struct Drawing {
    pub params: DrawParam,
    pub image: Option<Image>,
    pub width: Option<u32>,
    pub height: Option<u32>,

    pub color: Option<Color>,
    pub mesh: Option<Mesh>,
    pub meshbuilder: Option<MeshBuilder>,
    pub text: Option<Text>,
}

impl Drawing {
    pub fn new_mesh(mesh: MeshBuilder) -> Self {
        Drawing {
            meshbuilder: Some(mesh),
            ..Default::default()
        }
    }

    pub fn new_image(image: Option<Image>) -> Self {
        Drawing {
            image,
            ..Default::default()
        }
    }
}

impl Clone for Drawing {
    fn clone(&self) -> Self {
        Drawing {
            params: self.params.clone(),
            image: self.image.clone(),
            color: self.color.clone(),
            meshbuilder: self.meshbuilder.clone(),
            text: self.text.clone(),
            width: self.width.clone(),
            height: self.height.clone(),
            mesh: self.mesh.clone(),
        }
    }
}
impl Default for Drawing {
    fn default() -> Self {
        Self {
            params: DrawParam::new(),
            image: Default::default(),
            color: Default::default(),
            meshbuilder: Default::default(),
            text: Default::default(),
            width: Default::default(),
            height: Default::default(),
            mesh: Default::default(),
        }
    }
}

pub type DrawingReference = RefCell<Drawing>;
pub struct DrawError;
pub enum DrawResult {
    Draw(DrawParam),
    Skip,
}
pub type RetrieveDrawing = Result<DrawingReference, DrawError>;
// pub enum RetrieveDrawing<'a> {
// Ref(&'a mut Drawing),
// New(Drawing),
// }
// pub type GetDrawing<'a> = Option<&'a mut Drawing>;
// pub struct DrawResult<'a> {
//     pub params: DrawParam,
//     pub drawing: &'a Drawing,
// }
