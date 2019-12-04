use super::*;
use rayon::prelude::*;

pub fn main() -> Result<()> {
    let input = read_file("input/day_3")?;

    let mut current_wire_positions = HashSet::<Pos>::new();
    let mut previous_wire_positions = HashSet::<Pos>::new();
    let mut intersections = HashSet::<Pos>::new();

    let wire_paths = input
        .lines()
        .map(|line| {
            for position_from_previous_wire_path in current_wire_positions.drain() {
                previous_wire_positions.insert(position_from_previous_wire_path);
            }

            let moves = parse_moves(line)?;

            let mut wire_path = WirePath::new();

            for move_ in moves {
                for single_move in move_.into_single_moves() {
                    wire_path.apply(single_move);
                    let current_position = wire_path.current_position();

                    current_wire_positions.insert(current_position);

                    if previous_wire_positions.contains(&current_position) {
                        intersections.insert(current_position);
                    }
                }
            }

            Ok(wire_path)
        })
        .collect::<Result<Vec<_>>>()?;

    let min = intersections
        .par_iter()
        .map(|intersection| {
            let sum = wire_paths
                .par_iter()
                .map(move |wire_path| distance(*intersection, &wire_path.all_positions()))
                .collect::<Result<Vec<usize>>>()?
                .into_par_iter()
                .sum::<usize>();

            Ok(sum)
        })
        .collect::<Result<Vec<_>>>()?
        .into_par_iter()
        .min()
        .ok_or_else(|| Error::msg("no min"))?;

    println!("{}", min);

    Ok(())
}

fn distance(intersection: Pos, paths: &[Pos]) -> Result<usize> {
    paths
        .par_iter()
        .enumerate()
        .filter_map(|(step_count, pos)| {
            if pos == &intersection {
                Some(step_count)
            } else {
                None
            }
        })
        .min()
        .ok_or_else(|| Error::msg("no min"))
        .map_err(From::from)
}
