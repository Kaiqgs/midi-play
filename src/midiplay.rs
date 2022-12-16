use std::ops::Deref;
use std::rc::Rc;
use std::sync::Arc;

use ggez::event::EventHandler;
use ggez::graphics::{self, Canvas, Color, DrawParam, Drawable, Mesh};

use ggez::mint::Point2;
use ggez::{Context, GameResult};

use crate::components::component::{BuildContext, Component, ComponentObject, RenderUtilObject};
use crate::components::draw_util::DrawUtilGG;
use crate::components::drawing::Drawing;
use crate::components::pallete;
use crate::components::sheet::{definition, staff, track};

use crate::components::sheet::staff_system::StaffSystemComponentData;
use crate::models;
use crate::models::draw_util::DrawUtil;
use crate::models::midi::to_sheet::MidiSheetTransformer;
use crate::models::note::Note;
use crate::models::pausable::Pausable;

use crate::models::render_util::MockRenderUtil;
use crate::models::sheet::staff::Staff;
use crate::models::sheet::staff_system::StaffSystem;
use crate::models::track_manager::TrackManager;
pub struct MidiPlay {
    track: TrackManager<MidiSheetTransformer>,
    pause: bool,
}

impl MidiPlay {
    pub fn new(build: BuildContext) -> Self {
        // Load/create resources such as images here.

        let track = TrackManager::new("".to_owned(), MidiSheetTransformer::new(), build);

        MidiPlay {
            track,
            pause: false,
        }
    }

    pub async fn pick_track(&mut self, filepath: &str) -> bool {
        unimplemented!();
    }

    pub async fn quit(&mut self) -> bool {
        unimplemented!();
    }

    pub fn draw_component(
        &self,
        component: ComponentObject,
        canvas: RenderUtilObject,
        gfx: &Context,
        screen: &mut Canvas,
    ) {
        //let bcanvas = Box::new(canvas);
        let dresult = component.draw(canvas);
        let drawing = dresult.drawing;

        match drawing.mesh.as_ref() {
            Some(mesh) => {
                screen.draw(mesh, dresult.params);
            }
            None => match drawing.meshbuilder.as_ref() {
                Some(mb) => {
                    println!("Warning: costly mesh generation;");
                    let mesh = Mesh::from_data(gfx, mb.build());
                    screen.draw(&mesh, dresult.params);
                }
                None => (),
            },
        }

        match drawing.image.as_ref() {
            Some(image) => {
                screen.draw(
                    image,
                    dresult.params

                );
            }
            None => (),
        }

        // if dresult.drawing.is_some_image() {

        // }
    }
}

impl Pausable for MidiPlay {
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
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let width = ctx.gfx.window().outer_size().width;
        let height = ctx.gfx.window().outer_size().height;

        let dgg = MockRenderUtil::new();
        let rcanvas = RenderUtilObject::new(&dgg);
        // let rcanvas: Rc<RenderUtilObject> = Rc::new(&dgg);
        let mut canvas = graphics::Canvas::from_frame(ctx, pallete::LIGHTER_LIGHT);
        canvas.set_sampler(graphics::Sampler::nearest_clamp());

        let bsheet: ComponentObject = Arc::new(&self.track.sheet);
        let mut stack: Vec<ComponentObject> = vec![bsheet];
        let mut counter = 0;
        while !stack.is_empty() {
            let comp: ComponentObject = stack.pop().expect("err negative index");
            let next = comp.next();
            if next.len() > 0 {
                stack.extend(next);
            }
            self.draw_component(comp.clone(), rcanvas.clone(), ctx, &mut canvas);
            counter += 1;
        }
        println!("Updated[{}]", counter);
        canvas.finish(ctx)
    }
}
