use crate::*;

#[derive(Clone, Copy)]
struct SceneResult {
    sd: f32,
    emissive: f32,
}

impl SceneResult {
    fn union(&self, shape: &SceneResult) -> SceneResult {
        if self.sd < shape.sd {
            self.clone()
        } else {
            shape.clone()
        }
    }

    fn intersect(&self, shape: &SceneResult) -> SceneResult {
        SceneResult {
            sd: if self.sd > shape.sd { self.sd } else { shape.sd },
            emissive: if self.sd > shape.sd { shape.emissive } else { self.emissive },
        }
    }

    fn subtract(&self, shape: &SceneResult) -> SceneResult {
        SceneResult {
            sd: if self.sd > -shape.sd { self.sd } else { -shape.sd },
            emissive: self.emissive,
        }
    }

    fn complement(&self) -> SceneResult {
        SceneResult {
            sd: -self.sd,
            emissive: self.emissive,
        }
    }
}

fn scene(x: f32, y: f32) -> SceneResult {
    let a = SceneResult {
        sd: circle_sdf(x, y, 0.4, 0.5, 0.2),
        emissive: 1.0,
    };
    let b = SceneResult {
        sd: circle_sdf(x, y, 0.6, 0.5, 0.2),
        emissive: 0.8,
    };
    a.union(&b)
}

fn trace(ox: f32, oy: f32, dx: f32, dy: f32) -> f32 {
    let mut t = 0.0f32;
    let mut i: i32 = 0;
    while i < MAX_STEP && t < MAX_DISTANCE {
        let result = scene(ox + dx * t, oy + dy * t);
        if result.sd < EPSILON {
            return result.emissive;
        }
        t += result.sd;
        i += 1;
    }
    0.0
}

pub fn sample(x: f32, y: f32, rng: &mut rand::rngs::ThreadRng) -> f32 {
    (0..N)
        .map(|i| DOUBLE_PI * (i as f32 + rng.gen::<f32>()) / (N as f32))
        .fold(0f32, |sum, a| sum + trace(x, y, a.cos(), a.sin()))
        / (N as f32)
}
