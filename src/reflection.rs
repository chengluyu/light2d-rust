use crate::sdf;
use crate::MAX_DISTANCE;
use crate::MAX_STEP;
use crate::N;
use rand::prelude::*;
use std::f32;

#[derive(Clone, Copy)]
pub struct TraceResult {
    pub sd: f32,
    pub emissive: f32,
    pub reflectivity: f32,
}

#[allow(dead_code)]
impl TraceResult {
    pub fn union(&self, shape: &TraceResult) -> TraceResult {
        if self.sd < shape.sd {
            self.clone()
        } else {
            shape.clone()
        }
    }

    pub fn intersect(&self, shape: &TraceResult) -> TraceResult {
        TraceResult {
            sd: if self.sd > shape.sd {
                self.sd
            } else {
                shape.sd
            },
            emissive: if self.sd > shape.sd {
                shape.emissive
            } else {
                self.emissive
            },
            reflectivity: if self.sd > shape.sd {
                shape.reflectivity
            } else {
                self.reflectivity
            },
        }
    }

    pub fn subtract(&self, shape: &TraceResult) -> TraceResult {
        TraceResult {
            sd: if self.sd > -shape.sd {
                self.sd
            } else {
                -shape.sd
            },
            emissive: self.emissive,
            reflectivity: self.reflectivity,
        }
    }

    pub fn complement(&self) -> TraceResult {
        TraceResult {
            sd: -self.sd,
            emissive: self.emissive,
            reflectivity: self.reflectivity,
        }
    }
}

fn scene(x: f32, y: f32) -> TraceResult {
    let a = TraceResult {
        sd: sdf::circle(x, y, 0.4, 0.2, 0.1),
        emissive: 2.0,
        reflectivity: 0.0,
    };
    let b = TraceResult {
        sd: sdf::rectangle(x, y, 0.5, 0.8, 2.0 * f32::consts::PI / 16.0, 0.1, 0.1),
        emissive: 0.0,
        reflectivity: 0.9,
    };
    let c = TraceResult {
        sd: sdf::rectangle(x, y, 0.8, 0.5, 2.0 * f32::consts::PI / 16.0, 0.1, 0.1),
        emissive: 0.0,
        reflectivity: 0.9,
    };
    a.union(&b.union(&c))
}

fn reflect(ix: f32, iy: f32, nx: f32, ny: f32) -> (f32, f32) {
    let idotn2 = (ix * nx + iy * ny) * 2.0;
    (ix - idotn2 * nx, iy - idotn2 * ny)
}

fn gradient(x: f32, y: f32) -> (f32, f32) {
    (
        (scene(x + f32::EPSILON, y).sd - scene(x - f32::EPSILON, y).sd) * (0.5 / f32::EPSILON),
        (scene(x, y + f32::EPSILON).sd - scene(x, y - f32::EPSILON).sd) * (0.5 / f32::EPSILON),
    )
}

const BIAS: f32 = 1e-4;
const MAX_DEPTH: i32 = 10;

fn trace(ox: f32, oy: f32, dx: f32, dy: f32, depth: i32) -> f32 {
    let mut t = 0.0;
    let mut i = 0;
    while i < MAX_STEP && t < MAX_DISTANCE {
        let x = ox + dx * t;
        let y = oy + dy * t;
        let result = scene(x, y);
        if result.sd < f32::EPSILON {
            return result.emissive
                + if depth < MAX_DEPTH && result.reflectivity > 0.0 {
                    let (nx, ny) = gradient(x, y);
                    let (rx, ry) = reflect(dx, dy, nx, ny);
                    result.reflectivity * trace(x + nx * BIAS, y + ny * BIAS, rx, ry, depth + 1)
                } else {
                    0.0
                };
        }
        i += 1;
        t += result.sd;
    }
    0.0
}

pub fn sample(x: f32, y: f32, rng: &mut rand::rngs::ThreadRng) -> f32 {
    (0..N)
        .map(|i| f32::consts::PI * 2.0 * (i as f32 + rng.gen::<f32>()) / (N as f32))
        .map(|a| trace(x, y, a.cos(), a.sin(), 0))
        .sum::<f32>()
        / (N as f32)
}
