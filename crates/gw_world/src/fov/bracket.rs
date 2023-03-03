use super::{FovSource, FovTarget};
use crate::prelude::*;
// use bracket_lib::prelude::{field_of_view, Algorithm2D, BaseMap, SmallVec};

struct BracketSource<'a, S: FovSource + 'a> {
    source: &'a S,
}

impl<'a, S> BracketSource<'a, S>
where
    S: FovSource + 'a,
{
    fn new(source: &'a S) -> Self {
        BracketSource { source }
    }

    fn to_point(&self, idx: usize) -> Point {
        let size = self.source.get_size();
        let x = idx as i32 % size.0 as i32;
        let y = idx as i32 / size.0 as i32;
        Point::new(x, y)
    }

    fn to_idx(&self, x: i32, y: i32) -> Option<usize> {
        let size = self.source.get_size();
        if x < 0 || (x as u32) >= size.0 || y < 0 || (y as u32) >= size.1 {
            return None;
        }
        Some(x as usize + (y as u32 * size.0) as usize)
    }
}

impl<'a, S> BaseMap for BracketSource<'a, S>
where
    S: FovSource + 'a,
{
    fn is_opaque(&self, idx: usize) -> bool {
        let p = self.to_point(idx);
        self.source.is_opaque(p.x, p.y)
    }

    fn get_available_exits(&self, idx: usize) -> SmallVec<[(usize, f32); 10]> {
        let mut res = SmallVec::new();

        let pt = self.to_point(idx);

        for dir in DIRS.iter() {
            let x = pt.x + dir.x;
            let y = pt.y + dir.y;
            if let Some(idx) = self.to_idx(x, y) {
                if !self.source.is_opaque(x, y) {
                    res.push((idx, 1.0));
                }
            }
        }

        res
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        let a = self.to_point(idx1);
        let b = self.to_point(idx2);
        a.distance(&b)
    }
}

impl<'a, S> Algorithm2D for BracketSource<'a, S>
where
    S: FovSource + 'a,
{
    fn dimensions(&self) -> Point {
        let size = self.source.get_size();
        Point::new(size.0, size.1)
    }
}

pub fn calculate_fov<S: FovSource, T: FovTarget>(
    source: &S,
    origin: Point,
    radius: u32,
    target: &mut T,
) {
    let size = source.get_size();
    target.reset(size.0, size.1);

    let bracket_source = BracketSource::new(source);
    for point in field_of_view(origin, radius as i32, &bracket_source).iter() {
        target.set_visible(point.x, point.y, 1.0);
    }
}
