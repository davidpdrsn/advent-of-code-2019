use std::fmt;
use structopt::StructOpt;

type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

mod day_1;
mod day_2;
mod day_3;

#[derive(Debug, StructOpt)]
#[structopt(name = "aoc")]
struct Opt {
    #[structopt(name = "DAY")]
    day: usize,
}

fn main() {
    let opt = Opt::from_args();

    let out = match opt.day {
        1 => day_1::main(),
        2 => day_2::main(),
        3 => day_3::main(),
        other => {
            eprintln!("Unknown day {}", other);
            std::process::exit(1)
        }
    };

    match out {
        Ok(()) => {}
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1)
        }
    }
}

fn read_file(path: &str) -> Result<String> {
    std::fs::read_to_string(path).map_err(From::from)
}

#[derive(Debug)]
struct Error {
    msg: String,
}

impl Error {
    fn boxed<T: Into<String>>(msg: T) -> Box<Self> {
        Box::new(Error { msg: msg.into() })
    }
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}
