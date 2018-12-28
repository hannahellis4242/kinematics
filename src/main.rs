extern crate clap;
#[cfg(test)]
#[macro_use]
extern crate hamcrest;

mod four_vector;
fn main() {
    let v = four_vector::FourVector::new(
        1.0,
        2.0,
        four_vector::direction::Direction::new(
            four_vector::direction::angle::Angle::deg(0.0),
            four_vector::direction::angle::Angle::deg(0.0),
        ),
    );
    println!("{:?}", v);
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
