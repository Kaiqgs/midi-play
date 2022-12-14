use std::path;

use ggez::event::{self};
use ggez::mint::Point2;
use ggez::ContextBuilder;
use midiplaylib::components::component::BuildContext;

fn main() {
    // Make a Context.
    let (ctx, event_loop) = ContextBuilder::new("midi play", "kags")
        .add_resource_path(path::PathBuf::from("./resources"))
        .build()
        .expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let width = ctx.gfx.window().outer_size().width;
    let height = ctx.gfx.window().outer_size().height;
    let buildctx = BuildContext::new(Some(&ctx), Point2::from([width, height]));
    let my_game = midiplaylib::MidiPlay::new(buildctx);

    // Run!
    event::run(ctx, event_loop, my_game);
}
