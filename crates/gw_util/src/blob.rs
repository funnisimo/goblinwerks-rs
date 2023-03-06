use crate::{
    grid::{flood_replace, value_bounds, Grid},
    point::DIRS,
    rect::Rect,
    rng::RandomNumberGenerator,
};

pub struct BlobConfig {
    pub rng: RandomNumberGenerator,
    pub rounds: u32,
    pub min_width: u32,
    pub min_height: u32,
    pub max_width: u32,
    pub max_height: u32,
    pub percent_seeded: u32,
    pub birth_parameters: [bool; 9],
    pub survival_parameters: [bool; 9],
}

impl Default for BlobConfig {
    fn default() -> BlobConfig {
        BlobConfig {
            rng: RandomNumberGenerator::new(),
            rounds: 5,
            min_width: 999,
            min_height: 999,
            max_width: 999,
            max_height: 999,
            percent_seeded: 50,
            birth_parameters: [false, false, false, false, false, false, true, true, true], // 'ffffffttt',  TODO - birth_count?
            survival_parameters: [false, false, false, false, true, true, true, true, true], // 'ffffttttt',    TODO - survival_count?
        }
    }
}

pub struct Blob {
    config: BlobConfig,
}

impl Blob {
    pub fn new(mut config: BlobConfig) -> Self {
        if config.min_width >= config.max_width {
            config.min_width = (0.75 * config.max_width as f32).round() as u32;
            config.max_width = (1.25 * config.max_width as f32).round() as u32;
        }
        if config.min_height >= config.max_height {
            config.min_height = (0.75 * config.max_height as f32).round() as u32;
            config.max_height = (1.25 * config.max_height as f32).round() as u32;
        }

        Blob { config }
    }

    pub fn carve<F>(&mut self, width: u32, height: u32, mut set_fn: F) -> Rect
    where
        F: FnMut(i32, i32) -> (),
    {
        let mut blob_number;
        let mut blob_size;
        let mut top_blob_number;
        let mut top_blob_size;

        let mut bounds;
        let mut dest = Grid::new(width as usize, height as usize, 0);

        let max_width = width.min(self.config.max_width);
        let max_height = height.min(self.config.max_height);

        let min_width = width.min(self.config.min_width);
        let min_height = height.min(self.config.min_height);

        let left = (dest.width().saturating_sub(max_width as usize) / 2) as i32;
        let top = (dest.height().saturating_sub(max_height as usize) / 2) as i32;

        let mut tries = 10;

        // Generate blobs until they satisfy the minBlobWidth and minBlobHeight restraints
        loop {
            // Clear buffer.
            dest.fill(0);

            // Fill relevant portion with noise based on the percentSeeded argument.
            for j in 0..max_height as i32 {
                for i in 0..max_width as i32 {
                    let val = match self.config.rng.chance(self.config.percent_seeded) {
                        false => 0,
                        true => 1,
                    };
                    dest.set(i + left, j + top, val);
                }
            }

            // Some iterations of cellular automata
            for _ in 0..self.config.rounds {
                if !cellular_automata_round(
                    &mut dest,
                    &self.config.birth_parameters,
                    &self.config.survival_parameters,
                ) {
                    break; // cellularAutomataRound did not make any changes
                }
            }

            // Now to measure the result. These are best-of variables; start them out at worst-case values.
            top_blob_size = 0;
            top_blob_number = 0;

            // Fill each blob with its own number, starting with 2 (since 1 means floor), and keeping track of the biggest:
            blob_number = 2;

            for j in 0..dest.height() as i32 {
                for i in 0..dest.width() as i32 {
                    if *dest.get_unchecked(i, j) == 1 {
                        // an unmarked blob
                        // Mark all the cells and returns the total size:
                        blob_size = flood_replace(&mut dest, i, j, 1, blob_number);
                        if blob_size > top_blob_size {
                            // if this blob is a new record
                            top_blob_size = blob_size;
                            top_blob_number = blob_number;
                        }
                        blob_number = blob_number + 1;
                    }
                }
            }

            // Figure out the top blob's height and width:
            bounds = value_bounds(&mut dest, top_blob_number);

            if (bounds.width() < min_width || bounds.height() < min_height || top_blob_number == 0)
                && tries > 0
            {
                tries = tries - 1;
                continue; //try again
            }

            break;
        }

        // Replace the winning blob with 1's, and everything else with 0's:
        for j in 0..dest.height() as i32 {
            for i in 0..dest.width() as i32 {
                if *dest.get_unchecked(i, j) == top_blob_number {
                    set_fn(i, j);
                }
            }
        }

        // Populate the returned variables.
        bounds
    }
}

