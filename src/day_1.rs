use super::{read_file, Result};

pub fn main() -> Result<()> {
    let mut total = 0;

    for line in read_file("input/day_1")?.lines() {
        let mass = line.parse::<i64>()?;
        total += fuel(mass);
    }

    dbg!(total);

    Ok(())
}

fn fuel(mass: i64) -> i64 {
    let n = (mass / 3) - 2;
    if n < 0 {
        0
    } else {
        n + fuel(n)
    }
}
