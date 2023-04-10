use lazy_static::lazy_static;
// use std::ops::{Add, AddAssign};
use super::BlockedSource;
use crate::point::Point;
use crate::point::DIRS;
use std::fmt::{Debug, Display};
use std::sync::Mutex;

lazy_static! {
    static ref CACHE: Mutex<Vec<Vec<Node>>> = Mutex::new(Vec::new());
}

pub const OK: f32 = 1.0;
pub const AVOIDED: f32 = 10.0;
pub const BLOCKED: f32 = 10000.0;
pub const OBSTRUCTION: f32 = 20000.0; // Blocks Diagonal
pub const NOT_DONE: f32 = 30000.0;

#[derive(Default, Copy, Clone, Debug)]
pub struct Node {
    pub xy: Point,
    // idx: usize,
    pub cost_so_far: f32,
    pub estimate_left: f32,
    prev: Option<usize>,
    next: Option<usize>,
}

impl Node {
    fn new() -> Self {
        Node {
            xy: Point::new(0, 0),
            // idx: 0,
            cost_so_far: 0.0,
            estimate_left: 0.0,
            next: None,
            prev: None,
        }
    }

    // fn reset(&mut self, x: i32, y: i32, idx: usize) {
    fn reset(&mut self, x: i32, y: i32) {
        // fn reset(&mut self, idx: usize) {
        self.cost_so_far = NOT_DONE;
        self.estimate_left = NOT_DONE;
        self.xy.set(x as i32, y as i32);
        // self.idx = idx;
        self.next = None;
        self.prev = None;
    }

    pub fn score(&self) -> f32 {
        score(self.cost_so_far, self.estimate_left)
    }

    pub fn dump(&self, width: usize) {
        let base = format!(
            "{},{} :: {:.1} + {:.1} = {}",
            self.xy.x,
            self.xy.y,
            self.cost_so_far,
            self.estimate_left,
            self.score()
        );
        let prev_text = match self.prev {
            None => "none".to_owned(),
            Some(idx) => format!("{},{}", idx % width, idx / width),
        };
        let next_text = match self.next {
            None => "none".to_owned(),
            Some(idx) => format!("{},{}", idx % width, idx / width),
        };
        println!("{}, prev={}, next={}", base, prev_text, next_text);
    }
}

pub fn score(cost_so_far: f32, estimate_left: f32) -> f32 {
    (cost_so_far * 100.0 + estimate_left * 125.0).floor()
}

pub struct SearchGrid {
    width: u32,
    height: u32,
    data: Option<Vec<Node>>,
    todo: Option<usize>,
    // active: bool,
    pub count: u32,
}

impl SearchGrid {
    // pub fn alloc(width: u32, height: u32) -> Box<Self> {
    //     let mut cache = CACHE.lock().unwrap();
    //     match cache.pop() {
    //         Some(mut item) => {
    //             // println!(" - reuse: {}x{}", item.width, item.height);
    //             item.reset(width, height);
    //             item
    //         }
    //         None => {
    //             // println!(" - create: {}x{}", width, height);
    //             Box::new(SearchGrid::new(width, height))
    //         }
    //     }
    // }

    // pub fn free(mut map: Box<SearchGrid>) {
    //     let mut cache = CACHE.lock().unwrap();
    //     map.active = false;
    //     // println!(" - save: {}x{}", map.width, map.height);
    //     cache.push(map);
    // }

    // fn new(width: u32, height: u32) -> Self {
    //     let mut c = SearchGrid {
    //         width,
    //         height,
    //         data: Vec::new(),
    //         active: true,
    //         todo: None,
    //         count: 0,
    //     };
    //     c.reset(width, height);
    //     c
    // }

