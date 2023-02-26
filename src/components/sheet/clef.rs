use std::{cell::RefCell, path::Path};

use ggez::{
    context::Has,
    graphics::{self, Color, DrawParam, Image, MeshBuilder},
    mint::Point2,
    Context,
};

use crate::{
    components::{
        component::{BuildContext, Component, ComponentObject},
        draw_util::DrawUtil,
        drawing::{DrawResult, Drawing, RetrieveDrawing},
    },
    models::{note::Note, render_util::RenderUtil, sheet::clef::Clef},
};

use super::sheet_component_const;

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
                DrawUtil::left_image(&mut drawing, build.clone(), note);
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
    fn get_name(&self) -> String {
        "[Clef]".to_string()
    }
    fn get_drawing(&self) -> RetrieveDrawing {
        RetrieveDrawing::Ok(RefCell::new(self.component_data.drawing.clone()))
    }
    fn draw(&self, canvas: RenderUtil) -> DrawResult {
        DrawResult::Draw(
            // DrawParam::new()
            //     .dest([0.0, 0.0])
            //     .scale([sheet_component_const::SCALEF, sheet_component_const::SCALEF])
            //     .z(0),
            self.component_data.drawing.params,
        )
    }
}
