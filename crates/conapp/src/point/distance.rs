use super::Point;

pub fn simple(a: &Point, b: &Point) -> f32 {
    let dx = (a.x - b.x).abs() as f32;
    let dy = (a.y - b.y).abs() as f32;
    if dx > dy {
        dx + dy * 0.42
    } else {
        dy + dx * 0.42
    }
}

pub fn diagonal(a: &Point, b: &Point) -> f32 {
    let dx = (a.x - b.x).abs() as f32;
    let dy = (a.y - b.y).abs() as f32;
    if dx > dy {
        dx
    } else {
        dy
    }
}

pub fn manhattan(a: &Point, b: &Point) -> f32 {
    let dx = (a.x - b.x).abs() as f32;
    let dy = (a.y - b.y).abs() as f32;
    dx + dy
}

pub fn precise(a: &Point, b: &Point) -> f32 {
    let dx = (a.x - b.x).abs() as f32;
    let dy = (a.y - b.y).abs() as f32;
    (dx.powi(2) + dy.powi(2)).sqrt()
}

pub fn squared(a: &Point, b: &Point) -> f32 {
    let dx = (a.x - b.x).abs() as f32;
    let dy = (a.y - b.y).abs() as f32;
    dx.powi(2) + dy.powi(2)
}
