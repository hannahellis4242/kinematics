use std::fmt;
mod direction;
struct FourVector {
    energy: f64,
    momentum: f64,
}

impl FourVector {
    fn new(e: f64, m: f64) -> Option<FourVector> {
        if e >= m {
            Some(FourVector {
                energy: e,
                momentum: f64::sqrt(e * e - m * m),
            })
        } else {
            None
        }
    }
}

impl fmt::Display for FourVector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.energy, self.momentum)
    }
}

impl fmt::Debug for FourVector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "FourVector {{ energy: {}, momentum: {} }}",
            self.energy, self.momentum
        )
    }
}

impl PartialEq for FourVector {
    fn eq(&self, other: &FourVector) -> bool {
        self.energy == other.energy && self.momentum == other.momentum
    }
}

#[test]
fn test_eq() {
    use hamcrest::prelude::*;
    match FourVector::new(1.0, 1.0) {
        Some(v) => {
            assert_that!(
                v,
                is(equal_to(FourVector {
                    energy: 1.0,
                    momentum: 0.0
                }))
            );
        }
        None => panic!("None"),
    }
}
