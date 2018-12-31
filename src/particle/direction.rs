use super::angle;
use super::vector;
pub struct Direction {
    polar_angle: angle::Angle,
    azmuthal_angle: angle::Angle,
}

fn atan2(y: f64, x: f64) -> f64 {
    y.atan2(x)
}

impl Direction {
    pub fn new(p: angle::Angle, a: angle::Angle) -> Direction {
        Direction {
            polar_angle: p,
            azmuthal_angle: a,
        }
    }
    pub fn zero() -> Direction {
        Direction {
            polar_angle: angle::Angle::deg(0.0),
            azmuthal_angle: angle::Angle::deg(0.0),
        }
    }
    pub fn from(v: vector::Vector) -> Direction {
        let u = v.unit();
        let x = vector::dot(&u, &vector::Vector::i());
        let y = vector::dot(&u, &vector::Vector::j());
        let z = vector::dot(&u, &vector::Vector::k());
        let theta = z.acos();
        let phi = atan2(y, x);
        Direction {
            polar_angle: angle::Angle::rad(theta),
            azmuthal_angle: angle::Angle::rad(phi),
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
    pub fn oposite(&self) -> Direction {
        use super::modulo::Modulo;
        Direction {
            polar_angle: if self.polar_angle == angle::Angle::deg(0.0) {
                angle::Angle::deg(180.0)
            } else {
                (angle::Angle::deg(180.0) - self.polar_angle.clone())
                    .modulo(&angle::Angle::deg(180.0))
            },
            azmuthal_angle: (self.azmuthal_angle.clone() + angle::Angle::deg(180.0))
                .modulo(&angle::Angle::deg(360.0)),
        }
    }
    pub fn to_deg(&self) -> Direction {
        Direction {
            polar_angle: self.polar_angle.to_deg(),
            azmuthal_angle: self.azmuthal_angle.to_deg(),
        }
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

impl Clone for Direction {
    fn clone(&self) -> Self {
        Direction {
            polar_angle: self.polar_angle.clone(),
            azmuthal_angle: self.azmuthal_angle.clone(),
        }
    }
}
