use structopt::StructOpt;

type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

mod day_1;

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
        other => {
            eprintln!("Unknown day {}", other);
            std::process::exit(1)
        },
    };

    match out {
        Ok(()) => {},
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1)
        },
    }
}

fn read_file(path: &str) -> Result<String> {
    std::fs::read_to_string(path).map_err(From::from)
}
