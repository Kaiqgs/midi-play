use core::panic;

use ggez::{
    graphics::{Color, DrawParam, Mesh, MeshBuilder},
    mint::Point2,
    Context,
};

use crate::models::note::Note;

use super::drawing::Drawing;
use super::sheet::sheet_component_const::{self, Zindex};
use crate::models::build_context::BuildContext;
use crate::models::window_context::WindowContext;

pub struct DrawUtil {}

impl DrawUtil {
    pub fn new() -> Self {
        DrawUtil {}
    }
    pub fn staff_block<'a>(
        draw: &'a mut Drawing,
        build: BuildContext,
        notes: Vec<&Note>,
        color: Color,
    ) {
        let meshopt = draw.meshbuilder.as_mut();
        match meshopt {
            Some(mb) => {
                for note in notes {
                    if note.id % 2 != 0 {
                        continue; //Whites;
                    }
                    let scaled_note = note.line * sheet_component_const::NOTE_HEIGHT;
                    let level = scaled_note;
                    let start = Point2::from([0.0, level as f32]);
                    let end = Point2::from([build.winctx.size.x as f32, level as f32]);
                    mb.line(&[start, end], 1.0, color)
                        .expect("Failed to draw staff");
                }

                match build.ctx {
                    Some(ctx) => draw.mesh = Some(Mesh::from_data(ctx, mb.build())),
                    None => (),
                }
                let params = DrawParam::new()
                    .dest(Point2::from([0.0, 0.0]))
                    .scale([build.winctx.scale, build.winctx.scale])
                    .z(Zindex::Track.get());
                draw.params = params;
            }
            None => panic!("Mandatory mesh builder;"),
        }
    }

    pub fn left_image<'a>(
        draw: &'a mut Drawing,
        winctx: &'a WindowContext,
        note: &'a Note,
    ) -> DrawParam {
        // image requires manual scaling...
        // dependency scale;
        let scaled_note = note.line * sheet_component_const::NOTE_HEIGHT;
        // image displaying upscaling;
        let scaled_note = scaled_note as f32 * winctx.scale;
        let ledger_padding = 2.0 * winctx.scale * sheet_component_const::NOTE_HEIGHT as f32;
        let mut level = scaled_note + ledger_padding;

        match &draw.image {
            Some(image) => {
                let scaled_height = image.height() as f32 * winctx.scale - winctx.scale / 2.0;
                level -= scaled_height;
            }
            None => panic!("Mandatory image"),
        }
        let params = DrawParam::new()
            .dest(Point2::from([0.0, level as f32]))
            .scale([winctx.scale, winctx.scale])
            .z(Zindex::Note.get());

        draw.params = params;
        params
    }
}

#[allow(unused)]
fn build_mesh_if_context<'a>(ctxopt: Option<&'a Context>, mb: &'a MeshBuilder, draw: &mut Drawing) {
    match ctxopt {
        Some(ctx) => draw.mesh = Some(Mesh::from_data(ctx, mb.build())),
        None => todo!(),
    }
}
