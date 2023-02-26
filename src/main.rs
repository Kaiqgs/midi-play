use ggez::event;
use ggez::mint::Point2;
use ggez::ContextBuilder;
use midiplaylib::components::component::BuildContext;
use std::path;

fn main() {
    env_logger::builder()
        .format_timestamp(None)
        .format_module_path(false)
        .format_target(false)
        .init();

    log::info!("Initted...");
    // Make a Context.
    let (ctx, event_loop) = ContextBuilder::new("midi play", "kags")
        .add_resource_path(path::PathBuf::from("./resources"))
        .build()
        .expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let width = ctx.gfx.window().inner_size().width;
    let height = ctx.gfx.window().inner_size().height;
    let buildctx = BuildContext::new(Some(&ctx), Point2::from([width, height]), None);
    let midi_play = midiplaylib::MidiPlay::new(buildctx, None);
    event::run(ctx, event_loop, midi_play);
}
