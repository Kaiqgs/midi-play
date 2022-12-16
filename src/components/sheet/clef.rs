use std::path::Path;

use ggez::{
    context::Has,
    graphics::{self, Color, DrawParam, Image, MeshBuilder},
    mint::Point2,
    Context,
};

use crate::{
    components::{
        draw_util::DrawUtilGG,
        component::{BuildContext, Component, ComponentObject, RenderUtilObject},
        drawing::{DrawResult, Drawing},
    },
    models::{draw_util::DrawUtil, sheet::clef::Clef, note::Note},
};

use super::definition;

/// Draws clef symbol;
pub struct ClefComponentData {
    filepath: Option<String>,
    drawing: Drawing,
}

fn image_from_optional(ctx: &Context, path: Option<String>) -> Image {
    match path {
        Some(filepath) => {
            let imres = Image::from_path(ctx, Path::new(&filepath));
            match imres {
                Ok(image) => image,
                Err(_) => todo!(),
            }
        }
        None => Image::from_solid(ctx, 128, Color::GREEN),
    }
}

impl ClefComponentData {
    pub fn new(note: &Note, filepath: Option<String>, build: BuildContext) -> Self {
        let mut drawing = Drawing::default();
    
        match build.ctx {
            Some(ctx) => {
                drawing.image = Some(image_from_optional(ctx, filepath.clone()));
                DrawUtilGG::left_image(&mut drawing, build.clone(), note);
            }
            None => (),
        };
        ClefComponentData {
            filepath: filepath.clone(),
            drawing,
        }
    }
}

impl Component for Clef {
    fn draw(&self, canvas: RenderUtilObject) -> DrawResult {
        
        
        
        
        DrawResult {
            params: self.component_data.drawing.params,
            drawing: &self.component_data.drawing,
        }
    }

    fn next(&self) -> Vec<ComponentObject> {
        Vec::new()
    }
}
