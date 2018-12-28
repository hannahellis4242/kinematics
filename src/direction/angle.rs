pub enum Angle {
    Radian(f64),
    Degree(f64),
    Gradian(f64),
    Circle(f64),
}

impl Angle {
    pub fn rad(v: f64) -> Angle {
        Angle::Radian(v)
    }
    pub fn deg(v:f64)->Angle{
        Angle::Degree(v)
    }
    pub fn grad(v:f64)->Angle{
        Angle::Gradian(v)
    }
    pub fn circ(v:f64)->Angle{
        Angle::Circle(v)
    }

    pub to_rad(&self)->Angle::Radian{
        use std::f64::consts::PI;
        match self{
            Radian(v)=>Radian(v),
            Degree(v)=>Radian(v*PI/180.0),
            Gradian(v)=>Radian(v*PI/200),
            Circle(v)=>Radian(2*v*PI),
        }
    }
    pub to_deg(&self)->Angle::Degree{
        use std::f64::consts::PI;
        match self{
            Radian(v)=>Degree(v*180.0/PI),
            Degree(v)=>Degree(v),
            Gradian(v)=>Degree(v*36.0/40.0),
            Circle(v)=>Degree(v*360.0),
        }
    }
    pub to_grad(&self)->Angle::Grad{
        use std::f64::consts::PI;
        match self{
            Radian(v)=>Grad(v*200.0/PI),
            Degree(v)=>Grad(v*40/36),
            Gradian(v)=>Grad(v),
            Circle(v)=>Grad(v*400.0),
        }
    }
    pub to_circ(&self)->Angle::Circle{
        use std::f64::consts::PI;
        match self{
            Radian(v)=>Circle(2*v/PI),
            Degree(v)=>Circle(v/360.0),
            Gradian(v)=>Circle(v/400.0),
            Circle(v)=>Circle(v),
        }
    }
    pub fn sin(&self) -> f64 {
        use std::f64::consts::PI;
        match self{
            Radian(v)=>f64::sin(v),
            Degree(v)=>f64::sin(v*PI/180.0),
            Gradian(v)=>f64::sin(v*PI/200),
            Circle(v)=>f64::sin(2*v*PI),
        }
    }
    pub fn cos(&self) -> f64 {
        use std::f64::consts::PI;
        match self{
            Radian(v)=>f64::cos(v),
            Degree(v)=>f64::cos(v*PI/180.0),
            Gradian(v)=>f64::cos(v*PI/200),
            Circle(v)=>f64::cos(2*v*PI),
        }
    }
}

use std::fmt;
impl fmt::Display for Angle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self{
            Radian(v)=>write!(f, "{}r", self.value),
            Degree(v)=>write!(f, "{}°", self.value),
            Gradian(v)=>write!(f, "{}g", self.value),
            Circle(v)=>write!(f, "{}", self.value),
        }
    }
}

impl fmt::Debug for Angle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self{
            Radian(v)=>write!(f, "Radian({})", self.value),
            Degree(v)=>write!(f, "Degree({})", self.value),
            Gradian(v)=>write!(f, "Gradian({})", self.value),
            Circle(v)=>write!(f, "Circle({})", self.value),
        }
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
