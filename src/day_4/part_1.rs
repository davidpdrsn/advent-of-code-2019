use super::*;
use rayon::prelude::*;

pub fn main<R>(input: R) -> Result<()>
where
    R: RangeBounds<u32> + IntoParallelIterator<Item = u32> + Clone,
{
    let count_matching = input.into_par_iter().filter_map(validate).count();

    println!("{}", count_matching);

    Ok(())
}

fn validate(n: u32) -> Option<u32> {
    validate_length(n, 6)
        .and_then(validate_two_identical_adjacent_digits)
        .and_then(validate_each_digit_increases)
}
