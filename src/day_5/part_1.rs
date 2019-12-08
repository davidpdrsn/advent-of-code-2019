use super::*;

pub fn main(input: String) -> Result<()> {
    let mem = parse_input_to_mem(&input)?;
    // let mem = parse_input_to_mem("3,0,4,0,99")?;
    let machine = IntMachine::new(mem);

    machine.run_to_completion()?;

    Ok(())
}
