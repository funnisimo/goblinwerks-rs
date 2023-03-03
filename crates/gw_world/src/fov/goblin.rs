use super::{FovSource, FovTarget};
use gw_util::point::Point;
use gw_util::point::DIRS;

struct Data {
    start_x: i32,
    start_y: i32,
    max_radius: u32,
}

impl Data {
    fn new(radius: u32) -> Data {
        Data {
            start_x: 0,
            start_y: 0,
            max_radius: radius,
        }
    }
}

pub fn calculate_fov<S: FovSource, T: FovTarget>(
    source: &S,
    point: Point,
    max_radius: u32,
    target: &mut T,
) {
    let size = source.get_size();
    target.reset(size.0, size.1);

    let mut data = Data::new(max_radius);
    data.start_x = point.x;
    data.start_y = point.y;

    target.set_visible(point.x, point.y, 1.0);

    // uses the diagonals
    for i in 4..8 {
        let d = &DIRS[i];
        cast_light(&mut data, source, target, 1, 1.0, 0.0, 0, d.x, d.y, 0);
        cast_light(&mut data, source, target, 1, 1.0, 0.0, d.x, 0, 0, d.y);
    }
}

// NOTE: slope starts a 1 and ends at 0.
fn cast_light<S: FovSource, T: FovTarget>(
    data: &mut Data,
    source: &S,
    target: &mut T,
    row: i32,
    start_slope: f32,
    end_slope: f32,
    xx: i32,
    xy: i32,
    yx: i32,
    yy: i32,
) {
    if (row as u32) > data.max_radius {
        // println!(
        //     "CAST: row={}, start={}, end={}, row >= max_radius => cancel",
        //     row, start_slope, end_slope
        // );
        return;
    }
    if start_slope < end_slope {
        // println!(
        //     "CAST: row={}, start={}, end={}, start < end => cancel",
        //     row, start_slope, end_slope
        // );
        return;
    }
    // println!(
    //     "CAST: row={}, start={}, end={}, x={},{}, y={},{}",
    //     row, start_slope, end_slope, xx, xy, yx, yy
    // );

    let mut next_start = start_slope;

    let mut blocked = false;
    let delta_y = -row;
    let mut current_x: i32;
    let mut current_y: i32;
    let mut outer_slope: f32;
    let mut inner_slope: f32;
    let mut max_slope: f32;
    let mut min_slope: f32;

    for delta_x in -row..=0 {
        current_x = data.start_x + delta_x * xx + delta_y * xy;
        current_y = data.start_y + delta_x * yx + delta_y * yy;
        outer_slope = (delta_x as f32 - 0.5) / (delta_y as f32 + 0.5);
        inner_slope = (delta_x as f32 + 0.5) / (delta_y as f32 - 0.5);
        max_slope = delta_x as f32 / (delta_y as f32 + 0.5);
        min_slope = (delta_x as f32 + 0.5) / delta_y as f32;

        if !source.has_xy(current_x, current_y) {
            blocked = true;
            // next_start = inner_slope;
            continue;
        }

        // println!(
        //     "- test {},{} ... start={}, min={}, max={}, end={}, dx={}, dy={}",
        //     current_x, current_y, start_slope, max_slope, min_slope, end_slope, delta_x, delta_y
        // );

        if start_slope < min_slope {
            blocked = source.is_opaque(current_x, current_y);
            continue;
        } else if end_slope > max_slope {
            break;
        }

        //check if it's within the lightable area and light if needed
        let radius = source.calc_radius(delta_x, delta_y);
        // println!("       - radius: {} / {}", radius, data.max_radius);
        if radius.round() <= data.max_radius as f32 {
            let bright = 1.0 - ((radius - 1.0) / data.max_radius as f32);
            target.set_visible(current_x, current_y, bright);
            // println!("       - visible: {},{}: {}", current_x, current_y, bright);
        }

        if blocked {
            //previous cell was a blocking one
            if source.is_opaque(current_x, current_y) {
                //hit a wall
                // println!(
                //     "       - blocked {},{} ... next_start: {}",
                //     current_x, current_y, inner_slope
                // );
                next_start = inner_slope;
                continue;
            } else {
                blocked = false;
            }
        } else {
            if source.is_opaque(current_x, current_y) && (row as u32) < data.max_radius {
                //hit a wall within sight line
                // println!(
                //     "       - blocked {},{} ... start:{}, end:{}, next_start: {}",
                //     current_x, current_y, next_start, outer_slope, inner_slope
                // );
                blocked = true;
                cast_light(
                    data,
                    source,
                    target,
                    row + 1,
                    next_start,
                    outer_slope,
                    xx,
                    xy,
                    yx,
                    yy,
                );
                next_start = inner_slope;
            }
        }
    }

    if !blocked {
        cast_light(
            data,
            source,
            target,
            row + 1,
            next_start,
            end_slope,
            xx,
            xy,
            yx,
            yy,
        );
    }
}