    fn reset(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                if let Some(idx) = self.to_idx(x as i32, y as i32) {
                    let item = &mut self.data.as_mut().unwrap()[idx];
                    // item.reset(x,y,idx);
                    // item.reset(idx);
                    item.reset(x as i32, y as i32);
                }
            }
        }
    }

    pub fn new(width: u32, height: u32) -> Self {
        let size = (width * height) as usize;
        let data = match CACHE.lock().unwrap().pop() {
            Some(mut data) => {
                data.resize_with(size, Default::default);
                data
            }
            None => vec![Node::new(); size],
        };

        let mut grid = SearchGrid {
            width,
            height,
            data: Some(data),
            // active: true,
            todo: None,
            count: 0,
        };

        grid.reset();
        grid
    }

    pub fn clear_todo(&mut self) {
        for node in self.data.as_mut().unwrap().iter_mut() {
            node.next = None;
            node.prev = None;
        }
        self.todo = None;
    }

    pub fn len(&self) -> usize {
        self.data.as_ref().unwrap().len()
    }

    pub fn to_idx(&self, x: i32, y: i32) -> Option<usize> {
        if x < 0 || x >= self.width as i32 || y < 0 || y >= self.height as i32 {
            return None;
        }
        Some((x + (y * self.width as i32)) as usize)
    }

    pub fn to_xy(&self, idx: usize) -> (i32, i32) {
        (
            idx as i32 % self.width as i32,
            idx as i32 / self.width as i32,
        )
    }

    pub fn get_xy(&self, x: i32, y: i32) -> Option<&Node> {
        match self.to_idx(x, y) {
            None => None,
            Some(idx) => self.data.as_ref().unwrap().get(idx),
        }
    }

    pub fn get(&self, idx: usize) -> Option<&Node> {
        self.data.as_ref().unwrap().get(idx)
    }

    // fn get_mut_xy(&mut self, x: i32, y: i32) -> Option<&mut Node> {
    //     if !self.has_xy(x, y) {
    //         return None;
    //     }
    //     let idx = self.to_idx(x, y);
    //     self.data.get_mut(idx)
    // }

    fn get_mut(&mut self, idx: usize) -> Option<&mut Node> {
        self.data.as_mut().unwrap().get_mut(idx)
    }

    pub fn set_goal_xy(&mut self, x: i32, y: i32) {
        if let Some(idx) = self.to_idx(x, y) {
            self.set_goal(idx);
        }
    }

    fn set_goal(&mut self, idx: usize) {
        if idx < self.data.as_ref().unwrap().len() {
            self.update_node(idx, 0.0, 0.0);
        }
    }

    pub fn update_node_xy(&mut self, x: i32, y: i32, distance: f32, cost: f32) -> bool {
        match self.to_idx(x, y) {
            None => false,
            Some(idx) => self.update_node(idx, distance, cost),
        }
    }

    fn update_node(&mut self, idx: usize, distance: f32, estimate: f32) -> bool {
        // if !self.has_idx(idx) {
        //     return false;
        // }

        let data = self.data.as_mut().unwrap();

        let (prev_idx, next_idx) = {
            let item = data.get(idx).unwrap();
            if item.score() <= score(distance, estimate) {
                // println!(
                //     "- existing dist {} <= dist {}",
                //     item.score(),
                //     score(distance, estimate)
                // );
                return false;
            }

            (item.prev, item.next)
        };

        // remove from list...
        if self.todo == Some(idx) {
            self.todo = next_idx;
        }

        if prev_idx.is_some() {
            assert!(prev_idx.unwrap() != idx);
            assert!(prev_idx != next_idx);
            let mut prev = data.get_mut(prev_idx.unwrap()).unwrap();
            prev.next = next_idx;
        }

        if next_idx.is_some() {
            assert!(next_idx.unwrap() != idx);
            assert!(next_idx != prev_idx);
            {
                let mut next = data.get_mut(next_idx.unwrap()).unwrap();
                next.prev = prev_idx;
            }
        }

        let item = data.get_mut(idx).unwrap();
        item.prev = None;
        item.next = None;

        if distance >= OBSTRUCTION {
            item.cost_so_far = OBSTRUCTION;
            item.estimate_left = 0.0;
            // println!("- obstruction");
            return false;
        } else if distance >= BLOCKED {
            item.cost_so_far = BLOCKED;
            item.estimate_left = 0.0;
            // println!("- blocked");
            return false;
        }

        item.cost_so_far = distance;
        item.estimate_left = estimate;
        self.insert_node(idx)
    }

    pub fn distance_xy(&self, x: i32, y: i32) -> f32 {
        match self.get_xy(x, y) {
            None => NOT_DONE,
            Some(item) => item.cost_so_far,
        }
    }

    pub fn estimate_xy(&self, x: i32, y: i32) -> f32 {
        match self.get_xy(x, y) {
            None => NOT_DONE,
            Some(item) => item.estimate_left,
        }
    }

    pub fn score_xy(&self, x: i32, y: i32) -> f32 {
        match self.to_idx(x, y) {
            None => OBSTRUCTION,
            Some(idx) => self.score(idx),
        }
    }

    fn score(&self, idx: usize) -> f32 {
        match self.get(idx) {
            None => NOT_DONE,
            Some(item) => item.score(),
        }
    }

    // fn iter(&self) -> std::slice::Iter<'_, Node> {
    //     self.data.iter()
    // }

    // fn iter_mut(&mut self) -> std::slice::IterMut<'_, Node> {
    //     self.data.iter_mut()
    // }

    pub fn next_xy(&self, x: i32, y: i32) -> Option<usize> {
        match self.get_xy(x, y) {
            None => None,
            Some(item) => item.next,
        }
    }

    pub fn next(&self, idx: usize) -> Option<usize> {
        match self.get(idx) {
            None => None,
            Some(item) => item.next,
        }
    }

    pub fn prev(&self, idx: usize) -> Option<usize> {
        match self.get(idx) {
            None => None,
            Some(item) => item.prev,
        }
    }

    pub fn prev_xy(&self, x: i32, y: i32) -> Option<usize> {
        match self.get_xy(x, y) {
            None => None,
            Some(item) => item.prev,
        }
    }

    pub fn insert_node(&mut self, idx: usize) -> bool {
        self.count += 1;
        let new_score = self.score(idx);

        if self.todo.is_none() {
            self.todo = Some(idx);
            let item = self.data.as_mut().unwrap().get_mut(idx).unwrap();
            item.next = None;
            item.prev = None;

            // println!("- set todo => {}", idx);
            return true;
        } else if self.score(self.todo.unwrap()) > new_score {
            // println!(
            //     "- todo distance {} > distance {}",
            //     self.score(self.todo.unwrap()),
            //     score(distance, estimate)
            // );
            {
                let todo_item = self
                    .data
                    .as_mut()
                    .unwrap()
                    .get_mut(self.todo.unwrap())
                    .unwrap();
                todo_item.prev = Some(idx);
            }
            {
                let item = self.data.as_mut().unwrap().get_mut(idx).unwrap();
                item.next = self.todo;
                item.prev = None;
            }
            self.todo = Some(idx);
            return true;
        }

        let mut prev = self.todo;
        let mut current = self.next(self.todo.unwrap());

        loop {
            if current.is_none() {
                break;
            }

            // if current.unwrap() == idx {
            //     panic!("Found item in TODO list!");
            // }

            let current_score = self.score(current.unwrap());
            // println!(
            //     "- insert check @ {} - {} vs {}",
            //     current.unwrap(),
            //     current_score,
            //     new_score
            // );
            if current_score > new_score {
                break;
            }
            prev = current;
            current = self.next(prev.unwrap());
        }

        {
            let item = self.get_mut(idx).unwrap();
            item.prev = prev;
            item.next = current;
            // println!(
            //     "- inserted - prev: {:?}, next: {:?}, dist: {}",
            //     item.prev,
            //     item.next,
            //     item.score()
            // );
        }

        if prev.is_some() {
            // if prev.unwrap() == idx {
            //     panic!("ABOUT TO ADD PREV LOOP");
            // }
            let prev_item = &mut self.get_mut(prev.unwrap()).unwrap();
            prev_item.next = Some(idx);
        }
        if current.is_some() {
            // if current.unwrap() == idx {
            //     panic!("ABOUT TO ADD NEXT LOOP");
            // }
            let current_item = &mut self.get_mut(current.unwrap()).unwrap();
            current_item.prev = Some(idx);
        }

        return true;
    }

    pub fn add(&mut self, other: &SearchGrid) {
        let self_data = self.data.as_mut().unwrap();
        let other_data = other.data.as_ref().unwrap();

        if self_data.len() != other_data.len() {
            panic!("Cannot add costmaps with different size!");
        }
        for i in 0..self_data.len() {
            let a = self_data.get_mut(i).unwrap();
            let b = &other_data[i];
            a.cost_so_far += b.cost_so_far;
            a.estimate_left += b.estimate_left;
        }
    }

    // fn pop_idx(&mut self, idx: usize) {
    //     let (prev, next) = match self.get_mut(idx) {
    //         None => return,
    //         Some(item) => {
    //             let prev = item.prev;
    //             let next = item.next;
    //             item.prev = None;
    //             item.next = None;
    //             (prev, next)
    //         }
    //     };

    //     if prev.is_some() {
    //         if let Some(prev) = self.get_mut(prev.unwrap()) {
    //             prev.next = next;
    //         }
    //     }
    //     if next.is_some() {
    //         if let Some(next) = self.get_mut(next.unwrap()) {
    //             next.prev = prev;
    //         }
    //     }
    // }

    pub fn pop_todo(&mut self) -> Option<usize> {
        if self.todo.is_none() {
            return None;
        }

        let idx = self.todo.take().unwrap();
        self.todo = self.next(idx);

        if self.todo.is_some() {
            let item = self.get_mut(self.todo.unwrap()).unwrap();
            item.prev = None;
        }

        let item = self.get_mut(idx).unwrap();
        item.next = None;
        item.prev = None;

        assert!(Some(idx) != self.todo);
        // println!("POP TODO = {}, Next TODO = {:?}", idx, self.todo);

        Some(idx)
    }

    pub fn next_dir<B: BlockedSource>(
        &self,
        from_x: i32,
        from_y: i32,
        is_blocked: &B,
        allow_diagonal: bool,
    ) -> Option<&'static Point> {
        let mut new_x;
        let mut new_y;
        let mut best_score = f32::MAX;

        // brogueAssert(coordinatesAreInMap(x, y));

        let mut best_dir: usize = usize::MAX;

        if self.to_idx(from_x, from_y).is_none() {
            panic!("Invalid XY: {}, {}", from_x, from_y);
        }

        let dist = self.distance_xy(from_x, from_y);
        let range = match allow_diagonal {
            true => 0..8,
            false => 0..4,
        };

        // println!(
        //     "next step: {},{} @ {} ({})",
        //     from_x, from_y, dist, allow_diagonal
        // );
        for index in range {
            let dir = &DIRS[index];
            new_x = from_x + dir.x;
            new_y = from_y + dir.y;
            if self.to_idx(new_x, new_y).is_none() {
                continue;
            }

            if index > 3 {
                // is diagonal
                if self.distance_xy(new_x, from_y) >= OBSTRUCTION
                    || self.distance_xy(from_x, new_y) >= OBSTRUCTION
                {
                    continue; // diagonal blocked
                }
            }

            let new_dist = self.distance_xy(new_x, new_y);
            if new_dist >= BLOCKED {
                continue;
            }

            // println!(" - {},{} @ {}", new_x, new_y, new_dist);

            if new_dist < dist {
                if new_dist < best_score
                    && (new_dist == 0.0 || !is_blocked.is_blocked(new_x, new_y))
                {
                    best_dir = index;
                    best_score = new_dist;
                    // println!(" :: !!!");
                }
            }
        }
        match best_dir {
            usize::MAX => None,
            _ => DIRS.get(best_dir),
        }
    }

    pub fn path_from<'a, B: BlockedSource>(
        &'a self,
        pt: Point,
        is_blocked: &'a B,
        allow_diagonal: bool,
    ) -> PathIter<B> {
        PathIter::new(&self, pt.x, pt.y, is_blocked, allow_diagonal)
    }

    pub fn path_from_xy<'a, B: BlockedSource>(
        &'a self,
        x: i32,
        y: i32,
        is_blocked: &'a B,
        allow_diagonal: bool,
    ) -> PathIter<B> {
        PathIter::new(&self, x, y, is_blocked, allow_diagonal)
    }
}

