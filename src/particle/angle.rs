pub enum Angle {
    Radian(f64),
    Degree(f64),
    Gradian(f64),
    Circle(f64),
}

fn rad_to_deg(x: f64) -> f64 {
    use std::f64::consts::PI;
    x * 180.0 / PI
}

fn rad_to_grad(x: f64) -> f64 {
    use std::f64::consts::PI;
    x * 200.0 / PI
}

fn rad_to_circ(x: f64) -> f64 {
    use std::f64::consts::PI;
    x / (2.0 * PI)
}

fn deg_to_rad(x: f64) -> f64 {
    use std::f64::consts::PI;
    x * PI / 180.0
}

fn deg_to_grad(x: f64) -> f64 {
    x * 10.0 / 9.0
}

fn deg_to_circ(x: f64) -> f64 {
    x / 180.0
}

fn grad_to_rad(x: f64) -> f64 {
    use std::f64::consts::PI;
    x * PI / 200.0
}

fn grad_to_deg(x: f64) -> f64 {
    x * 9.0 / 10.0
}

fn grad_to_circ(x: f64) -> f64 {
    x / 400.0
}

fn circ_to_rad(x: f64) -> f64 {
    use std::f64::consts::PI;
    2.0 * PI * x
}

fn circ_to_deg(x: f64) -> f64 {
    x * 360.0
}

fn circ_to_grad(x: f64) -> f64 {
    x * 400.0
}

impl Angle {
    pub fn rad(v: f64) -> Angle {
        Angle::Radian(v)
    }
    pub fn deg(v: f64) -> Angle {
        Angle::Degree(v)
    }
    pub fn grad(v: f64) -> Angle {
        Angle::Gradian(v)
    }
    pub fn circ(v: f64) -> Angle {
        Angle::Circle(v)
    }

    pub fn to_rad(&self) -> Angle {
        match self {
            Angle::Radian(v) => Angle::Radian(*v),
            Angle::Degree(v) => Angle::Radian(deg_to_rad(*v)),
            Angle::Gradian(v) => Angle::Radian(grad_to_rad(*v)),
            Angle::Circle(v) => Angle::Radian(circ_to_rad(*v)),
        }
    }
    pub fn to_deg(&self) -> Angle {
        match self {
            Angle::Radian(v) => Angle::Degree(rad_to_deg(*v)),
            Angle::Degree(v) => Angle::Degree(*v),
            Angle::Gradian(v) => Angle::Degree(grad_to_deg(*v)),
            Angle::Circle(v) => Angle::Degree(circ_to_deg(*v)),
        }
    }
    pub fn to_grad(&self) -> Angle {
        match self {
            Angle::Radian(v) => Angle::Gradian(rad_to_grad(*v)),
            Angle::Degree(v) => Angle::Gradian(deg_to_grad(*v)),
            Angle::Gradian(v) => Angle::Gradian(*v),
            Angle::Circle(v) => Angle::Gradian(circ_to_grad(*v)),
        }
    }
    pub fn to_circ(&self) -> Angle {
        match self {
            Angle::Radian(v) => Angle::Circle(rad_to_circ(*v)),
            Angle::Degree(v) => Angle::Circle(deg_to_circ(*v)),
            Angle::Gradian(v) => Angle::Circle(grad_to_circ(*v)),
            Angle::Circle(v) => Angle::Circle(*v),
        }
    }
    pub fn sin(&self) -> f64 {
        match self {
            Angle::Radian(v) => f64::sin(*v),
            Angle::Degree(v) => f64::sin(deg_to_rad(*v)),
            Angle::Gradian(v) => f64::sin(grad_to_rad(*v)),
            Angle::Circle(v) => f64::sin(circ_to_rad(*v)),
        }
    }
    pub fn cos(&self) -> f64 {
        match self {
            Angle::Radian(v) => f64::cos(*v),
            Angle::Degree(v) => f64::cos(deg_to_rad(*v)),
            Angle::Gradian(v) => f64::cos(grad_to_rad(*v)),
            Angle::Circle(v) => f64::cos(circ_to_rad(*v)),
        }
    }
}

use std::fmt;
impl fmt::Display for Angle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Angle::Radian(v) => write!(f, "{}r", v),
            Angle::Degree(v) => write!(f, "{}°", v),
            Angle::Gradian(v) => write!(f, "{}g", v),
            Angle::Circle(v) => write!(f, "{}", v),
        }
    }
}

impl fmt::Debug for Angle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Angle::Radian(v) => write!(f, "Radian({})", v),
            Angle::Degree(v) => write!(f, "Degree({})", v),
            Angle::Gradian(v) => write!(f, "Gradian({})", v),
            Angle::Circle(v) => write!(f, "Circle({})", v),
        }
    }
}

impl PartialEq for Angle {
    fn eq(&self, other: &Angle) -> bool {
        match self {
            Angle::Radian(x) => match other {
                Angle::Radian(y) => *x == *y,
                Angle::Degree(y) => rad_to_deg(*x) == *y,
                Angle::Gradian(y) => rad_to_grad(*x) == *y,
                Angle::Circle(y) => rad_to_circ(*x) == *y,
            },
            Angle::Degree(x) => match other {
                Angle::Radian(y) => deg_to_rad(*x) == *y,
                Angle::Degree(y) => *x == *y,
                Angle::Gradian(y) => deg_to_grad(*x) == *y,
                Angle::Circle(y) => deg_to_circ(*x) == *y,
            },
            Angle::Gradian(x) => match other {
                Angle::Radian(y) => grad_to_rad(*x) == *y,
                Angle::Degree(y) => grad_to_deg(*x) == *y,
                Angle::Gradian(y) => *x == *y,
                Angle::Circle(y) => grad_to_circ(*x) == *y,
            },
            Angle::Circle(x) => match other {
                Angle::Radian(y) => circ_to_rad(*x) == *y,
                Angle::Degree(y) => circ_to_deg(*x) == *y,
                Angle::Gradian(y) => circ_to_grad(*x) == *y,
                Angle::Circle(y) => *x == *y,
            },
        }
    }
}

impl Clone for Angle {
    fn clone(&self) -> Self {
        match self {
            Angle::Radian(v) => Angle::Radian(*v),
            Angle::Degree(v) => Angle::Degree(*v),
            Angle::Gradian(v) => Angle::Gradian(*v),
            Angle::Circle(v) => Angle::Circle(*v),
        }
    }
}

#[test]
fn test_angle() {
    use hamcrest::prelude::*;
    assert_that!(Angle::rad(0.1), is(equal_to(Angle::rad(0.1))));
}
