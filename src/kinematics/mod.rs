trait Solvable {
    type Solution;
    // Static method signature; `Self` refers to the implementor type.
    fn solve(&self) -> Option<solution>;
}

struct Vector {}

struct FourVector {}

struct TwoBodyDecayWithKnownCentreOfMassAngle {
    parent_mass: f64,
    parent_energy: f64,
    child_one_mass: f64,
    child_two_mass: f64,
    polar_angle: f64,
    azmuthal_angle: f64,
}

struct TwoBodyDecayWithKnownCentreOfMassAngleSolution {}

enum Problem {}
