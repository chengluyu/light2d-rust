pub fn circle(x: f32, y: f32, cx: f32, cy: f32, r: f32) -> f32 {
    let ux = x - cx;
    let uy = y - cy;
    (ux * ux + uy * uy).sqrt() - r
}

pub fn plane(x: f32, y: f32, px: f32, py: f32, nx: f32, ny: f32) -> f32 {
    (x - px) * nx + (y - py) * ny
}

pub fn segment(x: f32, y: f32, ax: f32, ay: f32, bx: f32, by: f32) -> f32 {
    let vx = x - ax;
    let vy = y - ay;
    let ux = bx - ax;
    let uy = by - ay;
    let t = ((vx * ux + vy * uy) / (ux * ux + uy * uy))
        .min(1.0)
        .max(0.0);
    let dx = vx - ux * t;
    let dy = vy - uy * t;
    (dx * dx + dy * dy).sqrt()
}

pub fn capsule(x: f32, y: f32, ax: f32, ay: f32, bx: f32, by: f32, r: f32) -> f32 {
    segment(x, y, ax, ay, bx, by) - r
}

pub fn rectangle(x: f32, y: f32, cx: f32, cy: f32, theta: f32, sx: f32, sy: f32) -> f32 {
    let cos_theta = theta.cos();
    let sin_theta = theta.sin();
    let dx = ((x - cx) * cos_theta + (y - cy) * sin_theta).abs() - sx;
    let dy = ((y - cy) * cos_theta - (x - cx) * sin_theta).abs() - sy;
    let ax = dx.max(0.0);
    let ay = dy.max(0.0);
    dx.max(dy).min(0.0) + (ax * ax + ay * ay).sqrt()
}

pub fn triangle(x: f32, y: f32, ax: f32, ay: f32, bx: f32, by: f32, cx: f32, cy: f32) -> f32 {
    let d = segment(x, y, ax, ay, bx, by)
        .min(segment(x, y, bx, by, cx, cy))
        .min(segment(x, y, cx, cy, ax, ay));
    if (bx - ax) * (y - ay) > (by - ay) * (x - ax)
        && (cx - bx) * (y - by) > (cy - by) * (x - bx)
        && (ax - cx) * (y - cy) > (ay - cy) * (x - cx)
    {
        -d
    } else {
        d
    }
}
