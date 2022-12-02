use ggez::event::EventHandler;
use ggez::graphics::{self, Color, DrawParam, Mesh, MeshBuilder, TextFragment};
use ggez::mint::Point2;
use ggez::{Context, GameResult};

use crate::models::{self};

pub struct AskDialog {}

impl AskDialog {
    pub fn new() -> Self {
        AskDialog {}
    }
}

pub struct MidiPlay {
    // Your state here...
    width: u32,
    height: u32,
    pause: bool,
}

impl MidiPlay {
    pub fn new(width: u32, height: u32) -> MidiPlay {
        // Load/create resources such as images here.
        

        MidiPlay {
            width,
            height,
            pause: false,
        }
    }

    pub async fn pick_track(&mut self, filepath: &str) -> bool {
        unimplemented!();
    }

    pub async fn quit(&mut self) -> bool {
        unimplemented!();
    }
}

impl models::Pausable for MidiPlay {
    fn pause(&mut self) -> bool {
        let success = !self.pause;
        self.pause = true;
        success
    }

    fn resume(&mut self) -> bool {
        let success = self.pause;
        self.pause = false;
        success
    }
}

impl EventHandler for MidiPlay {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // Update code here...
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::new(0.25, 0.25, 0.25, 1.0));
        let welcoming = graphics::Text::new(TextFragment::new("Hello World"));
        // let line1 = graphics::Rect::new(0.0, 5.0, 100.0, 1.0 );
        let mb = &mut MeshBuilder::new();

        let note_count = 88;
        let note_length: f32 = 3.0;

        for i in 0..note_count {
            let y = f32::from(i as i16) * note_length;
            let p0: Point2<f32> = { Point2 { x: 0.0, y: y } };
            let p1: Point2<f32> = {
                Point2 {
                    x: f32::from(self.width as u16),
                    y: y,
                }
            };
            mb.line(&[p0, p1], 1.0, Color::BLACK)
                .expect("error creating line");
        }

        let mesh = Mesh::from_data(ctx, mb.build());
        // canvas.draw(drawable, param)
        // Draw code here...

        //canvas.draw(&line1);

        canvas.draw(&mesh, DrawParam::new());
        canvas.draw(&welcoming, DrawParam::new());
        canvas.finish(ctx)
    }
}
