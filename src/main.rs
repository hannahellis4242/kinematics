extern crate clap;
#[cfg(test)]
#[macro_use]
extern crate hamcrest;

mod vector {
    use std::fmt;
    use std::ops::Add;
    use std::ops::Div;
    use std::ops::Mul;
    use std::ops::Neg;
    use std::ops::Sub;

    struct Vector {
        x: f64,
        y: f64,
        z: f64,
    }

    impl Vector {
        fn new(x: f64, y: f64, z: f64) -> Vector {
            Vector { x: x, y: y, z: z }
        }
        fn zeros() -> Vector {
            Vector {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }
        }
    }

    impl fmt::Display for Vector {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({}, {}, {})", self.x, self.y, self.z)
        }
    }

    impl fmt::Debug for Vector {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "Vector {{ x: {}, y: {} , z:{} }}",
                self.x, self.y, self.z
            )
        }
    }

    impl PartialEq for Vector {
        fn eq(&self, other: &Vector) -> bool {
            self.x == other.x && self.y == other.y && self.z == other.z
        }
    }

    impl Add for Vector {
        type Output = Vector;

        fn add(self, other: Vector) -> Vector {
            Vector {
                x: self.x + other.x,
                y: self.y + other.y,
                z: self.z + other.z,
            }
        }
    }

    impl Sub for Vector {
        type Output = Vector;

        fn sub(self, other: Vector) -> Vector {
            Vector {
                x: self.x - other.x,
                y: self.y - other.y,
                z: self.z - other.z,
            }
        }
    }

    impl Neg for Vector {
        type Output = Vector;

        fn neg(self) -> Vector {
            Vector {
                x: -self.x,
                y: -self.y,
                z: -self.z,
            }
        }
    }

    impl Mul<f64> for Vector {
        type Output = Vector;
        fn mul(self, rhs: f64) -> Self {
            Vector {
                x: rhs * self.x,
                y: rhs * self.y,
                z: rhs * self.z,
            }
        }
    }
    impl Mul<Vector> for f64 {
        type Output = Vector;
        fn mul(self, rhs: Vector) -> Vector {
            Vector {
                x: rhs.x * self,
                y: rhs.y * self,
                z: rhs.z * self,
            }
        }
    }

    impl Div<f64> for Vector {
        type Output = Vector;

        fn div(self, rhs: f64) -> Vector {
            if rhs == 0.0 {
                panic!("Cannot divide by zero");
                Vector::zeros()
            } else {
                Vector {
                    x: self.x / rhs,
                    y: self.y / rhs,
                    z: self.z / rhs,
                }
            }
        }
    }

    fn dot(a: &Vector, b: &Vector) -> f64 {
        a.x * b.x + a.y * b.y + a.z * b.z
    }

    fn cross(a: &Vector, b: &Vector) -> Vector {
        Vector {
            x: a.y * b.z - a.z * b.y,
            y: a.z * b.x - a.x * b.z,
            z: a.x * b.y - a.y * b.x,
        }
    }

    #[test]
    fn test_format() {
        use hamcrest::prelude::*;
        let v = Vector::zeros();
        assert_that!(&format!("{}", v), is(equal_to("(0, 0, 0)")));
    }

    #[test]
    fn test_equal() {
        use hamcrest::prelude::*;
        assert_that!(
            Vector::new(1.0, 0.0, 0.0),
            is(equal_to(Vector::new(1.0, 0.0, 0.0)))
        );
        assert_that!(
            Vector::new(1.0, 0.0, 0.0),
            is(not(equal_to(Vector::new(2.0, 0.0, 0.0))))
        );
        assert_that!(
            Vector::new(1.0, 0.0, 0.0),
            is(not(equal_to(Vector::new(1.0, 1.0, 0.0))))
        );
        assert_that!(
            Vector::new(1.0, 0.0, 0.0),
            is(not(equal_to(Vector::new(1.0, 0.0, 1.0))))
        );
    }
    #[test]
    fn test_add() {
        use hamcrest::prelude::*;
        assert_that!(
            Vector::new(1.0, 0.0, 0.0) + Vector::new(4.0, 0.0, 0.0),
            is(equal_to(Vector::new(5.0, 0.0, 0.0)))
        );
        assert_that!(
            Vector::new(1.0, 0.0, 0.0) + Vector::new(0.0, 1.0, 0.0),
            is(equal_to(Vector::new(1.0, 1.0, 0.0)))
        );
    }
    #[test]
    fn test_subtract() {
        use hamcrest::prelude::*;
        assert_that!(
            Vector::new(1.0, 0.0, 0.0) - Vector::new(4.0, 0.0, 0.0),
            is(equal_to(Vector::new(-3.0, 0.0, 0.0)))
        );
        assert_that!(
            Vector::new(1.0, 0.0, 0.0) - Vector::new(0.0, 1.0, 0.0),
            is(equal_to(Vector::new(1.0, -1.0, 0.0)))
        );
    }
    #[test]
    fn test_negate() {
        use hamcrest::prelude::*;
        assert_that!(
            -Vector::new(1.0, 0.0, 0.0),
            is(equal_to(Vector::new(-1.0, 0.0, 0.0)))
        );
        assert_that!(
            -Vector::new(1.0, -1.0, 0.0),
            is(equal_to(Vector::new(-1.0, 1.0, 0.0)))
        );
    }
    #[test]
    fn test_scale() {
        use hamcrest::prelude::*;
        assert_that!(
            5.0 * Vector::new(1.0, 0.0, 0.0),
            is(equal_to(Vector::new(5.0, 0.0, 0.0)))
        );
        assert_that!(
            3.0 * Vector::new(1.0, -1.0, 0.0),
            is(equal_to(Vector::new(3.0, -3.0, 0.0)))
        );
        assert_that!(
            Vector::new(1.0, -1.0, 2.0) * 2.0,
            is(equal_to(Vector::new(2.0, -2.0, 4.0)))
        );

        assert_that!(
            Vector::new(1.0, 0.0, 0.0) / 5.0,
            is(equal_to(Vector::new(0.2, 0.0, 0.0)))
        );
        assert_that!(
            Vector::new(1.0, -1.0, 0.0) / 2.0,
            is(equal_to(Vector::new(0.5, -0.5, 0.0)))
        );
        assert_that!(
            Vector::new(1.0, -1.0, 2.0) / 2.0,
            is(equal_to(Vector::new(0.5, -0.5, 1.0)))
        );
    }
    #[test]
    fn test_dot() {
        use hamcrest::prelude::*;
        assert_that!(
            dot(&Vector::new(5.0, 3.0, 4.0), &Vector::new(3.0, -2.0, 2.0)),
            is(equal_to(17.0))
        );
    }
    #[test]
    fn test_cross() {
        use hamcrest::prelude::*;
        assert_that!(
            cross(&Vector::new(5.0, 3.0, 4.0), &Vector::new(3.0, -2.0, 2.0)),
            is(equal_to(Vector::new(14.0, 2.0, -19.0)))
        );
    }
}