impl Drop for SearchGrid {
    fn drop(&mut self) {
        if let Some(data) = self.data.take() {
            CACHE.lock().unwrap().push(data);
        }
    }
}

fn as_char(v: f32) -> char {
    if v >= NOT_DONE as f32 {
        return '?';
    }
    if v >= OBSTRUCTION as f32 {
        return '*';
    }
    if v >= BLOCKED as f32 {
        return '#';
    }
    if v >= 61.0 {
        return 'Z';
    }
    if v < 0.0 {
        return '-';
    }

    let v = v.floor() as u8;
    if v <= 9 {
        return (('0' as u8) + v) as u8 as char;
    }
    if v <= 35 {
        return (('a' as u8) - 10 + v) as u8 as char;
    }
    (('A' as u8) - 36 + v as u8) as char
}

impl Display for SearchGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            let line = (0..self.width)
                .map(|x| as_char(self.distance_xy(x as i32, y as i32)))
                .collect::<String>();
            writeln!(f, "{}", line)?
        }
        Ok(())
    }
}

impl Debug for SearchGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)?;

        let mut current = self.todo;
        let mut todo: Vec<usize> = Vec::new();
        while current.is_some() {
            todo.push(current.unwrap());
            current = self.next(current.unwrap());
        }
        writeln!(
            f,
            "TODO: {}None",
            todo.iter()
                .map(|v| v.to_string() + " -> ")
                .collect::<String>()
        )?;
        Ok(())
    }
}

