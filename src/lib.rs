/*
*  The FIREPLACEv1.0 allows the use of the `esb` tooling for solving Advent of Code problems.
*  This is an implementation of FIREPLACEv1.0 for Rust.
*
*  Check [esb](https://github.com/luxedo/esb) for more information.
*/
use std::error::Error;
use std::fmt::Display;
use std::io;
use std::str::FromStr;

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

pub trait InputReader {
    fn load_fireplace_input(&mut self) -> FireplaceResult<String>;
}

impl InputReader for io::Stdin {
    fn load_fireplace_input(&mut self) -> FireplaceResult<String> {
        let stdin = io::read_to_string(self)?;
        Ok(stdin)
    }
}

pub struct FireplaceArgs<'a> {
    part: AoCPart,
    args: Vec<&'a str>,
}

impl TryFrom<clap::ArgMatches> for FireplaceArgs<'_> {
    type Error = FireplaceError;

    fn try_from(matches: clap::ArgMatches) -> Result<Self, Self::Error> {
        let Some(part) = matches.get_one::<String>("part") else {
            return Err(FireplaceError::MissingPart);
        };
        let part = part.parse::<AoCPart>()?;
        let args: Vec<&str> = matches
            .get_many("args")
            .map_or(vec![], |v| v.cloned().collect::<Vec<&str>>());
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
                .num_args(0..)
                .help("Additional arguments for running the solutions")
        )
}

enum Either<T, U> {
    Left(T),
    Right(U),
}

impl<T: Display, U: Display> Display for Either<T, U> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Either::Left(t) => write!(f, "{}", t),
            Either::Right(u) => write!(f, "{}", u),
        }
    }
}

pub fn v1_run<T, E1, U, E2>(
    solve_pt1: impl Fn(&str, Vec<&str>) -> Result<T, E1>,
    solve_pt2: impl Fn(&str, Vec<&str>) -> Result<U, E2>,
) -> FireplaceResult<()>
where
    T: Display + 'static,
    U: Display + 'static,
    E1: Error,
    E2: Error,
{
    let parser_matches = parser().get_matches();
    let fp_args = FireplaceArgs::try_from(parser_matches)?;
    run(&solve_pt1, &solve_pt2, io::stdin(), fp_args)?;
    Ok(())
}

fn run<T, E1, U, E2>(
    solve_pt1: impl Fn(&str, Vec<&str>) -> Result<T, E1>,
    solve_pt2: impl Fn(&str, Vec<&str>) -> Result<U, E2>,
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
    let answer = match fp_args.part {
        AoCPart::Pt1 => solve_pt1(&input_data, fp_args.args)
            .map_err(|e| FireplaceError::FromUser(e.to_string()))
            .map(Either::Left),
        AoCPart::Pt2 => solve_pt2(&input_data, fp_args.args)
            .map_err(|e| FireplaceError::FromUser(e.to_string()))
            .map(Either::Right),
    };

    match answer {
        Ok(answer) => {
            println!("{}", answer);
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

    fn solve_pt1(input_data: &str, args: Vec<&str>) -> FireplaceResult<String> {
        Ok(match args.len() {
            0 => input_data.trim().into(),
            _ => args.join(" "),
        })
    }

    const PT2_RETURN: &str = "2";
    fn solve_pt2(_input_data: &str, _args: Vec<&str>) -> FireplaceResult<String> {
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
            args: vec!["a", "b", "c"],
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
            |_: &str, _: Vec<&str>| -> Result<String, std::fmt::Error> { Err(std::fmt::Error) };
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
