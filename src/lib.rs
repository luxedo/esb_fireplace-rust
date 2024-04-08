/*
*  The FIREPLACEv1.0 allows the use of the `esb` tooling for solving Advent of Code problems.
*  This is an implementation of FIREPLACEv1.0 for Rust.
*
*  Check [esb](https://github.com/luxedo/esb) for more information.
*/
use std::fmt::Display;
use std::io;
use std::str::FromStr;

enum AoCPart {
    Pt1,
    Pt2,
}

impl FromStr for AoCPart {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(Self::Pt1),
            "2" => Ok(Self::Pt2),
            _ => Err(()),
        }
    }
}

trait FireplaceSolver<T> {
    fn solve(&self, input: &str, args: Option<Vec<String>>) -> T;
}

impl<T, F> FireplaceSolver<T> for F
where
    F: Fn(&str, Option<Vec<String>>) -> T,
    T: Display,
{
    fn solve(&self, input: &str, args: Option<Vec<String>>) -> T {
        self(input, args)
    }
}

struct FireplaceV1 {
    part: AoCPart,
    args: Option<Vec<String>>,
}

impl FireplaceV1 {
    fn parser() -> clap::Command {
        clap::Command::new("esb_fireplace")
        .about("Script your way to rescue Christmas as part of the ElfScript Brigade team. `esb` is a CLI tool to help us _elves_ to save Christmas for the [Advent Of Code](https://adventofcode.com/)yearly events (Thank you [Eric 😉!](https://twitter.com/ericwastl)). For more information visit https://github.com/luxedo/esb")
        .arg(
            clap::Arg::new("part")
                .short('p')
                .long("part")
                .help("Run solution part 1 or part 2")
                .value_parser(["1", "2"])
                .required(true),
        )
        .arg(
            clap::Arg::new("args")
                .short('a')
                .long("args")
                .num_args(0..)
                .help("Additional arguments for running the solutions")
        )
    }

    fn process(matches: clap::ArgMatches) -> Self {
        let part: AoCPart = matches.get_one::<String>("part").unwrap().parse().unwrap();
        let args: Option<Vec<String>> =
            if let Ok(Some(args_vec)) = matches.try_get_many::<String>("args") {
                Some(args_vec.cloned().collect())
            } else {
                None
            };
        Self { part, args }
    }

    fn load_input_data() -> io::Result<String> {
        let stdin = io::read_to_string(io::stdin())?;
        Ok(stdin)
    }

    fn run<F, G, T, U>(
        solve_pt1: F,
        solve_pt2: G,
        input_data: String,
        fp_args: FireplaceV1,
    ) -> Box<dyn Display>
    where
        F: FireplaceSolver<T>,
        G: FireplaceSolver<U>,
        T: Display + 'static,
        U: Display + 'static,
    {
        match fp_args.part {
            AoCPart::Pt1 => Box::new(solve_pt1.solve(&input_data, fp_args.args)),
            AoCPart::Pt2 => Box::new(solve_pt2.solve(&input_data, fp_args.args)),
        }
    }

    pub fn run_solutions<F, G, T, U>(solve_pt1: F, solve_pt2: G) -> io::Result<()>
    where
        F: FireplaceSolver<T>,
        G: FireplaceSolver<U>,
        T: Display + 'static,
        U: Display + 'static,
    {
        let parser = Self::parser();
        let fp_args = Self::process(parser.get_matches());
        let input_data = Self::load_input_data()?;
        let answer = Self::run(solve_pt1, solve_pt2, input_data, fp_args);
        println!("{}", answer);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn solve_pt1(input_data: &str, args: Option<Vec<String>>) -> String {
        if let Some(a) = args {
            a.join(" ")
        } else {
            input_data.trim().into()
        }
    }

    const PT2_RETURN: &str = "2";
    fn solve_pt2(_input_data: &str, _args: Option<Vec<String>>) -> String {
        PT2_RETURN.into()
    }

    fn test_runner(cmd_args: Vec<&str>, input_data: &str) -> Box<dyn Display> {
        let parser = FireplaceV1::parser();
        let matches = parser.get_matches_from(cmd_args);
        let fp_args = FireplaceV1::process(matches);
        FireplaceV1::run(solve_pt1, solve_pt2, input_data.into(), fp_args)
    }

    #[test]
    fn test_calls_solve_pt1() -> () {
        let cmd_args = vec!["prog", "--part", "1"];
        let input_data = "sample input";
        let answer = test_runner(cmd_args, input_data);
        assert_eq!(input_data, answer.to_string());
    }

    #[test]
    fn test_calls_solve_pt2() -> () {
        let cmd_args = vec!["prog", "--part", "2"];
        let input_data = "sample input";
        let answer = test_runner(cmd_args, input_data);
        assert_eq!(PT2_RETURN, answer.to_string());
    }
}
