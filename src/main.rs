use ggez::event::{self};
use ggez::ContextBuilder;

fn main() {
    // Make a Context.
    let (ctx, event_loop) = ContextBuilder::new("midi play", "kags")
        .build()
        .expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let width = ctx.gfx.window().outer_size().width;
    let height = ctx.gfx.window().outer_size().height;
    let my_game = midiplaylib::MidiPlay::new(width, height);

    // Run!
    event::run(ctx, event_loop, my_game);
}
