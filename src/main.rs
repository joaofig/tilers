mod localdb;
mod models;
mod qk_tools;

#[macro_use] extern crate rocket;
extern crate geo_quadkey_rs;

use std::io::Result;
use std::fs::create_dir_all;
use std::path::{PathBuf, Path};
use rocket::fs::NamedFile;
use image::{ImageBuffer, Rgba, RgbaImage};
use geo_quadkey_rs::Quadkey;
use colorgrad::{Color, Gradient, CustomGradient};
use crate::localdb::{get_level_range, get_tile_quad_keys};
use crate::models::{TileQuadKey, Point, TilePixel, LevelRange};


fn get_color_gradient() -> Gradient {
    let grad = CustomGradient::new().colors(
        &[Color::from_rgba8(255, 255, 0, 127),
          Color::from_rgba8(255, 0, 0, 127)]
    ).build().unwrap();
    grad
}


fn make_rgba_tile(r: u8, g: u8, b: u8, a: u8) -> RgbaImage {
    ImageBuffer::from_pixel(256, 256, Rgba([r, g, b, a]))
}


fn paint_tile(pixels: Vec<TilePixel>, 
              color_grad: Gradient,
              range: LevelRange) -> RgbaImage {
    let mut bitmap = make_rgba_tile(0, 0, 0, 0);

    if range.width() > 0.0 {
        for pixel in pixels {
            let intensity = (pixel.get_c() - range.min()).ln() / range.width().ln();
            let rgba = color_grad.at(intensity);
            let pix = Rgba::from(rgba.to_rgba8());
            bitmap.put_pixel(pixel.get_x() as u32,
                             pixel.get_y() as u32,
                             pix);
        }
    } else {
        let rgba = color_grad.at(1.0);
        let pix = Rgba::from(rgba.to_rgba8());
        for pixel in pixels {
            bitmap.put_pixel(pixel.get_x() as u32, 
                             pixel.get_y() as u32,
                             pix);
        }
    }
    bitmap
}


fn get_default_filename() -> String {
    let file_name = String::from("./tiles/default.png");
    let path = Path::new(&file_name);
    if !path.exists() {
        let dir = "./tiles";
        if !Path::new(&dir).exists() && std::fs::create_dir_all(&dir).is_ok() {
            let image = make_rgba_tile(0u8, 0u8, 0u8, 0u8);
            image.save(&file_name).unwrap();
        }
    }
    file_name
}


fn get_tile_pixels(qk: &str, tile_quad_keys: Vec<TileQuadKey>) -> Vec<TilePixel> {
    let lhc = Point::from_quad_key_str(qk);
    let mut points: Vec<TilePixel> = Vec::new();
    let zoom: i32 = qk.len() as i32 + 8;

    for tile_quad_key in tile_quad_keys {
        let point = tile_quad_key.to_point(zoom);
        points.push(TilePixel::from_point(&point.div(256).sub(&lhc),
                                          tile_quad_key.get_intensity()));
    }
    points
}


async fn get_tile_file_name(x: i32, y: i32, z:i32) -> String {
    let quad_key = Quadkey::tile_to_quadkey(x, y, z as usize);
    let mut file_name = format!("./tiles/{}/{}.png", z, quad_key);
    let path = Path::new(&file_name);

    if !path.exists() {
        let tile_quad_keys = get_tile_quad_keys(&quad_key).await;

        if tile_quad_keys.is_empty() {
            file_name = get_default_filename();
        } else {
            let tile_quad_keys = get_tile_quad_keys(&quad_key).await;

            if tile_quad_keys.is_empty() {
                file_name = get_default_filename();
            } else {
                let tile_pixels = get_tile_pixels(&quad_key, tile_quad_keys);
                let range = get_level_range(quad_key.len() as i32 + 8).await;
                let picture = paint_tile(tile_pixels, 
                                                    get_color_gradient(),
                                                    range);
                let folder_name = format!("./tiles/{}", z);
                create_dir_all(&folder_name).unwrap();
                picture.save(&file_name).unwrap();
            }
        }
    }
    file_name
}


#[get("/")]
fn index() -> &'static str {
    "VED Tile Server: Use /density/x/y/z to generate tiles."
}

#[get("/density/<x>/<y>/<z>")]
async fn get_density_tile(x: i32, y: i32, z: i32) -> Result<NamedFile> {
    let file_name = if !(1..=18).contains(&z) {
        get_default_filename()
    } else {
        get_tile_file_name(x, y, z).await
    };
    NamedFile::open(PathBuf::from(file_name)).await
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, get_density_tile])
}
