use super::angle;
use super::vector;
pub struct Direction {
    polar_angle: angle::Angle,
    azmuthal_angle: angle::Angle,
}
impl Direction {
    pub fn new(p: angle::Angle, a: angle::Angle) -> Direction {
        Direction {
            polar_angle: p,
            azmuthal_angle: a,
        }
    }
    pub fn to_vec(&self) -> vector::Vector {
        let s = self.polar_angle.sin();
        vector::Vector::new(
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
