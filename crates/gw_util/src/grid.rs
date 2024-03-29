use std::{fmt::Display, iter::Enumerate, slice::Iter};

use crate::{
    point::{Point, DIRS4},
    rect::Rect,
    rng::RandomNumberGenerator,
    xy::Wrap,
};

#[derive(Debug, Clone)]
pub struct Grid<T: Copy + Default> {
    size: (usize, usize),
    data: Vec<T>,
}

impl<T> Grid<T>
where
    T: Copy + Default,
{
    pub fn new(width: usize, height: usize, val: T) -> Self {
        Grid {
            data: vec![val; width * height],
            size: (width, height),
        }
    }

    pub fn size(&self) -> (usize, usize) {
        self.size
    }

    pub fn width(&self) -> usize {
        self.size.0
    }

    pub fn height(&self) -> usize {
        self.size.1
    }

    pub fn fill(&mut self, val: T) {
        self.data.fill(val);
    }

    pub fn has_xy(&self, x: i32, y: i32) -> bool {
        self.to_idx(x, y).is_some()
    }

    fn to_idx(&self, x: i32, y: i32) -> Option<usize> {
        if x < 0 || y < 0 || x >= self.size.0 as i32 || y >= self.size.1 as i32 {
            return None;
        }
        Some(x as usize + y as usize * self.size.0)
    }

    pub fn get_unchecked(&self, x: i32, y: i32) -> &T {
        let idx = x as usize + y as usize * self.size.0;
        unsafe { self.data.get_unchecked(idx) }
    }

    pub fn get(&self, x: i32, y: i32) -> Option<&T> {
        match self.to_idx(x, y) {
            None => None,
            Some(idx) => self.data.get(idx),
        }
    }

    pub fn set(&mut self, x: i32, y: i32, val: T) -> bool {
        match self.to_idx(x, y) {
            None => false,
            Some(idx) => {
                self.data[idx] = val;
                true
            }
        }
    }

    pub fn set_unchecked(&mut self, x: i32, y: i32, val: T) {
        let idx = x as usize + y as usize * self.size.0;
        self.data[idx] = val;
    }

    pub fn count(&self, val: T) -> usize
    where
        T: PartialEq,
    {
        self.data
            .iter()
            .fold(0, |out, v| if *v == val { out + 1 } else { out })
    }

    pub fn iter(&self) -> GridIterator<T> {
        GridIterator::new(self.data.iter(), self.width())
    }
}

pub struct GridIterator<'g, T> {
    iter: Enumerate<Iter<'g, T>>,
    width: usize,
}

impl<'g, T> GridIterator<'g, T> {
    fn new(iter: Iter<'g, T>, width: usize) -> Self {
        GridIterator {
            iter: iter.enumerate(),
            width,
        }
    }
}

impl<'g, T> Iterator for GridIterator<'g, T> {
    type Item = (i32, i32, &'g T);

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            None => None,
            Some((idx, val)) => {
                let x = idx as i32 % self.width as i32;
                let y = idx as i32 / self.width as i32;
                Some((x, y, val))
            }
        }
    }
}

// Marks a cell as being a member of blobNumber, then recursively iterates through the rest of the blob
pub fn flood_replace<T>(dest: &mut Grid<T>, x: i32, y: i32, find: T, replace: T) -> u32
where
    T: PartialEq + Copy + Default,
{
    let mut done = Grid::new(dest.width(), dest.height(), false);
    let mut todo: Vec<Point> = vec![Point::new(x, y)];

    let mut count = 0;

    while todo.len() > 0 {
        let item = todo.pop().unwrap();
        let x = item.x;
        let y = item.y;

        if !dest.has_xy(x, y) || *done.get_unchecked(x, y) {
            continue;
        }
        if *dest.get_unchecked(x, y) != find {
            continue;
        }
        dest.set(x, y, replace);
        done.set(x, y, true);
        count += 1;

        // Iterate through the four cardinal neighbors.
        for dir in DIRS4.iter() {
            let new_x = x + dir.x;
            let new_y = y + dir.y;
            // If the neighbor is an unmarked region cell,
            todo.push(Point::new(new_x, new_y));
        }
    }

    count
}

// Marks a cell as being a member of blobNumber, then recursively iterates through the rest of the blob
pub fn wrapped_flood_replace<T>(
    dest: &mut Grid<T>,
    origin: Point,
    find: T,
    replace: T,
    wrap: Wrap,
) -> u32
where
    T: PartialEq + Copy + Default,
{
    let mut done = Grid::new(dest.width(), dest.height(), false);
    let mut todo: Vec<Point> = vec![origin];

    let mut count = 0;

    while todo.len() > 0 {
        let item = todo.pop().unwrap();
        let x = item.x;
        let y = item.y;

        if !dest.has_xy(x, y) || *done.get_unchecked(x, y) {
            continue;
        }
        if *dest.get_unchecked(x, y) != find {
            continue;
        }
        dest.set(x, y, replace);
        done.set(x, y, true);
        count += 1;

        // Iterate through the four cardinal neighbors.
        for dir in DIRS4.iter() {
            let new_x = wrap.wrap_x(x + dir.x, dest.width() as u32);
            let new_y = wrap.wrap_y(y + dir.y, dest.height() as u32);
            // If the neighbor is an unmarked region cell,
            todo.push(Point::new(new_x, new_y));
        }
    }

    count
}

