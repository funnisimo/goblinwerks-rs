use crate::rng::RandomNumberGenerator;
use lazy_static::lazy_static;
use regex::{Captures, Regex};
use std::{fmt::Display, str::FromStr};

#[derive(Debug, Clone, PartialEq)]
pub enum DiceStep {
    Die(i32, u32),
    Const(i32),
}

impl DiceStep {
    pub fn roll(&self, rng: &mut RandomNumberGenerator) -> i32 {
        match self {
            DiceStep::Const(v) => *v,
            DiceStep::Die(c, s) => c.signum() * rng.roll_dice(c.abs() as u32, *s) as i32,
        }
    }

    pub fn to_neg(self) -> Self {
        match self {
            DiceStep::Const(v) => DiceStep::Const(-v),
            DiceStep::Die(s, c) => DiceStep::Die(-s, c),
        }
    }
}

impl Display for DiceStep {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DiceStep::Const(v) => {
                if *v > 0 {
                    write!(f, "+{}", v)
                } else {
                    write!(f, "{}", v)
                }
            }
            DiceStep::Die(c, s) => {
                if *c > 0 {
                    write!(f, "+{}d{}", c, s)
                } else {
                    write!(f, "-{}d{}", c, s)
                }
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum DiceParseError {
    WrongFormat,
}

lazy_static! {
    static ref DICE_RE: Regex = Regex::new(r"^((\d*)d)?(\d+)$").unwrap();
}

impl FromStr for DiceStep {
    type Err = DiceParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // need to split on +/- but retain sign...

        let caps: Captures = match DICE_RE.captures(s) {
            None => return Err(DiceParseError::WrongFormat),
            Some(v) => v,
        };

        println!("- {:?}", caps);

        match caps.get(1) {
            None => {
                let val_text = caps.get(3).ok_or(DiceParseError::WrongFormat)?.as_str();
                let val: i32 = val_text.parse().unwrap_or(0);
                println!("- Not dice : {} = {}", val_text, val);
                Ok(DiceStep::Const(val))
            }
            Some(_) => {
                let count_text = caps.get(2).ok_or(DiceParseError::WrongFormat)?.as_str();
                let count: i32 = count_text.parse().unwrap_or(1);
                let sides_text = caps.get(3).ok_or(DiceParseError::WrongFormat)?.as_str();
                let sides: u32 = sides_text.parse().unwrap_or(1);
                println!(
                    "- Dice : {}d{} = {}d{}",
                    count_text, sides_text, count, sides
                );
                Ok(DiceStep::Die(count, sides))
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Dice {
    steps: Vec<DiceStep>,
}

impl Dice {
    pub fn new() -> Self {
        Dice { steps: Vec::new() }
    }

    pub fn simple(count: i32, sides: u32, plus: i32) -> Self {
        let mut steps = Vec::new();
        if plus != 0 {
            steps.push(DiceStep::Const(plus));
        }
        if count != 0 {
            steps.push(DiceStep::Die(count, sides));
        }
        Dice { steps }
    }

    pub fn push(&mut self, step: DiceStep) {
        self.steps.push(step);
    }
}

impl Display for Dice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for step in self.steps.iter() {
            write!(f, "{}", step)?;
        }
        Ok(())
    }
}

pub fn roll(dice: &Dice, rng: &mut RandomNumberGenerator) -> i32 {
    dice.steps.iter().fold(0, |out, step| out + step.roll(rng))
}

impl FromStr for Dice {
    type Err = DiceParseError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let mut is_neg = false;
        let mut next_neg: bool;
        let mut dice = Dice::new();

        for mut step in text.split_inclusive(&['+', '-']) {
            if step.ends_with('-') {
                step = step.strip_suffix('-').unwrap();
                next_neg = true;
            } else if step.ends_with('+') {
                step = step.strip_suffix('+').unwrap();
                next_neg = false;
            } else {
                println!("step - {} {}", is_neg, step);
                next_neg = false;
            }

            if step.len() > 0 {
                println!("step - {} {}", is_neg, step);
                let mut dice_step: DiceStep = step.parse()?;
                if is_neg {
                    dice_step = dice_step.to_neg();
                }
                dice.push(dice_step);
            }
            is_neg = next_neg;
        }

        Ok(dice)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn make() {
        let mut rng = RandomNumberGenerator::seeded(12345);

        let d = Dice::simple(1, 6, 2);
        assert_eq!(roll(&d, &mut rng), 4);
        assert_eq!(roll(&d, &mut rng), 6);
        assert_eq!(roll(&d, &mut rng), 7);
    }

    #[test]
    fn parse_step() {
        let step: DiceStep = "2d4".parse().unwrap();
        assert_eq!(step, DiceStep::Die(2, 4));

        let step: DiceStep = "d4".parse().unwrap();
        assert_eq!(step, DiceStep::Die(1, 4));

        let step: DiceStep = "4".parse().unwrap();
        assert_eq!(step, DiceStep::Const(4));

        assert_eq!(
            "abc".parse::<DiceStep>().err(),
            Some(DiceParseError::WrongFormat)
        );

        assert_eq!(
            "3c".parse::<DiceStep>().err(),
            Some(DiceParseError::WrongFormat)
        );
    }

    #[test]
    fn parse_dice() {
        let mut rng = RandomNumberGenerator::seeded(12345);
        let dice: Dice = "4".parse().unwrap();
        assert_eq!(roll(&dice, &mut rng), 4);

        let dice: Dice = "d4".parse().unwrap();
        assert_eq!(roll(&dice, &mut rng), 2);
        assert_eq!(roll(&dice, &mut rng), 4);
        assert_eq!(roll(&dice, &mut rng), 3);

        let dice: Dice = "2d4".parse().unwrap();
        assert_eq!(roll(&dice, &mut rng), 3);
        assert_eq!(roll(&dice, &mut rng), 5);
        assert_eq!(roll(&dice, &mut rng), 5);

        let dice: Dice = "2d4+10".parse().unwrap();
        assert_eq!(roll(&dice, &mut rng), 17);
        assert_eq!(roll(&dice, &mut rng), 17);
        assert_eq!(roll(&dice, &mut rng), 15);

        let dice: Dice = "20d1".parse().unwrap();
        assert_eq!(roll(&dice, &mut rng), 20);
        assert_eq!(roll(&dice, &mut rng), 20);
        assert_eq!(roll(&dice, &mut rng), 20);

        let dice: Dice = "20d1-5d1-5".parse().unwrap();
        assert_eq!(roll(&dice, &mut rng), 10);
        assert_eq!(roll(&dice, &mut rng), 10);
        assert_eq!(roll(&dice, &mut rng), 10);

        let dice: Dice = "+2".parse().unwrap();
        assert_eq!(roll(&dice, &mut rng), 2);
        assert_eq!(roll(&dice, &mut rng), 2);
        assert_eq!(roll(&dice, &mut rng), 2);

        let dice: Dice = "-2".parse().unwrap();
        assert_eq!(roll(&dice, &mut rng), -2);
        assert_eq!(roll(&dice, &mut rng), -2);
        assert_eq!(roll(&dice, &mut rng), -2);
    }
}
