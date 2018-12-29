use std::fmt;
pub mod angle;
pub mod direction;
pub mod vector;
pub struct Particle {
    name: String,
    mass: f64,
    energy: f64,
    momentum: f64,
    direction: direction::Direction,
}

impl Particle {
    pub fn new(n: &str, m: f64, e: f64, d: direction::Direction) -> Option<Particle> {
        if e >= m {
            Some(Particle {
                name: n.to_string(),
                mass: m,
                energy: e,
                momentum: f64::sqrt(e * e - m * m),
                direction: d,
            })
        } else {
            None
        }
    }
    pub fn rest(n: &str, m: f64) -> Particle {
        Particle {
            name: n.to_string(),
            mass: m,
            energy: m,
            momentum: 0.0,
            direction: direction::Direction::zero(),
        }
    }

    pub fn beta(&self) -> vector::Vector {
        (self.momentum / self.energy) * self.direction.to_vec()
    }

    pub fn beta_mag(&self) -> f64 {
        self.momentum / self.energy
    }

    pub fn gamma(&self) -> f64 {
        self.energy / self.mass
    }

    pub fn gammabeta(&self) -> f64 {
        self.momentum / self.mass
    }
}

impl fmt::Display for Particle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "particle : {}
        \t mass : {} eV.c^-2
        \t energy : {} eV.c^-2
        \t momentum : {} eV.c^-1
        \t direction : {}",
            self.name, self.mass, self.energy, self.momentum, self.direction
        )
    }
}

impl fmt::Debug for Particle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Particle {{ name: {}, mass: {}, energy: {}, momentum: {} , direction: {}}}",
            self.name, self.mass, self.energy, self.momentum, self.direction
        )
    }
}

impl Clone for Particle {
    fn clone(&self) -> Self {
        Particle {
            name: self.name.clone(),
            mass: self.mass.clone(),
            energy: self.energy.clone(),
            momentum: self.momentum.clone(),
            direction: self.direction.clone(),
        }
    }
}

fn daughter_cm_energy(parent_mass: f64, daughter_mass: f64, daughter_other_mass: f64) -> f64 {
    (parent_mass * parent_mass + daughter_mass * daughter_mass
        - daughter_other_mass * daughter_other_mass)
        / (2.0 * parent_mass)
}

pub fn two_body_decay(
    parent_mass: f64,
    daughter_1_mass: f64,
    daughter_2_mass: f64,
    daughter_1_direction: direction::Direction,
) -> (Particle, Option<Particle>, Option<Particle>) {
    let parent = Particle::rest("parent cm", parent_mass);
    let daughter_1_energy = daughter_cm_energy(parent_mass, daughter_1_mass, daughter_2_mass);
    let daughter_2_energy = daughter_cm_energy(parent_mass, daughter_2_mass, daughter_1_mass);
    let oposite = daughter_1_direction.oposite();
    (
        parent,
        Particle::new(
            "daughter_1",
            daughter_1_mass,
            daughter_1_energy,
            daughter_1_direction,
        ),
        Particle::new("daughter_2", daughter_2_mass, daughter_2_energy, oposite),
    )
}
