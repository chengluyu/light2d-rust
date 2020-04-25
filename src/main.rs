#[macro_use]
extern crate clap;
extern crate png;
extern crate rand;

mod csg;
mod scene;
mod sdf;
mod shapes;

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

fn trace(ox: f32, oy: f32, dx: f32, dy: f32) -> f32 {
    let mut t = 0.0f32;
    let mut i: i32 = 0;
    while i < MAX_STEP && t < MAX_DISTANCE {
        let sd = sdf::circle(ox + dx * t, oy + dy * t, 0.5, 0.5, 0.1);
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

fn basic(path: &Path) {
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
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);
    let mut encoder = png::Encoder::new(w, WIDTH as u32, HEIGHT as u32);
    encoder.set_color(png::ColorType::Grayscale);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(data.as_slice()).unwrap();
}

fn csg(path: &Path) {
    use crate::csg::sample;

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
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);
    let mut encoder = png::Encoder::new(w, WIDTH as u32, HEIGHT as u32);
    encoder.set_color(png::ColorType::Grayscale);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(data.as_slice()).unwrap();
}

fn shapes(path: &Path) {
    use crate::shapes::sample;

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
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);
    let mut encoder = png::Encoder::new(w, WIDTH as u32, HEIGHT as u32);
    encoder.set_color(png::ColorType::Grayscale);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(data.as_slice()).unwrap();
}

fn scene(path: &Path) {
    use crate::scene::*;

    let config = SceneConfig {
        width: WIDTH * 2,
        height: HEIGHT * 2,
    };
    let scene = Scene(
        (0..10)
            .map(|i| (i as f32) / 10.0)
            .map(|x| (x, x * f32::consts::PI * 2.0))
            .map(|(emissive, angle)| {
                Object::Shape(
                    Shape::Circle {
                        cx: 0.5 + angle.cos() * 0.4,
                        cy: 0.5 + angle.sin() * 0.4,
                        r: 0.05,
                    },
                    emissive,
                )
            })
            .fold(
                Object::Shape(
                    Shape::Circle {
                        cx: 0.5,
                        cy: 0.5,
                        r: 0.1,
                    },
                    1.0,
                ),
                |left, right| Object::Union(Box::new(left), Box::new(right)),
            ),
    );
    let data = scene.render(&config);

    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);
    let mut encoder = png::Encoder::new(w, config.width as u32, config.height as u32);
    encoder.set_color(png::ColorType::Grayscale);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(data.as_slice()).unwrap();
}

fn main() {
    let matches = clap_app!(light2d =>
        (version: "1.0.0")
        (about: "Milo Yip's light2d in Rust")
        (@arg output: -o --output +takes_value "Specify the path of the output image")
        (@subcommand basic =>
            (about: "Run basic light simulation")
        )
        (@subcommand csg =>
            (about: "Constructive solid geometry")
        )
        (@subcommand shapes =>
            (about: "Different shapes")
        )
        (@subcommand scene =>
            (about: "Scene API")
        )
    )
    .get_matches();

    if let Some(subcommand) = matches.subcommand_name() {
        let default_path = Path::new("output").join(format!("{}.png", subcommand));
        let path = matches
            .value_of("output")
            .map(Path::new)
            .unwrap_or(default_path.as_path());
        match subcommand {
            "basic" => basic(&path),
            "csg" => csg(&path),
            "shapes" => shapes(&path),
            "scene" => scene(&path),
            _ => println!("Unknown command: {}", subcommand),
        }
    } else {
        println!("Please supply a command");
    }
}