// impl AddAssign<&Box<CostMap>> for Box<CostMap> {
//     fn add_assign(&mut self, rhs: &Box<CostMap>) {
//         CostMap::add(self, rhs);
//     }
// }

// impl Add<&Box<CostMap>> for &Box<CostMap> {
//     type Output = Box<CostMap>;

//     fn add(self, rhs: &Box<CostMap>) -> Self::Output {
//         if self.data.len() != rhs.data.len() {
//             panic!("Cannot add costmaps with different size!");
//         }
//         let mut result = CostMap::alloc(self.width, self.height);
//         for i in 0..self.data.len() {
//             let a = &self.data[i];
//             let b = &rhs.data[i];
//             let c = result.data.get_mut(i).unwrap();
//             c.distance = a.distance + b.distance;
//         }
//         result
//     }
// }

pub struct PathIter<'a, B>
where
    B: BlockedSource,
{
    cost_map: &'a SearchGrid,
    x: i32,
    y: i32,
    is_blocked: &'a B,
    allow_diagonal: bool,
}

impl<'a, B> PathIter<'a, B>
where
    B: BlockedSource,
{
    fn new(
        cost_map: &'a SearchGrid,
        x: i32,
        y: i32,
        is_blocked: &'a B,
        allow_diagonal: bool,
    ) -> Self {
        PathIter {
            cost_map,
            x,
            y,
            is_blocked,
            allow_diagonal,
        }
    }
}

