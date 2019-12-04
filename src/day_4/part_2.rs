use super::*;
use rayon::prelude::*;

pub fn main<R>(input: R) -> Result<()>
where
    R: RangeBounds<u64> + IntoParallelIterator<Item = u64>,
{
    let count_matching = input.into_par_iter().filter_map(validate).count();

    println!("{}", count_matching);

    Ok(())
}

fn validate(n: u64) -> Option<u64> {
    validate_length(n, 6)
        .and_then(validate_two_identical_adjacent_digits_small_group)
        .and_then(validate_each_digit_increases)
}
