extern crate png;
extern crate rand;

use rand::prelude::*;
use std::f32;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

const DOUBLE_PI: f32 = 2.0 * 3.1415926;
const WIDTH: i32 = 512;
const HEIGHT: i32 = 512;
const N: i32 = 64;
const MAX_STEP: i32 = 100;
const MAX_DISTANCE: f32 = 2.0;
const EPSILON: f32 = 1e-6;

fn circle_sdf(x: f32, y: f32, cx: f32, cy: f32, r: f32) -> f32 {
    let ux = x - cx;
    let uy = y - cy;
    (ux * ux + uy * uy).sqrt() - r
}

fn trace(ox: f32, oy: f32, dx: f32, dy: f32) -> f32 {
    let mut t = 0.0f32;
    let mut i: i32 = 0;
    while i < MAX_STEP && t < MAX_DISTANCE {
        let sd = circle_sdf(ox + dx * t, oy + dy * t, 0.5, 0.5, 0.1);
        if sd < EPSILON {
            return 2.0;
        }
        t += sd;
        i += 1;
    }
    0.0
}

fn sample(x: f32, y: f32, rng: &mut rand::rngs::ThreadRng) -> f32 {
    (0..N)
        .map(|i| DOUBLE_PI * (i as f32 + rng.gen::<f32>()) / (N as f32))
        .fold(0f32, |sum, a| sum + trace(x, y, a.cos(), a.sin()))
        / (N as f32)
}

fn main() {
    let mut rng = rand::thread_rng();

    let data = (0..HEIGHT)
        .flat_map(|y| {
            (0..WIDTH).map(move |x| {
                (255.0
                    * sample(
                        (x as f32) / (WIDTH as f32),
                        (y as f32) / (HEIGHT as f32),
                        &mut rng,
                    )
                    .min(1.0)) as u8
            })
        })
        .collect::<Vec<u8>>();

    let path = Path::new("basic.png");
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);
    let mut encoder = png::Encoder::new(w, WIDTH as u32, HEIGHT as u32);
    encoder.set_color(png::ColorType::Grayscale);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(data.as_slice()).unwrap();
}
