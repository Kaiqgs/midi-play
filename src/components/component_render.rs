use ggez::{
    graphics::{Canvas, DrawParam, Image, Mesh, MeshBuilder, Text},
    Context,
};
use log::{info, trace};

use super::drawing::DrawingReference;
use crate::models::window_context::WindowContext;

fn draw_mesh_builder(
    mesh_builder: &Option<MeshBuilder>,
    gfx: &Context,
    canvas: &mut Canvas,
    params: DrawParam,
) {
    match mesh_builder.as_ref() {
        Some(mb) => {
            let mesh_data = mb.build();
            trace!(
                "Warning: costly mesh generation {}x tri {}x idx;",
                mesh_data.vertices.len(),
                mesh_data.indices.len()
            );
            let mesh = Mesh::from_data(gfx, mesh_data);
            draw_mesh(&Some(mesh), &None, gfx, canvas, params);
        }
        None => (),
    }
}

fn draw_mesh(
    mesh: &Option<Mesh>,
    meshbuilder: &Option<MeshBuilder>,
    gfx: &Context,
    canvas: &mut Canvas,
    params: DrawParam,
) {
    match mesh.as_ref() {
        Some(mesh) => {
            trace!(
                "Mesh render: Vertex[{}] Index[{}]",
                mesh.vertex_count(),
                mesh.index_count()
            );
            canvas.draw(mesh, params);
        }
        None => draw_mesh_builder(meshbuilder, gfx, canvas, params),
    }
}

fn draw_image(image: &Option<Image>, _gfx: &Context, canvas: &mut Canvas, params: DrawParam) {
    match image.as_ref() {
        Some(image) => {
            canvas.draw(image, params);
        }
        None => (),
    }
}

fn draw_text(text: &Text, _gfx: &Context, canvas: &mut Canvas, params: DrawParam) {
    match params.transform {
        ggez::graphics::Transform::Values { dest, .. } => {
            //anchor center by default
            let text_measure = text.measure(_gfx).expect("text measure failed");
            info!("text measure: {:?}", text_measure);
            info!("dest: {:?}", dest);
            let params = params
                .dest([dest.x - text_measure.x/2.0, dest.y - text_measure.y/2.0]);
            canvas.draw(text, params);
        }
        ggez::graphics::Transform::Matrix(_) => todo!(),
    }
    // params.dest()
}

pub(crate) fn render_drawing(
    drawing_ref: DrawingReference,
    params: DrawParam,
    canvas: &mut Canvas,
    gfx: &Context,
    _winctx: &WindowContext,
) {
    let drawing = drawing_ref.borrow();
    draw_mesh(&drawing.mesh, &drawing.meshbuilder, gfx, canvas, params);
    draw_image(&drawing.image, gfx, canvas, params);
    match drawing.text {
        Some(ref text) => draw_text(text, gfx, canvas, params),
        None => (),
    };
    if drawing.texts.len() == drawing.texts_params.len() {
        trace!("Drawing texts: {}", drawing.texts.len());
        for i in 0..drawing.texts.len() {
            let opttxt = drawing.texts.get(i).expect("Failed to get text");
            draw_text(opttxt, gfx, canvas, drawing.texts_params[i]);
        }
    }
}
