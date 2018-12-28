mod angle;
use crate::four_vector::direction::angle::Angle;
struct Direction {
    polar_angle: Angle,
    azmuthal_angle: Angle,
}
mod vector;
use crate::four_vector::direction::vector::Vector;
impl Direction {
    fn new(p: Angle, a: Angle) -> Direction {
        Direction {
            polar_angle: p,
            azmuthal_angle: a,
        }
    }
    fn to_vec(&self) -> Vector {
        let s = self.polar_angle.sin();
        Vector::new(
            s * self.azmuthal_angle.cos(),
            s * self.azmuthal_angle.sin(),
            self.polar_angle.cos(),
        )
    }
}

use std::fmt;
impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{},{}>", self.polar_angle, self.azmuthal_angle)
    }
}

impl fmt::Debug for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Direction {{ polar_angle: {} , azmuthal_angle: {} }}",
            self.polar_angle, self.azmuthal_angle
        )
    }
}

impl PartialEq for Direction {
    fn eq(&self, other: &Direction) -> bool {
        self.polar_angle == other.polar_angle && self.azmuthal_angle == other.azmuthal_angle
    }
}
