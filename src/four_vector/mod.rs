use std::fmt;
pub mod direction;
use crate::four_vector::direction::Direction;
pub struct FourVector {
    energy: f64,
    momentum: f64,
    direction: Direction,
}

impl FourVector {
    pub fn new(e: f64, m: f64, d: Direction) -> Option<FourVector> {
        if e >= m {
            Some(FourVector {
                energy: e,
                momentum: f64::sqrt(e * e - m * m),
                direction: d,
            })
        } else {
            None
        }
    }
}

impl fmt::Display for FourVector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {},{})", self.energy, self.momentum, self.direction)
    }
}

impl fmt::Debug for FourVector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "FourVector {{ energy: {}, momentum: {} , direction: {}}}",
            self.energy, self.momentum, self.direction
        )
    }
}

impl PartialEq for FourVector {
    fn eq(&self, other: &FourVector) -> bool {
        self.energy == other.energy
            && self.momentum == other.momentum
            && self.direction == other.direction
    }
}

#[test]
fn test_eq() {
    use hamcrest::prelude::*;
    match FourVector::new(
        1.0,
        1.0,
        direction::Direction::new(
            direction::angle::Angle::deg(0.0),
            direction::angle::Angle::deg(0.0),
        ),
    ) {
        Some(v) => {
            assert_that!(
                v,
                is(equal_to(FourVector {
                    energy: 1.0,
                    momentum: 0.0,
                    direction: direction::Direction::new(
                        direction::angle::Angle::rad(0.0),
                        direction::angle::Angle::rad(0.0)
                    ),
                }))
            );
        }
        None => panic!("None"),
    }
}