// Marks a cell as being a member of blobNumber, then recursively iterates through the rest of the blob
pub fn spread_replace<T>(
    dest: &mut Grid<T>,
    x: i32,
    y: i32,
    find: T,
    replace: T,
    rng: &mut RandomNumberGenerator,
    chance: u32,
) -> u32
where
    T: PartialEq + Copy + Default,
{
    let mut done = Grid::new(dest.width(), dest.height(), false);
    let mut todo: Vec<Point> = vec![Point::new(x, y)];

    let mut count = 0;

    while todo.len() > 0 {
        let item = todo.pop().unwrap();
        let x = item.x;
        let y = item.y;

        if !dest.has_xy(x, y) || *done.get_unchecked(x, y) {
            continue;
        }
        if *dest.get_unchecked(x, y) != find {
            continue;
        }
        if rng.chance(chance) == false {
            continue;
        }
        dest.set(x, y, replace);
        done.set(x, y, true);
        count += 1;

        // Iterate through the four cardinal neighbors.
        for dir in DIRS4.iter() {
            let new_x = x + dir.x;
            let new_y = y + dir.y;
            // If the neighbor is an unmarked region cell,
            todo.push(Point::new(new_x, new_y));
        }
    }

    count
}

pub fn value_bounds<T>(src: &Grid<T>, value: T) -> Rect
where
    T: PartialEq + Copy + Default,
{
    let mut found_value_at_this_line;

    let mut left = src.width().saturating_sub(1) as i32;
    let mut right = 0;
    let mut top = src.height().saturating_sub(1) as i32;
    let mut bottom = 0;

    // Figure out the top blob's height and width:
    // First find the max & min x:
    for i in 0..src.width() as i32 {
        found_value_at_this_line = false;
        for j in 0..src.height() as i32 {
            if *src.get_unchecked(i, j) == value {
                found_value_at_this_line = true;
                break;
            }
        }
        if found_value_at_this_line {
            if i < left {
                left = i;
            }
            if i > right {
                right = i;
            }
        }
    }

    // Then the max & min y:
    for j in 0..src.height() as i32 {
        found_value_at_this_line = false;
        for i in 0..src.width() as i32 {
            if *src.get_unchecked(i, j) == value {
                found_value_at_this_line = true;
                break;
            }
        }
        if found_value_at_this_line {
            if j < top {
                top = j;
            }
            if j > bottom {
                bottom = j;
            }
        }
    }

    Rect::with_bounds(left as i32, top as i32, right as i32, bottom as i32)
}

pub fn random_point_matching<T>(
    grid: &Grid<T>,
    find: T,
    rng: &mut RandomNumberGenerator,
) -> Option<Point>
where
    T: PartialEq + Copy + Default,
{
    let seq = rng.sequence((grid.width() * grid.height()) as usize);

    for v in seq {
        let x = (v % grid.width()) as i32;
        let y = (v / grid.width()) as i32;

        if *grid.get_unchecked(x, y) == find {
            return Some(Point::new(x, y));
        }
    }

    None
}

pub fn random_point_with<T, F>(
    grid: &Grid<T>,
    find: F,
    rng: &mut RandomNumberGenerator,
) -> Option<Point>
where
    T: PartialEq + Copy + Default,
    F: Fn(&T, i32, i32) -> bool,
{
    let seq = rng.sequence((grid.width() * grid.height()) as usize);

    for v in seq {
        let x = (v % grid.width()) as i32;
        let y = (v / grid.width()) as i32;

        let v = grid.get_unchecked(x, y);
        if find(v, x, y) {
            return Some(Point::new(x, y));
        }
    }

    None
}

pub fn dump_grid<T>(grid: &Grid<T>)
where
    T: Display + Copy + Default,
{
    let mut header = "   |".to_string();
    for x in 0..grid.width() {
        let text = format!("{}", x % 10);
        let ch = text.chars().next().unwrap();
        header.push(ch);
    }
    header.push('|');
    println!("{}", header);

    for y in 0..grid.height() as i32 {
        let mut line = format!("{:2} |", y);
        for x in 0..grid.width() as i32 {
            let v = grid.get(x, y).unwrap();
            let text = format!("{}", v);
            let ch = text.chars().last().unwrap_or('?');
            line.push(ch);
        }
        line.push_str(&format!("| {:2}", y));
        println!("{}", line);
    }

    let mut header = "   |".to_string();
    for x in 0..grid.width() {
        let text = format!("{}", x % 10);
        let ch = text.chars().next().unwrap();
        header.push(ch);
    }
    header.push('|');
    println!("{}", header);
}

pub fn dump_grid_with<T, F>(grid: &Grid<T>, func: F)
where
    F: Fn(&T, i32, i32) -> char,
    T: Display + Copy + Default,
{
    let mut header = "   |".to_string();
    for x in 0..grid.width() {
        let text = format!("{}", x % 10);
        let ch = text.chars().next().unwrap();
        header.push(ch);
    }
    header.push('|');
    println!("{}", header);

    for y in 0..grid.height() as i32 {
        let mut line = format!("{:2} |", y);
        for x in 0..grid.width() as i32 {
            let tile = func(grid.get(x, y).unwrap(), x, y);
            line.push(tile);
        }
        line.push_str(&format!("| {:2}", y));
        println!("{}", line);
    }

    let mut header = "   |".to_string();
    for x in 0..grid.width() {
        let text = format!("{}", x % 10);
        let ch = text.chars().next().unwrap();
        header.push(ch);
    }
    header.push('|');
    println!("{}", header);
}
