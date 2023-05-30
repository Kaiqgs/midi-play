use ggez::event;
use ggez::graphics::FontData;
use ggez::mint::Point2;
use ggez::ContextBuilder;
use midiplaylib::models::build_context::BuildContext;
use midiplaylib::models::config;
use midiplaylib::models::window_context::WindowContext;
use std::path;


fn main() {
    env_logger::builder()
        .format_timestamp(None)
        .format_module_path(false)
        .format_target(false)
        .init();

    log::info!("Initted...");
    // Make a Context.
    let (mut ctx, event_loop) = ContextBuilder::new("midi play", "kags")
        .add_resource_path(path::PathBuf::from(config::RELATIVE_RESOURCES))
        .window_mode(
            ggez::conf::WindowMode::default()
                .dimensions(800.0, 600.0)
                .resizable(true),
        )
        .build()
        .expect("aieee, could not create ggez context!");

    let path = "/fontface.ttf";
    // let mut fs = ctx.fs.open(path).expect("Could not open font file");
    let font = FontData::from_path(&ctx, path).expect("Could not load font");
    ctx.gfx.add_font(midiplaylib::components::constants::DEFAULT_FONT, font);
    // ctx.gfx.window().
    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let width = ctx.gfx.window().inner_size().width;
    let height = ctx.gfx.window().inner_size().height;
    let winctx = WindowContext::new(Point2::from([width, height]), None, None, None, None, None);
    let buildctx = BuildContext::new(Some(&ctx), winctx);
    let midi_play = midiplaylib::models::midiplay::MidiPlay::new(buildctx);

    // event_loop.run(move |event, _, control_flow| control_flow.set_wait());

    event::run(ctx, event_loop, midi_play);
}