fn cellular_automata_round(
    grid: &mut Grid<i32>,
    birth_parameters: &[bool; 9],
    survival_parameters: &[bool; 9],
) -> bool {
    let buffer2 = grid.clone();

    let mut did_something = false;
    for j in 0..grid.height() as i32 {
        for i in 0..grid.width() as i32 {
            let mut count = 0;

            for dir in DIRS.iter() {
                let new_x = dir.x + i as i32;
                let new_y = dir.y + j as i32;
                if grid.has_xy(new_x, new_y) && *buffer2.get_unchecked(new_x, new_y) != 0 {
                    count = count + 1;
                }
            }

            if *buffer2.get_unchecked(i, j) == 0 && birth_parameters[count] {
                grid.set(i, j, 1); // birth
                did_something = true;
            } else if *buffer2.get_unchecked(i, j) != 0 && survival_parameters[count] {
                // survival
            } else {
                grid.set(i, j, 0); // death
                did_something = true;
            }
        }
    }

    did_something
}

pub fn fill_blob<T>(grid: &mut Grid<T>, opts: Option<BlobConfig>) -> Rect
where
    T: Copy + PartialEq + Default + From<u8>,
{
    let mut blob = match opts {
        None => Blob::new(BlobConfig::default()),
        Some(opts) => Blob::new(opts),
    };

    return blob.carve(grid.width() as u32, grid.height() as u32, |x, y| {
        grid.set(x, y, T::from(1));
    });
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::grid::Grid;

    #[test]
    fn default_constructor() {
        let blob = Blob::new(BlobConfig::default());
        assert!(blob.config.min_height > 0);
        assert!(blob.config.min_width > 0);
    }

    #[test]
    fn fill_blob_default() {
        let mut a = Grid::new(80, 30, 0);
        assert_eq!(a.count(1), 0);

        fill_blob(&mut a, None);

        assert!(a.count(1) > 0);
    }

    /*
        test('fillBlob', () => {
            a = GW.grid.alloc(80, 30, 0);
            expect(a.count(1)).toEqual(0);

            GW.blob.fillBlob(a, {
                minWidth: 4,
                minHeight: 4,
                maxWidth: 30,
                maxHeight: 15,
                percentSeeded: 55,
            });
            expect(a.count(1)).toBeGreaterThan(10);
        });

        test('fillBlob - can handle min >= max', () => {
            GW.rng.random.seed(123456);
            a = GW.grid.alloc(50, 30, 0);
            expect(a.count(1)).toEqual(0);

            GW.blob.fillBlob(a, {
                minWidth: 12,
                minHeight: 12,
                maxWidth: 10,
                maxHeight: 10,
                percentSeeded: 55,
            });

            expect(a.count(1)).toBeGreaterThan(10);
        });

        test('fillBlob', () => {
            GW.rng.random.seed(12345);
            a = GW.grid.alloc(50, 50);

            const blob = new GW.blob.Blob({
                minWidth: 5,
                minHeight: 5,
                maxWidth: 20,
                maxHeight: 20,
                percentSeeded: 55,
            });

            let results = blob.carve(a.width, a.height, (x, y) => (a[x][y] = 1));
            expect(results).toEqual({
                x: 16,
                y: 23,
                width: 18,
                height: 12,
            });

            // force an odd return from '_cellularAutomataRound'

            // @ts-ignore
            jest.spyOn(blob, '_cellularAutomataRound').mockReturnValueOnce(false);

            a.fill(0);
            results = blob.carve(a.width, a.height, (x, y) => (a[x][y] = 1));

            expect(results).toEqual({
                x: 23,
                y: 15,
                width: 12,
                height: 14,
            });
        });
    });
    */
}
