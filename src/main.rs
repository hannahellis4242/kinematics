extern crate clap;
#[cfg(test)]
#[macro_use]
extern crate hamcrest;

mod particle;
fn main() {
    let energy = 3.0;
    let mass = 2.0;
    particle::Particle::new(
        "parent lab",
        mass,
        energy,
        particle::direction::Direction::zero(),
    )
    .map(|lab_parent| {
        match particle::two_body_decay(
            2.0,
            0.5,
            0.5,
            particle::direction::Direction::new(
                particle::angle::Angle::deg(90.0),
                particle::angle::Angle::deg(0.0),
            ),
        ) {
            (parent, Some(daughter_1), Some(daughter_2)) => {
                println!("Centre of mass frame");
                println!("--------------------");
                println!("parent : {}", parent);
                println!("daughter_1 : {}", daughter_1);
                println!("daughter_2 : {}", daughter_2);
                println!("====================");

                let gamma = lab_parent.gamma();
                let gammabeta = lab_parent.gammabeta();
                let boost_direction = particle::direction::Direction::zero().oposite();
                println!("lab frame");
                println!("--------------------");
                println!(
                    "parent lab : {}",
                    parent.lorentz(gamma, gammabeta, boost_direction.clone())
                );
                println!(
                    "daughter_1 {}",
                    daughter_1.lorentz(gamma, gammabeta, boost_direction.clone())
                );
                println!(
                    "daughter_2 {}",
                    daughter_2.lorentz(gamma, gammabeta, boost_direction)
                );
                println!("====================");
            }
            _ => println!("Error"),
        }
    });
}
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
