//!  The FIREPLACEv1.0 allows the use of the `esb` tooling for solving Advent of Code problems.
//!  This is an implementation of FIREPLACEv1.0 for Rust.
//!
//!  Check [esb](https://github.com/luxedo/esb) for more information.

use std::error::Error;
use std::fmt::Display;
use std::io;
use std::str::FromStr;
use std::time::Instant;

/// Handles incorrect command usage
#[derive(thiserror::Error, Debug)]
pub enum FireplaceError {
    #[error(transparent)]
    IO(#[from] io::Error),
    #[error("Invalid part, please use 1 or 2 as argument for --part flag.")]
    InvalidPart,
    #[error("Missing part, please use 1 or 2 as argument for --part flag.")]
    MissingPart,
    #[error("{0}")]
    FromUser(String),
}

/// Return value for the AoC solution functions
pub type FireplaceResult<T> = Result<T, FireplaceError>;

enum AoCPart {
    Pt1,
    Pt2,
}

impl FromStr for AoCPart {
    type Err = FireplaceError;

    fn from_str(s: &str) -> FireplaceResult<Self> {
        match s {
            "1" => Ok(Self::Pt1),
            "2" => Ok(Self::Pt2),
            _ => Err(FireplaceError::InvalidPart),
        }
    }
}

trait InputReader {
    fn load_fireplace_input(&mut self) -> FireplaceResult<String>;
}

impl InputReader for io::Stdin {
    fn load_fireplace_input(&mut self) -> FireplaceResult<String> {
        let stdin = io::read_to_string(self)?;
        Ok(stdin)
    }
}

struct FireplaceArgs {
    part: AoCPart,
    args: Vec<String>,
}

impl TryFrom<clap::ArgMatches> for FireplaceArgs {
    type Error = FireplaceError;

    fn try_from<'a>(matches: clap::ArgMatches) -> Result<Self, Self::Error> {
        let Some(part) = matches.get_one::<String>("part") else {
            return Err(FireplaceError::MissingPart);
        };
        let part = part.parse::<AoCPart>()?;
        let args: Vec<String> = matches
            .get_many::<String>("args")
            .unwrap_or_default()
            .map(|v| v.into())
            .collect();

        Ok(Self { part, args })
    }
}

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
                .help("Additional arguments for running the solutions")
                .num_args(0..)
        )
}

/// Contains the solution for pt1 or for pt2
pub enum Either<T, U> {
    Part1(T),
    Part2(U),
}

impl<T: Display, U: Display> Display for Either<T, U> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Either::Part1(t) => write!(f, "{}", t),
            Either::Part2(u) => write!(f, "{}", u),
        }
    }
}

/// Runs the solution functions in compliance with FIREPLACEv1 protocol
pub fn v1_run<T, E1, U, E2>(
    solve_pt1: impl Fn(&str, Vec<String>) -> Result<T, E1>,
    solve_pt2: impl Fn(&str, Vec<String>) -> Result<U, E2>,
) -> FireplaceResult<Either<T, U>>
where
    T: Display + 'static,
    U: Display + 'static,
    E1: Error,
    E2: Error,
{
    let parser_matches = parser().get_matches();
    let fp_args = FireplaceArgs::try_from(parser_matches)?;
    run(&solve_pt1, &solve_pt2, io::stdin(), fp_args)
}

fn run<T, E1, U, E2>(
    solve_pt1: impl Fn(&str, Vec<String>) -> Result<T, E1>,
    solve_pt2: impl Fn(&str, Vec<String>) -> Result<U, E2>,
    mut input_reader: impl InputReader,
    fp_args: FireplaceArgs,
) -> FireplaceResult<Either<T, U>>
where
    T: Display + 'static,
    U: Display + 'static,
    E1: Error,
    E2: Error,
{
    let input_data = input_reader.load_fireplace_input()?;
    let start = Instant::now();
    let answer = match fp_args.part {
        AoCPart::Pt1 => solve_pt1(&input_data, fp_args.args)
            .map_err(|e| FireplaceError::FromUser(e.to_string()))
            .map(Either::Part1),
        AoCPart::Pt2 => solve_pt2(&input_data, fp_args.args)
            .map_err(|e| FireplaceError::FromUser(e.to_string()))
            .map(Either::Part2),
    };
    let duration = start.elapsed();

    match answer {
        Ok(answer) => {
            println!("{}", answer);
            println!("RT {} ns", duration.as_nanos());
            Ok(answer)
        }
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestInputReader;
    impl InputReader for TestInputReader {
        fn load_fireplace_input(&mut self) -> FireplaceResult<String> {
            Ok("sample input".into())
        }
    }

    fn solve_pt1(input_data: &str, args: Vec<String>) -> FireplaceResult<String> {
        Ok(match args.len() {
            0 => input_data.trim().into(),
            _ => args.join(" "),
        })
    }

    const PT2_RETURN: &str = "2";
    fn solve_pt2(_input_data: &str, _args: Vec<String>) -> FireplaceResult<String> {
        Ok(PT2_RETURN.into())
    }

    fn test_runner(fp_args: FireplaceArgs) -> FireplaceResult<impl Display> {
        super::run(solve_pt1, solve_pt2, TestInputReader, fp_args)
    }

    #[test]
    fn test_calls_solve_pt1() {
        let fp_args = FireplaceArgs {
            part: AoCPart::Pt1,
            args: vec![],
        };
        let answer = test_runner(fp_args).unwrap();
        let expected = "sample input";
        assert_eq!(answer.to_string(), expected);
    }

    #[test]
    fn test_calls_solve_pt1_with_args() {
        let fp_args = FireplaceArgs {
            part: AoCPart::Pt1,
            args: vec!["a".into(), "b".into(), "c".into()],
        };
        let answer = test_runner(fp_args).unwrap();
        let expected = "a b c";
        assert_eq!(answer.to_string(), expected);
    }

    #[test]
    fn test_calls_solve_pt2() {
        let fp_args = FireplaceArgs {
            part: AoCPart::Pt2,
            args: vec![],
        };
        let answer = test_runner(fp_args).unwrap();
        assert_eq!(answer.to_string(), PT2_RETURN);
    }

    #[test]
    // Check if the error is converted to a FireplaceError::FromUser
    fn test_error_conversion() {
        let some_aoc_function =
            |_: &str, _: Vec<String>| -> Result<String, std::fmt::Error> { Err(std::fmt::Error) };
        let fp_args = FireplaceArgs {
            part: AoCPart::Pt1,
            args: vec![],
        };
        let result = super::run(some_aoc_function, solve_pt2, TestInputReader, fp_args);
        let Err(e) = result else {
            panic!("Expected an error");
        };
        assert!(matches!(e, FireplaceError::FromUser(_)));
    }
}
