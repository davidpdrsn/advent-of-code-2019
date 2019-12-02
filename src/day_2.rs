use super::{read_file, Error, Result};
use rayon::prelude::*;
use std::fmt;

pub fn main() -> Result<()> {
    let input = read_file("input/day_2")?;
    let mem = parse_input_to_mem(&input)?;

    let result = (0..=99)
        .into_par_iter()
        .flat_map(|noun| (0..=99).into_par_iter().map(move |verb| (noun, verb)))
        .find_first(|(noun, verb)| {
            let mut machine = IntMachine::new(&mem);

            machine.mem[1] = *noun;
            machine.mem[2] = *verb;

            machine.run_to_completion().ok().unwrap();

            let output = machine.mem[0];

            19_690_720 == output
        });

    let (noun, verb) = result.ok_or_else(|| Error::boxed("no match"))?;
    println!("{}", 100 * noun + verb);

    Ok(())
}

fn parse_input_to_mem(input: &str) -> Result<Vec<i32>> {
    input
        .split(',')
        .map(|i| i.replace('\n', "").parse::<i32>())
        .collect::<Result<Vec<_>, _>>()
        .map_err(From::from)
}

#[derive(Debug)]
struct IntMachine {
    mem: Vec<i32>,
    ip: usize,
}

impl IntMachine {
    fn new(mem: &[i32]) -> Self {
        Self {
            mem: mem.to_vec(),
            ip: 0,
        }
    }

    fn run_to_completion(&mut self) -> Result<()> {
        loop {
            match self.tick()? {
                TickOutput::Done => break,
                TickOutput::NotDone => {}
            }
        }

        Ok(())
    }

    fn tick(&mut self) -> Result<TickOutput> {
        match self.next_ops()? {
            OpCode::Add => self.run_arithmetic_op(|a, b| a + b),
            OpCode::Mul => self.run_arithmetic_op(|a, b| a * b),
            OpCode::Halt => Ok(TickOutput::Done),
        }
    }

    fn run_arithmetic_op(&mut self, f: impl Fn(i32, i32) -> i32) -> Result<TickOutput> {
        let a_pos = self.mem[self.ip + 1];
        let a = self.mem[a_pos as usize];

        let b_pos = self.mem[self.ip + 2];
        let b = self.mem[b_pos as usize];

        let destination = self.mem[self.ip + 3] as usize;
        self.mem[destination] = f(a, b);

        self.ip += 4;
        Ok(TickOutput::NotDone)
    }

    fn next_ops(&self) -> Result<OpCode> {
        match self.mem[self.ip] {
            99 => Ok(OpCode::Halt),
            1 => Ok(OpCode::Add),
            2 => Ok(OpCode::Mul),
            code => Err(Error::boxed(format!("Invalid op code {}", code))),
        }
    }
}

enum OpCode {
    Add,
    Mul,
    Halt,
}

enum TickOutput {
    Done,
    NotDone,
}

impl fmt::Display for IntMachine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mem = self
            .mem
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join(",");
        write!(f, "{}", mem)
    }
}

#[test]
fn test() {
    let ops = "1,9,10,3,2,3,11,0,99,30,40,50";
    let mem = parse_input_to_mem(ops).unwrap();
    let mut machine = IntMachine::new(&mem);
    machine.run_to_completion().unwrap();

    assert_eq!(machine.to_string(), "3500,9,10,70,2,3,11,0,99,30,40,50");
}
