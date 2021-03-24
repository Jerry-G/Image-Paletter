extern crate image;
use image::{GenericImageView, ImageBuffer, Pixel, Rgb, RgbImage};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

// struct for parsing the json palette
#[derive(Serialize, Deserialize)]
struct PaletteJSON {
    colors: Vec<String>,
}

// Calculate the distance between two Rbg values
// Note: this is a numerical distance and does not account for how humans
// see colors.
fn dist(c1: &image::Rgb<u8>, c2: &image::Rgb<u8>) -> f64 {
    let r1: f64 = (c1.0[0]) as f64;
    let g1: f64 = (c1.0[1]) as f64;
    let b1: f64 = (c1.0[2]) as f64;
    let r2: f64 = (c2.0[0]) as f64;
    let g2: f64 = (c2.0[1]) as f64;
    let b2: f64 = (c2.0[2]) as f64;

    return f64::sqrt((r2 - r1) * (r2 - r1) + (g2 - g1) * (g2 - g1) + (b2 - b1) * (b2 - b1));
}

// Take in a string that has a hex rgb value and return an Rgb
fn hex_to_rgb(str: &str) -> Result<Rgb<u8>, String> {
    // expect the color to be exactly 6 chars in length
    // a color with the hash symbol ( #FFFFFF ) will not be accepted
    if str.len() != 6 {
        return Err(format!("unable to interpret {:?} as color", &str));
    }

    // inelegant and not very fast way to turn color string into 3 u8s
    let mut r = "".to_owned();
    let mut g = "".to_owned();
    let mut b = "".to_owned();
    for (i, c) in str.chars().enumerate() {
        if i <= 1 {
            r.push_str(&c.to_string());
        } else if i <= 3 {
            g.push_str(&c.to_string());
        } else if i <= 5 {
            b.push_str(&c.to_string());
        }
    }
    let red = match u8::from_str_radix(&r, 16) {
        Err(_) => return Err(format!("unable to interpret {:?} as color", &str)),
        Ok(val) => val,
    };
    let grn = match u8::from_str_radix(&g, 16) {
        Err(_) => return Err(format!("unable to interpret {:?} as color", &str)),
        Ok(val) => val,
    };
    let blu = match u8::from_str_radix(&b, 16) {
        Err(_) => return Err(format!("unable to interpret {:?} as color", &str)),
        Ok(val) => val,
    };

    return Ok(Rgb([red, grn, blu]));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // expect exactly 4 args
    if args.len() != 4 {
        println!("Usage: paletter palette.json input.png output.png");
        std::process::exit(0);
    }

    // images
    let og = image::open(Path::new(&args[2])).unwrap();
    let mut img: RgbImage = ImageBuffer::new(og.dimensions().0, og.dimensions().1);

    let path = Path::new(&args[1]);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut data = String::new();
    match file.read_to_string(&mut data) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => (),
    }

    // parse the palette json
    let json: PaletteJSON = match serde_json::from_str(&data) {
        Err(why) => panic!("couldn't understand {} file: {}", display, why),
        Ok(val) => val,
    };

    // convert the parsed palette into native Rgb types
    let mut palette: Vec<Rgb<u8>> = Vec::new();
    for color in json.colors.iter() {
        let c: Rgb<u8> = match hex_to_rgb(color) {
            Err(why) => panic!("error when reading colors, {}", why),
            Ok(val) => val,
        };
        palette.push(c);
    }

    // for each pixel calculate the dist to each color in the palette,
    // inefficient, but not too slow in practice, could be improved
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let mut tmp_color = palette[0];
        let ogp = og.get_pixel(x, y).to_rgb();
        let mut tmp_dist = dist(&tmp_color, &ogp);
        for c in palette.iter() {
            let d = dist(&c, &ogp);
            if d < tmp_dist {
                tmp_dist = d;
                tmp_color = *c;
            }
        }
        *pixel = tmp_color;
    }

    // this fails if you try to write to a dir that does not exist
    img.save(Path::new(&args[3])).unwrap();
}
