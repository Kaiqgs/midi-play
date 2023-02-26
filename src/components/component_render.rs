use ggez::{
    graphics::{Canvas, DrawParam, Image, Mesh, MeshBuilder, Text},
    Context,
};
use log::{info, trace};

use super::{
    component::{BuildContext, WindowContext},
    drawing::DrawingReference,
};

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

fn draw_image(image: &Option<Image>, gfx: &Context, canvas: &mut Canvas, params: DrawParam) {
    match image.as_ref() {
        Some(image) => {
            canvas.draw(image, params);
        }
        None => (),
    }
}

fn draw_text(text: &Option<Text>, gfx: &Context, canvas: &mut Canvas, params: DrawParam) {
    match text.as_ref() {
        Some(text) => {
            canvas.draw(text, DrawParam::new());
        }
        None => (),
    }
}

pub(crate) fn render_drawing(
    drawing_ref: DrawingReference,
    params: DrawParam,
    canvas: &mut Canvas,
    gfx: &Context,
    winctx: &WindowContext,
) {
    let drawing = drawing_ref.borrow();
    draw_mesh(&drawing.mesh, &drawing.meshbuilder, gfx, canvas, params);
    draw_image(&drawing.image, gfx, canvas, params);
    draw_text(&drawing.text, gfx, canvas, params);
}
