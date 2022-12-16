use core::panic;

use ggez::{
    graphics::{Color, DrawParam, Image, Mesh, MeshBuilder},
    mint::Point2,
    Context,
};

use crate::models::{draw_util::DrawUtil, note::Note};

use super::sheet::definition;
use super::{component::BuildContext, drawing::Drawing};

pub struct DrawUtilGG {
    width: u32,
    height: u32,
}

impl DrawUtilGG {
    pub fn new(width: u32, height: u32) -> Self {
        DrawUtilGG { width, height }
    }
    pub fn horizontal_line(
        mesh: &mut ggez::graphics::MeshBuilder,
        level: u32,
        canvas_width: f32,
        color: Color,
    ) {
        let start = Point2::from([0.0, f32::from(level as i16)]);
        let end = Point2::from([f32::from(canvas_width as i16), f32::from(level as i16)]);
        mesh.line(&[start, end], definition::SCALE as f32, color);
    }
}

fn build_mesh_if_context<'a>(ctxopt: Option<&'a Context>, mb: &'a MeshBuilder, draw: &mut Drawing) {
    match ctxopt {
        Some(ctx) => draw.mesh = Some(Mesh::from_data(ctx, mb.build())),
        None => todo!(),
    }
}

impl DrawUtil for DrawUtilGG {
    fn staff_block<'a>(
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
                    let half_scale = (definition::SCALE / 2) as u32;
                    let scaled_note = note.line * definition::PIX_PER_HORIZONTAL_LINE * definition::SCALE;
                    let level  = half_scale + scaled_note - definition::SCALE;
                    let start = Point2::from([0.0, level as f32]);
                    let end = Point2::from([build.canvas_size.x as f32, level as f32]);
                    mb.line(&[start, end], definition::SCALE as f32, color);
                }

                match build.ctx {
                    Some(ctx) => draw.mesh = Some(Mesh::from_data(ctx, mb.build())),
                    None => (),
                }
            }
            None => panic!("Mandatory mesh builder;"),
        }
    }

    fn left_image<'a>(draw: &'a mut Drawing, build: BuildContext, note: &'a Note) {
        let scaled_note =  note.line * definition::SCALE * definition::PIX_PER_HORIZONTAL_LINE;
        let ledger_padding = 2 * definition::SCALE * definition::PIX_PER_HORIZONTAL_LINE;
        let mut level = scaled_note + ledger_padding;

        match &draw.image {
            Some(image) => {
                let scaled_height = image.height() * definition::SCALE - definition::SCALE;
                level -= scaled_height;
            }
            None => panic!("Mandatory image"),
        }
        let mut params = DrawParam::new()
            .dest(Point2::from([0.0, level as f32]))
            .scale([definition::SCALE as f32, definition::SCALE as f32]);

        draw.params = params;
        //build.ctx

        /*


        }

        let mut params = DrawParam::new()
            .dest(Point2::from([0.0, level as f32]))
            .scale([definition::SCALE as f32, definition::SCALE as f32]);

            */
    }

    fn rect<'a>(draw: &'a mut Drawing, build: BuildContext, color: Color) {
        todo!()
    }
}
