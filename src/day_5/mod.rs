use super::{read_file, Error, Part, Result};

mod part_1;
mod part_2;

pub fn main(part: Part) -> Result<()> {
    let input = read_file("input/day_5")?;

    match part {
        Part::One => part_1::main(input),
        Part::Two => part_2::main(input),
    }
}
