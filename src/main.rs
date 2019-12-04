use std::convert::TryFrom;
use structopt::StructOpt;

pub use anyhow::{Error, Result};

mod day_1;
mod day_2;
mod day_3;

#[derive(Debug, StructOpt)]
#[structopt(name = "aoc")]
struct Opt {
    #[structopt(name = "DAY")]
    day: usize,

    #[structopt(name = "PART")]
    part: Option<usize>,
}

fn main() {
    match try_main() {
        Ok(()) => {}
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1)
        }
    }
}

fn try_main() -> Result<()> {
    let opt = Opt::from_args();

    match (opt.day, opt.part) {
        (1, None) => day_1::main(),
        (1, Some(_)) => unimplemented!(),

        (2, None) => day_2::main(),
        (2, Some(_)) => unimplemented!(),

        (3, None) => day_3::main(Part::One),
        (3, Some(part)) => day_3::main(Part::try_from(part)?),

        (day, None) => Err(Error::msg(format!("Unknown day {}", day))),

        (day, Some(part)) => Err(Error::msg(format!("Unknown day {}, part {}", day, part))),
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Part {
    One,
    Two,
}

impl TryFrom<usize> for Part {
    type Error = anyhow::Error;

    fn try_from(other: usize) -> Result<Self> {
        match other {
            1 => Ok(Part::One),
            2 => Ok(Part::Two),
            other => Err(Error::msg(format!(
                "Invalid part in cmdline arg: {}",
                other
            ))),
        }
    }
}

fn read_file(path: &str) -> Result<String> {
    std::fs::read_to_string(path).map_err(From::from)
}
