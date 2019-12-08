#![allow(
    unused_parens,
    unstable_name_collisions,
    unused_imports,
    unused_variables,
    dead_code,
    clippy::unreadable_literal
)]

use extend::ext;
use std::convert::TryFrom;
use structopt::StructOpt;

pub use anyhow::{Error, Result};

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

macro_rules! define_parts {
    ( $(($mod:ident, $n:expr),)* ) => {
        define_parts!( $( ($mod, $n) ),* )
    };

    ( $(($mod:ident, $n:expr)),* ) => {
        $( mod $mod; )*

        fn try_main() -> Result<()> {
            let opt = Opt::from_args();

            match (opt.day, opt.part) {
                $(
                    ($n, None) => $mod::main(Part::One),
                    ($n, Some(part)) => $mod::main(Part::try_from(part)?),
                )*

                (day, None) => Err(Error::msg(format!("Unknown day {}", day))),

                (day, Some(part)) => Err(Error::msg(format!("Unknown day {}, part {}", day, part))),
            }
        }
    };
}

define_parts!(
    (day_1, 1),
    (day_2, 2),
    (day_3, 3),
    (day_4, 4),
    (day_5, 5),
    (day_6, 6)
);

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

#[ext(name = BoolThenExt)]
impl bool {
    #[inline]
    fn then<T>(self, t: T) -> Option<T> {
        if self {
            Some(t)
        } else {
            None
        }
    }
}
