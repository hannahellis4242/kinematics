pub struct Angle {
    value: f64,
}

impl Angle {
    pub fn rad(v: f64) -> Angle {
        Angle { value: v }
    }
    pub fn deg(v: f64) -> Angle {
        use std::f64::consts::PI;
        Angle {
            value: v * PI / 180.0,
        }
    }
    pub fn grad(v: f64) -> Angle {
        use std::f64::consts::PI;
        Angle {
            value: v * PI / 200.0,
        }
    }
    pub fn circ(v: f64) -> Angle {
        use std::f64::consts::PI;
        Angle {
            value: v * 2.0 * PI,
        }
    }
    pub fn sin(&self) -> f64 {
        f64::sin(self.value)
    }
    pub fn cos(&self) -> f64 {
        f64::cos(self.value)
    }
}

use std::fmt;
impl fmt::Display for Angle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} r", self.value)
    }
}

impl fmt::Debug for Angle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Angle {{ value: {} }}", self.value)
    }
}

impl PartialEq for Angle {
    fn eq(&self, other: &Angle) -> bool {
        self.value == other.value
    }
}

#[test]
fn test_angle() {
    use hamcrest::prelude::*;
    assert_that!(Angle::rad(0.1), is(equal_to(Angle { value: 0.1 })));
}
