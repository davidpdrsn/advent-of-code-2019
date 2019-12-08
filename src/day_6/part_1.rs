use super::{read_file, Error, Part, Result};
use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;

pub fn main(input: &str) -> Result<()> {
    let map = parse_input(&input);
    let mut count = 0;
    count_orbits(&map, "COM", &mut count, 0);
    println!("{}", count);

    Ok(())
}

fn count_orbits(map: &HashMap<&str, Vec<&str>>, planet: &str, count: &mut u64, depth: u64) {
    if let Some(orbiters) = map.get(planet) {
        for orbiter in orbiters {
            *count += 1;
            *count += depth;
            count_orbits(map, orbiter, count, depth + 1);
        }
    }
}

fn parse_input(input: &str) -> HashMap<&str, Vec<&str>> {
    let mut map = HashMap::new();

    for line in input.lines() {
        let parts = parse_line(line);
        map.entry(parts.0).or_insert(vec![]).push(parts.1);
    }

    map
}

fn parse_line(line: &str) -> (&str, &str) {
    let parts = line.split(")").collect::<Vec<_>>();
    (&parts[0], &parts[1])
}
