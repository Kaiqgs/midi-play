// pub fn debug_text(vec: &Vec<String>) -> String {
//     let mut text = String::new();
//     for line in vec {
//         text.push_str(&format!("{}
//
//     }
// }

use std::path::Path;

use ggez::{
    graphics::{Color, Image},
    Context,
};
use log::debug;

use crate::models::config;

use crate::models::build_context::BuildContext;

pub fn load_cover(bctx: BuildContext, cover_filepath: String) -> Image {
    match bctx.ctx {
        Some(ctx) => {
            let cover = Image::from_path(ctx, cover_filepath);
            match cover {
                Ok(img) => img,
                Err(_) => {
                    let path = format!("/{}", config::DEFAULT_COVER_FILEPATH);
                    let path = Path::new(&path);
                    let def_cover = Image::from_path(ctx, path.clone());
                    debug!("Default cover path: {:?}", path);
                    match def_cover {
                        Ok(img) => img,
                        Err(_) => {
                            Image::from_solid(ctx, config::DEFAULT_COVER_SIZE as u32, Color::GREEN)
                        }
                    }
                }
            }
        }
        None => panic!("No context found"),
    }
}

pub fn image_from_optional(ctx: &Context, path: Option<String>) -> Image {
    match path {
        Some(filepath) => {
            let imres = Image::from_path(ctx, Path::new(&filepath));
            match imres {
                Ok(image) => image,
                Err(_) => Image::from_solid(ctx, 128, Color::GREEN),
            }
        }
        None => Image::from_solid(ctx, 128, Color::GREEN),
    }
}

// p5.prototype.map = function(n, start1, stop1, start2, stop2) {
//  return ((n-start1)/(stop1-start1))*(stop2-start2)+start2;
// };
pub fn mapf64(n: f64, start1: f64, stop1: f64, start2: f64, stop2: f64) -> f64 {
    ((n - start1) / (stop1 - start1)) * (stop2 - start2) + start2
}
