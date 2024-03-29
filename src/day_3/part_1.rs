use super::*;
use rayon::prelude::*;

pub fn main() -> Result<()> {
    let input = read_file("input/day_3")?;

    let mut current_wire_positions = HashSet::<Pos>::new();
    let mut previous_wire_positions = HashSet::<Pos>::new();
    let mut intersections = HashSet::<Pos>::new();

    for line in input.lines() {
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
    }

    let min = intersections
        .into_par_iter()
        .min_by_key(|pos| manhattan_distance(Pos::zero(), *pos))
        .ok_or_else(|| Error::msg("no min"))?;

    println!("{}", manhattan_distance(Pos::zero(), min));

    Ok(())
}
