use rand::prelude::*;
use rand_pcg::Mcg128Xsl64;

pub use rand::seq::SliceRandom;
pub use rand::RngCore;

type RNG = Mcg128Xsl64;

pub struct RandomNumberGenerator {
    source: RNG,
}

impl RandomNumberGenerator {
    pub fn new() -> Self {
        RandomNumberGenerator {
            source: RNG::from_entropy(),
        }
    }

    /// Creates a new RNG from a specific seed
    pub fn seeded(seed: u64) -> Self {
        RandomNumberGenerator {
            source: RNG::seed_from_u64(seed),
        }
    }

    pub fn chance(&mut self, count: u32) -> bool {
        self.chance_in(count, 100)
    }

    pub fn chance_in(&mut self, count: u32, of_total: u32) -> bool {
        if count == 0 {
            return false;
        }
        let r = self.rand(of_total);
        // println!(
        //     "chance_in {} of {} = {}/{} : {}",
        //     count,
        //     of_total,
        //     r,
        //     of_total,
        //     r < count
        // );
        r < count
    }

    /// Returns a random value of 0 <= val < count
    pub fn rand(&mut self, count: u32) -> u32 {
        self.range(0, count as i32) as u32
    }

    /// Returns a random value in the specified range, of type specified at the call site.
    /// This is INCLUSIVE of the first parameter, and EXCLUSIVE of the second.
    /// So range(1,6) will give you numbers from 1 to 5.
    pub fn range(&mut self, min: i32, max: i32) -> i32 {
        if max <= min {
            return min;
        }
        let range = max - min;
        let rng = self.source.next_u32() % (range as u32);
        rng as i32 + min
    }

    /// Returns a random value in the specified range, of type specified at the call site.
    /// So range(1.0,6.0) will give you numbers from 1.0 to 5.99999
    pub fn frange(&mut self, min: f32, max: f32) -> f32 {
        if max <= min {
            return min;
        }
        let range = max - min;
        let pct = self.source.next_u32() as f32 / (u32::MAX as f32);

        let rng = pct * range;
        rng + min
    }

    /// Rolls dice, using the classic 3d6 type of format: count is the number of dice, sides is the max number on each die [1-sides].
    pub fn roll_dice(&mut self, count: u32, sides: u32) -> u32 {
        let mut r = 0;
        for _ in 0..count {
            r += self.range(1, sides as i32 + 1);
        }
        r as u32
    }

    // /// Rolls dice based on a DiceType struct, including application of the bonus
    // #[cfg(feature = "parsing")]
    // pub fn roll(&mut self, dice: DiceType) -> i32 {
    //     self.roll_dice(dice.n_dice, dice.die_type) + dice.bonus
    // }

    // /// Rolls dice based on passing in a string, such as roll_str("1d12")
    // #[cfg(feature = "parsing")]
    // pub fn roll_str<S: ToString>(&mut self, dice: S) -> Result<i32, DiceParseError> {
    //     match parse_dice_string(&dice.to_string()) {
    //         Ok(dt) => Ok(self.roll(dt)),
    //         Err(e) => Err(e),
    //     }
    // }

    /// Returns a random index into a slice
    pub fn random_slice_index<T>(&mut self, slice: &[T]) -> Option<usize> {
        if slice.is_empty() {
            None
        } else {
            let sz = slice.len();
            if sz == 1 {
                Some(0)
            } else {
                Some(self.range(0, sz as i32) as usize)
            }
        }
    }

    /// Returns a random entry in a slice (or none if empty)
    pub fn random_slice_entry<'a, T>(&mut self, slice: &'a [T]) -> Option<&'a T> {
        if slice.is_empty() {
            None
        } else {
            let sz = slice.len();
            if sz == 1 {
                Some(&slice[0])
            } else {
                Some(&slice[self.range(0, sz as i32) as usize])
            }
        }
    }

    // /// Get underlying RNG implementation for use in traits / algorithms exposed by
    // /// other crates (eg. `rand` itself)
    // pub fn get_rng(&mut self) -> &mut XorShiftRng {
    //     &mut self.rng
    // }
}

impl RngCore for RandomNumberGenerator {
    fn next_u64(&mut self) -> u64 {
        self.source.next_u64()
    }

    fn next_u32(&mut self) -> u32 {
        self.source.next_u32()
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.source.fill_bytes(dest);
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error> {
        self.source.try_fill_bytes(dest)
    }
}

#[cfg(test)]
mod tests {
    use super::RandomNumberGenerator;

    #[test]
    fn roll_basic() {
        let mut rng = RandomNumberGenerator::seeded(12345);
        assert_eq!(rng.roll_dice(1, 6), 2);
        assert_eq!(rng.roll_dice(3, 10), 22);
    }

    #[test]
    fn test_roll_range() {
        let mut rng = RandomNumberGenerator::new();
        for _ in 0..100 {
            let n = rng.roll_dice(1, 20);
            assert!(n > 0 && n < 21);
        }
    }

    #[test]
    fn random_slice_index_empty() {
        let mut rng = RandomNumberGenerator::new();
        let test_data: Vec<i32> = Vec::new();
        assert!(rng.random_slice_index(&test_data).is_none());
    }

    #[test]
    fn random_slice_index_valid() {
        let mut rng = RandomNumberGenerator::new();
        let test_data: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        for _ in 0..100 {
            match rng.random_slice_index(&test_data) {
                None => assert!(1 == 2),
                Some(idx) => assert!(idx < test_data.len()),
            }
        }
    }

    #[test]
    fn random_slice_entry_empty() {
        let mut rng = RandomNumberGenerator::new();
        let test_data: Vec<i32> = Vec::new();
        assert!(rng.random_slice_entry(&test_data).is_none());
    }

    #[test]
    fn random_slice_entry_valid() {
        let mut rng = RandomNumberGenerator::new();
        let test_data: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        for _ in 0..100 {
            match rng.random_slice_entry(&test_data) {
                None => assert!(1 == 2),
                Some(e) => assert!(*e > 0 && *e < 11),
            }
        }
    }
}
