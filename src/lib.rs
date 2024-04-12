/*
*  The FIREPLACEv1.0 allows the use of the `esb` tooling for solving Advent of Code problems.
*  This is an implementation of FIREPLACEv1.0 for Rust.
*
*  Check [esb](https://github.com/luxedo/esb) for more information.
*/
#![feature(trait_alias)]
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
    FromString(String),
}

type FireplaceResult<T> = Result<T, FireplaceError>;
pub trait AoCFunction<T: Display> = Fn(&str, Option<Vec<String>>) -> FireplaceResult<T>;

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

pub struct FireplaceArgs {
    part: AoCPart,
    args: Option<Vec<String>>,
}

impl TryFrom<clap::ArgMatches> for FireplaceArgs {
    type Error = FireplaceError;

    fn try_from(matches: clap::ArgMatches) -> Result<Self, Self::Error> {
        let Some(part) = matches.get_one::<String>("part") else {
            return Err(FireplaceError::MissingPart);
        };
        let part = part.parse::<AoCPart>()?;
        let args: Option<Vec<String>> = matches
            .try_get_many::<String>("args")
            .map_or(None, |opt_args| opt_args.map(|v| v.cloned().collect()));

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

pub fn run_with_clap<T, U>(
    solve_pt1: impl AoCFunction<T>,
    solve_pt2: impl AoCFunction<U>,
) -> FireplaceResult<()>
where
    T: Display + 'static,
    U: Display + 'static,
{
    let parser_matches = parser().get_matches();
    let fp_args = FireplaceArgs::try_from(parser_matches)?;
    run(&solve_pt1, &solve_pt2, io::stdin(), fp_args)?;
    Ok(())
}

pub fn run<T, U>(
    solve_pt1: impl AoCFunction<T>,
    solve_pt2: impl AoCFunction<U>,
    mut input_reader: impl InputReader,
    fp_args: FireplaceArgs,
) -> FireplaceResult<Box<dyn Display>>
where
    T: Display + 'static,
    U: Display + 'static,
{
    let input_data = input_reader.load_fireplace_input()?;
    let answer: Box<dyn Display> = match fp_args.part {
        AoCPart::Pt1 => Box::new(solve_pt1(&input_data, fp_args.args)?),
        AoCPart::Pt2 => Box::new(solve_pt2(&input_data, fp_args.args)?),
    };
    println!("{}", answer);
    Ok(answer)
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

    fn solve_pt1(input_data: &str, args: Option<Vec<String>>) -> FireplaceResult<String> {
        Ok(if let Some(a) = args {
            a.join(" ")
        } else {
            input_data.trim().into()
        })
    }

    const PT2_RETURN: &str = "2";
    fn solve_pt2(_input_data: &str, _args: Option<Vec<String>>) -> FireplaceResult<String> {
        Ok(PT2_RETURN.into())
    }

    fn test_runner(fp_args: FireplaceArgs) -> FireplaceResult<Box<dyn Display>> {
        super::run(solve_pt1, solve_pt2, TestInputReader, fp_args)
    }

    #[test]
    fn test_calls_solve_pt1() {
        let fp_args = FireplaceArgs {
            part: AoCPart::Pt1,
            args: None,
        };
        let answer = test_runner(fp_args).unwrap();
        let expected = "sample input";
        assert_eq!(answer.to_string(), expected);
    }

    #[test]
    fn test_calls_solve_pt1_with_args() {
        let fp_args = FireplaceArgs {
            part: AoCPart::Pt1,
            args: Some(vec!["a".into(), "b".into(), "c".into()]),
        };
        let answer = test_runner(fp_args).unwrap();
        let expected = "a b c";
        assert_eq!(answer.to_string(), expected);
    }

    #[test]
    fn test_calls_solve_pt2() {
        let fp_args = FireplaceArgs {
            part: AoCPart::Pt2,
            args: None,
        };
        let answer = test_runner(fp_args).unwrap();
        assert_eq!(answer.to_string(), PT2_RETURN);
    }
}