impl<'a, B> Iterator for PathIter<'a, B>
where
    B: BlockedSource,
{
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        match self
            .cost_map
            .next_dir(self.x, self.y, self.is_blocked, self.allow_diagonal)
        {
            Some(dir) => {
                self.x += dir.x;
                self.y += dir.y;
                Some(Point::new(self.x, self.y))
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn set_goal() {
        let mut c = SearchGrid::new(10, 10);
        c.set_goal_xy(5, 5);
        assert_eq!(c.todo, Some(55));
        assert_eq!(c.prev(55), None);
        assert_eq!(c.next(55), None);

        c.set_goal_xy(8, 8);
        assert_eq!(c.todo, Some(55));
        assert_eq!(c.prev(55), None);
        assert_eq!(c.next(55), Some(88));
        assert_eq!(c.prev(88), Some(55));
        assert_eq!(c.next(88), None);

        c.set_goal_xy(1, 1);
        assert_eq!(c.todo, Some(55));
        assert_eq!(c.prev(55), None);
        assert_eq!(c.next(55), Some(88));
        assert_eq!(c.prev(88), Some(55));
        assert_eq!(c.next(88), Some(11));
        assert_eq!(c.prev(11), Some(88));
        assert_eq!(c.next(11), None);

        assert_eq!(c.pop_todo(), Some(55));
        assert_eq!(c.pop_todo(), Some(88));
        assert_eq!(c.pop_todo(), Some(11));
        assert_eq!(c.pop_todo(), None);
    }

    // #[test]
    // fn iter() {
    //     let a = SearchGrid::alloc(10, 10);
    //     let mut count = 0;
    //     for _ in a.iter() {
    //         count += 1;
    //     }
    //     assert_eq!(count, a.data.len());
    //     SearchGrid::free(a);
    // }

    #[test]
    fn display() {
        let mut a = SearchGrid::new(10, 10);
        a.set_goal_xy(5, 5);
        println!("{}", a);

        // assert!(false);
    }

    #[test]
    fn debug() {
        let mut a = SearchGrid::new(10, 10);
        a.set_goal_xy(5, 5);
        a.set_goal_xy(8, 8);
        println!("{:?}", a);

        // assert!(false);
    }

    // #[test]
    // fn add_assign() {
    //     let mut a = CostMap::alloc(10, 10);
    //     let b = CostMap::alloc(10, 10);
    //     a += &b;
    //     CostMap::free(b);
    //     CostMap::free(a);
    // }

    // #[test]
    // fn add() {
    //     let a = CostMap::alloc(10, 10);
    //     let b = CostMap::alloc(10, 10);
    //     let c = &a + &b;
    //     CostMap::free(b);
    //     CostMap::free(a);
    //     CostMap::free(c);
    // }
}
