use crate::sdf;
use rand::prelude::*;

const DOUBLE_PI: f32 = 2.0 * 3.1415926;
const N: i32 = 64;
const MAX_STEP: i32 = 100;
const MAX_DISTANCE: f32 = 2.0;
const EPSILON: f32 = 1e-6;

pub struct SceneConfig {
    pub width: i32,
    pub height: i32,
}

#[allow(dead_code)]
pub enum Shape {
    Circle {
        cx: f32,
        cy: f32,
        r: f32,
    },
    Plane {
        px: f32,
        py: f32,
        nx: f32,
        ny: f32,
    },
    Segment {
        ax: f32,
        ay: f32,
        bx: f32,
        by: f32,
    },
    Capsule {
        ax: f32,
        ay: f32,
        bx: f32,
        by: f32,
        r: f32,
    },
    Rectangle {
        cx: f32,
        cy: f32,
        theta: f32,
        sx: f32,
        sy: f32,
    },
    Triangle {
        ax: f32,
        ay: f32,
        bx: f32,
        by: f32,
        cx: f32,
        cy: f32,
    },
}

impl Shape {
    fn sdf(&self, x: f32, y: f32) -> f32 {
        match self {
            Shape::Circle { cx, cy, r } => sdf::circle(x, y, *cx, *cy, *r),
            Shape::Plane { px, py, nx, ny } => sdf::plane(x, y, *px, *py, *nx, *ny),
            Shape::Segment { ax, ay, bx, by } => sdf::segment(x, y, *ax, *ay, *bx, *by),
            Shape::Capsule { ax, ay, bx, by, r } => sdf::capsule(x, y, *ax, *ay, *bx, *by, *r),
            Shape::Rectangle {
                cx,
                cy,
                theta,
                sx,
                sy,
            } => sdf::rectangle(x, y, *cx, *cy, *theta, *sx, *sy),
            Shape::Triangle {
                ax,
                ay,
                bx,
                by,
                cx,
                cy,
            } => sdf::triangle(x, y, *ax, *ay, *bx, *by, *cx, *cy),
        }
    }
}

#[derive(Clone, Copy)]
pub struct TraceResult {
    pub sd: f32,
    pub emissive: f32,
}

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
        }
    }

    pub fn complement(&self) -> TraceResult {
        TraceResult {
            sd: -self.sd,
            emissive: self.emissive,
        }
    }
}

#[allow(dead_code)]
pub enum Object {
    Shape(Shape, f32),
    Union(Box<Object>, Box<Object>),
    Intersect(Box<Object>, Box<Object>),
    Subtract(Box<Object>, Box<Object>),
    Complement(Box<Object>),
}

impl Object {
    pub fn trace(&self, x: f32, y: f32) -> TraceResult {
        match &self {
            Object::Shape(shape, emissive) => TraceResult {
                sd: shape.sdf(x, y),
                emissive: *emissive,
            },
            &Object::Union(left, right) => left.trace(x, y).union(&right.trace(x, y)),
            &Object::Intersect(left, right) => left.trace(x, y).intersect(&right.trace(x, y)),
            &Object::Subtract(left, right) => left.trace(x, y).subtract(&right.trace(x, y)),
            &Object::Complement(object) => object.trace(x, y).complement(),
        }
    }
}

pub struct Scene(pub Object);

impl Scene {
    fn trace(&self, ox: f32, oy: f32, dx: f32, dy: f32) -> f32 {
        let mut t = 0.0f32;
        let mut i: i32 = 0;
        while i < MAX_STEP && t < MAX_DISTANCE {
            let result = self.0.trace(ox + dx * t, oy + dy * t);
            if result.sd < EPSILON {
                return result.emissive;
            }
            t += result.sd;
            i += 1;
        }
        0.0
    }

    fn sample(&self, x: f32, y: f32, rng: &mut rand::rngs::ThreadRng) -> f32 {
        (0..N)
            .map(|i| DOUBLE_PI * (i as f32 + rng.gen::<f32>()) / (N as f32))
            .fold(0f32, |sum, a| sum + self.trace(x, y, a.cos(), a.sin()))
            / (N as f32)
    }

    pub fn render(&self, config: &SceneConfig) -> Vec<u8> {
        let mut rng = rand::thread_rng();
        (0..config.height)
            .flat_map(|y| {
                (0..config.width).map(move |x| {
                    (255.0
                        * self
                            .sample(
                                (x as f32) / (config.width as f32),
                                (y as f32) / (config.height as f32),
                                &mut rng,
                            )
                            .min(1.0)) as u8
                })
            })
            .collect::<Vec<u8>>()
    }
}