fn main() {}
/*fn main() {
    use clap::{App, Arg, SubCommand};
    use std::io::ErrorKind;
    match App::new("kinematics")
        .version("0.1.0")
        .author("Hannah Ellis <hannahellis4242@gmail.com>")
        .about("Solves relativistic kinematics problems")
        .subcommand(
            SubCommand::with_name("two_body_decay")
                .about("solves the two body decay problem with the given configuration file")
                .version("0.1.0")
                .author("Hannah Ellis <hannahellis4242@gmail.com>")
                .arg(
                    Arg::with_name("file")
                        .short("f")
                        .long("input")
                        .value_name("FILE")
                        .help("problem file")
                        .takes_value(true)
                        .required(true),
                ),
        )
        .get_matches()
        .subcommand()
    {
        ("two_body_decay", Some(matches)) => matches
            .value_of("file")
            .ok_or::<std::io::Error>(std::io::Error::new(
                ErrorKind::Other,
                "missing file argument",
            ))
            .and_then(|file_name| std::fs::read_to_string(file_name))
            //.map(|text| println!("{}", text))
            .and_then(|text| {
                sudoku::json::read_problem(&text)
                    .map_err(|e| std::io::Error::new(ErrorKind::Other, e))
            })
            .map(|p| (p.clone(), sudoku::solve(p)))
            .map(|(problem, solutions)| {
                solutions
                    .iter()
                    .map(|solution| sudoku::json::write_solution(&problem, solution.as_slice()))
                    .for_each(|text| println!("{}", text))
            })
            .map_err(|e| println!("{}", e))
            .unwrap_or(()), // solve was used
        _ => println!("{}", "missing subcommand"), // Either no subcommand or one not tested for...
    }
}*/
