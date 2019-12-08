use super::{read_file, Error, Part, Result};
use anyhow::format_err;
use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;

mod part_1;

pub fn main(part: Part) -> Result<()> {
    let input = read_file("input/day_6")?;

    match part {
        Part::One => part_1::main(&input)?,
        Part::Two => println!("{}", part_2(&input)?),
    }

    Ok(())
}

fn part_2(input: &str) -> Result<u64> {
    let (map, orbiter_to_planet) = parse_input(&input);

    let start = orbiter_to_planet["YOU"];
    let end = orbiter_to_planet["SAN"];

    let path = shortest_path(&map, &mut HashSet::new(), start, end)
        .ok_or_else(|| format_err!("no path"))?;

    Ok(path)
}

fn shortest_path<'a>(
    map: &HashMap<&str, Vec<&'a str>>,
    visited: &mut HashSet<&'a str>,
    start: &str,
    end: &str,
) -> Option<u64> {
    if start == end {
        return Some(0);
    }

    map.get(start).and_then(|orbiters| {
        let orbiters = orbiters
            .iter()
            .filter(|orbiter| !visited.contains(*orbiter))
            .collect::<Vec<_>>();

        for orbiter in &orbiters {
            visited.insert(orbiter);
        }

        orbiters
            .iter()
            .filter_map(|orbiter| shortest_path(map, visited, *orbiter, end).map(|len| len + 1))
            .min()
    })
}

fn parse_input(input: &str) -> (HashMap<&str, Vec<&str>>, HashMap<&str, &str>) {
    let mut map = HashMap::new();
    let mut orbiter_to_planet = HashMap::new();

    for line in input.lines() {
        let parts = parse_line(line);

        map.entry(parts.0).or_insert(vec![]).push(parts.1);

        map.entry(parts.1).or_insert(vec![]).push(parts.0);

        orbiter_to_planet.insert(parts.1, parts.0);
    }

    (map, orbiter_to_planet)
}

fn parse_line(line: &str) -> (&str, &str) {
    let parts = line.split(")").collect::<Vec<_>>();
    (&parts[0], &parts[1])
}

#[test]
fn part_2_test() {
    let input = vec![
        "COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L", "K)YOU",
        "I)SAN",
    ]
    .join("\n");
    assert_eq!(4, part_2(&input).unwrap());
}
