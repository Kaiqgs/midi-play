use core::panic;

use ggez::{
    graphics::{Color, DrawParam, Image, Mesh, MeshBuilder},
    mint::Point2,
    Context,
};

use crate::models::note::Note;

use super::sheet::sheet_component_const::{self, Zindex};
use super::{component::BuildContext, drawing::Drawing};

pub struct DrawUtil {
    width: u32,
    height: u32,
}

impl DrawUtil {
    pub fn new(width: u32, height: u32) -> Self {
        DrawUtil { width, height }
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
                    mb.line(&[start, end], 1.0, color);
                }

                match build.ctx {
                    Some(ctx) => draw.mesh = Some(Mesh::from_data(ctx, mb.build())),
                    None => (),
                }
                let mut params = DrawParam::new()
                    .dest(Point2::from([0.0, 0.0]))
                    .scale([sheet_component_const::SCALEF, sheet_component_const::SCALEF])
                    .z(Zindex::Track.get());
                draw.params = params;
            }
            None => panic!("Mandatory mesh builder;"),
        }
    }

    pub fn note_sheet<'a>(draw: &mut Drawing, note: &'a Note) {
        todo!()
    }

    pub fn left_image<'a>(draw: &'a mut Drawing, build: BuildContext, note: &'a Note) {
        //image requires manual scaling...
        //j
        let scaled_note =
            note.line * sheet_component_const::SCALE * sheet_component_const::NOTE_HEIGHT;
        let ledger_padding = 2 * sheet_component_const::SCALE * sheet_component_const::NOTE_HEIGHT;
        let mut level = scaled_note + ledger_padding;

        match &draw.image {
            Some(image) => {
                let scaled_height = image.height() * sheet_component_const::SCALE
                    - sheet_component_const::SCALE / 2;
                level -= scaled_height;
            }
            None => panic!("Mandatory image"),
        }
        let mut params = DrawParam::new()
            .dest(Point2::from([0.0, level as f32]))
            .scale([sheet_component_const::SCALEF, sheet_component_const::SCALEF])
            .z(Zindex::Note.get());
        draw.params = params;
    }

    fn note<'a>(draw: &'a mut Drawing, build: BuildContext, note: &'a Note) {
        todo!()
    }

    fn rect<'a>(draw: &'a mut Drawing, build: BuildContext, color: Color) {
        todo!()
    }
}

fn build_mesh_if_context<'a>(ctxopt: Option<&'a Context>, mb: &'a MeshBuilder, draw: &mut Drawing) {
    match ctxopt {
        Some(ctx) => draw.mesh = Some(Mesh::from_data(ctx, mb.build())),
        None => todo!(),
    }
}
