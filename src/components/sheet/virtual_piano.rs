use crate::{
    components::sheet::sheet_component_const::Zindex,
    models::{
        game_mode::NOTES_MASK,
        sheet::{sheet_const::Accidentals, virtual_piano::VirtualPiano},
    },
};

use ggez::{
    graphics::{DrawMode, DrawParam, MeshBuilder, Rect},
    mint::Point2,
};

use crate::{
    components::{
        component::Component,
        drawing::{DrawResult, Drawing, DrawingReference, RetrieveDrawing},
        pallete,
        sheet::sheet_component_const,
    },
    models::sheet::sheet_const,
};

pub struct VirtualPianoComponentData {
    pub drawing: DrawingReference,
}

impl VirtualPianoComponentData {
    pub fn new() -> VirtualPianoComponentData {
        VirtualPianoComponentData {
            drawing: DrawingReference::new(Drawing::default()),
        }
    }
}

impl Component for VirtualPiano {
    fn get_name(&self) -> String {
        "[Virtual Piano]".to_string()
    }

    fn update(&mut self, _reutil: crate::models::render_util::RenderUtil) {
        ()
    }

    fn get_drawing(&self) -> RetrieveDrawing {
        Ok(self.component_data.drawing.clone())
    }

    fn draw(&self, reutil: crate::models::render_util::RenderUtil) -> DrawResult {
        // return DrawResult::Skip;
        let drawing = Drawing::new_mesh(MeshBuilder::new());
        let drawing_reference = DrawingReference::new(drawing);
        self.component_data.drawing.swap(&drawing_reference);
        let mut drawing = self.component_data.drawing.borrow_mut();

        const WHITE_WIDTH: u32 = 138;
        const BLACK_WIDTH: u32 = 85;
        const WHITE_TO_BLACK: f32 = WHITE_WIDTH as f32 / BLACK_WIDTH as f32;
        const WHITE_WIDTH_PX: u32 = 15;
        const PIANO_SPACING_PX: u32 = 3;

        if let Some(mb) = drawing.meshbuilder.as_mut() {
            let canvas_width = reutil.winctx.size.x as f32 / reutil.winctx.scale;
            let canvas_height = reutil.winctx.size.y as f32 / reutil.winctx.scale;
            let position = canvas_width / 4.0;
            let width = WHITE_WIDTH_PX as f32;
            let padding = PIANO_SPACING_PX as f32;

            mb.rectangle(
                DrawMode::fill(),
                Rect::new(position - padding, 0.0, padding, canvas_height),
                pallete::LIGHTER_DARK,
            )
            .expect("Failed to draw piano padding");

            for note in self.notes.iter() {
                let scaled_note = note.line * sheet_component_const::NOTE_HEIGHT;
                let mut level = scaled_note;
                let start = Point2::from([position, level as f32]);
                let end = Point2::from([position + width, level as f32]);
                let color = if note.naturality == Accidentals::Natural && note.on.unwrap_or(false) {
                    pallete::LIGHTER_DARK
                } else {
                    pallete::LIGHT
                };
                mb.line(
                    &[start, end],
                    sheet_component_const::NOTE_HEIGHT as f32,
                    color,
                )
                .expect("Failed to draw piano white key");
                level -= 1;
                // let start = Point2::from([0.0, level as f32]);
                // let end = Point2::from([width, level as f32]);
                // mb.line(&[start, end], 1.0, pallete::DARKER_LIGHT);

                // Drawing the accidentals, sharps and flats;
                let natural_note = (note.id % sheet_const::NATURAL_NOTES) as usize;

                let mut up = false;
                let mut down = false;
                match sheet_component_const::THREE_WIDE_PROGRESSION[natural_note] {
                    sheet_component_const::ThreeWideKey::UpDown => {
                        up = true;
                        down = true;
                    }
                    sheet_component_const::ThreeWideKey::Up => up = true,
                    sheet_component_const::ThreeWideKey::Down => down = true,
                }

                let width = (width / WHITE_TO_BLACK).floor();
                if up {
                    let color = if note.on.unwrap_or(false) && note.naturality == Accidentals::Sharp
                    {
                        pallete::DARKER_LIGHT
                    } else {
                        pallete::DARKER_DARK
                    };
                    let start = Point2::from([position, level as f32]);
                    let end = Point2::from([position + width, level as f32]);
                    mb.line(&[start, end], 1.0, color)
                        .expect("Failed to draw black key up");
                }
                if down {
                    level += 2;
                    let color = if note.on.unwrap_or(false) && note.naturality == Accidentals::Flat
                    {
                        pallete::DARKER_LIGHT
                    } else {
                        pallete::DARKER_DARK
                    };
                    let start = Point2::from([position, level as f32]);
                    let end = Point2::from([position + width, level as f32]);
                    mb.line(&[start, end], 1.0, color)
                        .expect("Failed to draw black key down");
                }

                //
                // level += 1;
                // let start = Point2::from([0.0, level as f32]);
                // let end = Point2::from([width/4.0, level as f32]);
                // mb.line(&[start, end], 1.0, pallete::LIGHTER_DARK);
                // level += 1;
                // let start = Point2::from([0.0, level as f32]);
                // let end = Point2::from([width/4.0, level as f32]);
                // mb.line(&[start, end], 1.0, pallete::DARKER_DARK);
            }
            // mb.rectangle(
            //     DrawMode::Fill(FillOptions::default()),
            //     Rect::new(0.0, 0.0, 100.0, 100.0),
            //     Color::BLUE,
            // );
            let _mesh = mb.build();
            drawing.meshbuilder = Some(mb.to_owned());
            DrawResult::Draw(
                DrawParam::new()
                    .dest([0.0, 0.0])
                    .scale([reutil.winctx.scale, reutil.winctx.scale])
                    .z(Zindex::VirtualPiano.get()),
            )
        } else {
            DrawResult::Skip
        }
    }

    fn get_mask(&self) -> crate::models::bit_mode::BitMask {
        NOTES_MASK
    }
}
