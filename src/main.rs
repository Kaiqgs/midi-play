use ggez::event;
use ggez::graphics::{FontData, Image};
use ggez::mint::Point2;
use ggez::ContextBuilder;
use midiplaylib::models::build_context::BuildContext;
use midiplaylib::models::config;
use midiplaylib::models::window_context::WindowContext;
use std::path::{self, Path};

fn main() -> ggez::GameResult {
    env_logger::builder()
        .format_timestamp(None)
        .format_module_path(false)
        .format_target(false)
        .init();

    log::info!("Initted...");
    // Make a Context.
    let resources_path = path::PathBuf::from(config::RELATIVE_RESOURCES);
    log::warn!("Resources path: {:?}", resources_path.display());
    let (mut ctx, event_loop) = ContextBuilder::new("midi play", "kags")
        .add_resource_path(resources_path)
        .window_mode(
            ggez::conf::WindowMode::default()
                .dimensions(800.0, 600.0)
                .resizable(true),
        )
        .build()
        .expect("aieee, could not create ggez context!");

    let fontpath = "/fontface.ttf";
    let font = FontData::from_path(&ctx, fontpath).expect("Could not load font");

    ctx.gfx
        .add_font(midiplaylib::components::constants::DEFAULT_FONT, font);
    let width = ctx.gfx.window().inner_size().width;
    let height = ctx.gfx.window().inner_size().height;
    // warn!("Resources folder?");
    let resources_folder: String = ctx.fs.resources_dir().to_str().expect("Tostr").into();
    // TODO: fix B#3;
    let resources_folder = resources_folder[4..].to_string();
    match std::fs::create_dir(resources_folder.clone()) {
        Ok(_) => (),
        Err(_) => (),
    }

    let cover_path = Path::new(config::DEFAULT_COVER_FILEPATH.into());
    let default_cover = Some(Image::from_path(&mut ctx, cover_path)?);
    let winctx = WindowContext::new(
        Point2::from([width, height]),
        None,
        None,
        None,
        None,
        None,
        Some(resources_folder),
        default_cover,
    );
    let buildctx = BuildContext::new(Some(&ctx), winctx);
    let midi_play = midiplaylib::models::midiplay::MidiPlay::new(buildctx);

    // event_loop.run(move |event, _, control_flow| control_flow.set_wait());

    event::run(ctx, event_loop, midi_play);
}
