use super::{read_file, Error, Part, Result};
use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;

mod part_1;
mod part_2;

pub fn main(part: Part) -> Result<()> {
    let input = read_file("input/day_5")?;

    match part {
        Part::One => part_1::main(input),
        Part::Two => part_2::main(input),
    }
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
    pc: usize,
}

impl IntMachine {
    fn new(mem: Vec<i32>) -> Self {
        Self { mem, pc: 0 }
    }

    fn run_to_completion(mut self) -> Result<()> {
        loop {
            println!("raw instruction = {:?}", self.mem[self.pc]);
            let Instruction { op_code, modes } = parse_instruction(self.mem[self.pc])?;
            println!("op_code = {:?}", op_code);
            println!("modes = {:?}", modes);

            match op_code {
                OpCode::Add => self.run_arithmetic_op("add", modes, |a, b| a + b),

                OpCode::Mul => self.run_arithmetic_op("mul", modes, |a, b| a * b),

                OpCode::Input => {
                    let arg = self.get_arg(1, Mode::Position);
                    let input = get_int()?;
                    self.mem[arg as usize] = 1;
                }

                OpCode::Output => {
                    let arg = self.get_arg(1, modes[0]);
                    println!("{}", &self.mem[arg as usize]);
                }

                OpCode::Halt => break,
            }

            self.pc += op_code.len();
            println!("-----------------------");
        }

        Ok(())
    }

    fn run_arithmetic_op(&mut self, name: &str, modes: Vec<Mode>, f: impl Fn(i32, i32) -> i32) {
        let a = self.get_arg(1, modes[0]);
        let b = self.get_arg(2, modes[1]);
        let c = self.get_arg(3, Mode::Position);

        self.mem[c as usize] = f(a, b);
    }

    fn get_arg(&self, offset: usize, mode: Mode) -> i32 {
        let raw = self.mem[self.pc + offset];

        match mode {
            Mode::Position => self.mem[raw as usize],
            Mode::Immediate => raw,
        }
    }
}

fn get_int() -> Result<i32> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line)?;
    line.replace('\n', "").parse().map_err(From::from)
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum OpCode {
    Add,
    Mul,
    Input,
    Output,
    Halt,
}

impl OpCode {
    fn len(&self) -> usize {
        match self {
            OpCode::Add | OpCode::Mul => 4,
            OpCode::Input => 2,
            OpCode::Output => 2,
            OpCode::Halt => 1,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Mode {
    Position,
    Immediate,
}

#[derive(Debug, Eq, PartialEq)]
struct Instruction {
    op_code: OpCode,
    modes: Vec<Mode>,
}

fn digits(n: i32) -> Vec<i32> {
    n.to_string()
        .split("")
        .filter(|digit| digit != &"")
        .map(|digit| digit.parse::<i32>().unwrap())
        .collect()
}

fn left_most<T: Clone>(vec: &[T], n: usize) -> Vec<T> {
    if n > vec.len() {
        return vec.to_vec();
    }

    let v = vec.to_vec().reversed();
    let v = v[0..n].to_vec().reversed();
    v
}

fn parse_instruction(n: i32) -> Result<Instruction> {
    let mut digits = std::iter::repeat(0).take(4).collect::<Vec<_>>();
    digits.extend(self::digits(n));

    let raw_op_code = left_most(&digits, 2);
    let op_code = match (raw_op_code.get(0), raw_op_code.get(1)) {
        (Some(0), Some(1)) | (Some(1), None) => OpCode::Add,
        (Some(0), Some(2)) | (Some(2), None) => OpCode::Mul,
        (Some(0), Some(3)) | (Some(3), None) => OpCode::Input,
        (Some(0), Some(4)) | (Some(4), None) => OpCode::Output,
        (Some(9), Some(9)) => OpCode::Halt,
        other => return Err(Error::msg(format!("invalid op code {:?}", other))),
    };

    let raw_modes = digits[0..digits.len() - 2].to_vec();
    let raw_modes = raw_modes
        .reversed()
        .resized(op_code.len() - 1, 0)
        .reversed();

    let modes = raw_modes
        .iter()
        .map(|mode| match mode {
            0 => Ok(Mode::Position),
            1 => Ok(Mode::Immediate),
            other => Err(Error::msg(format!("invalid mode {}", other))),
        })
        .collect::<Result<Vec<_>>>()?;

    Ok(Instruction { op_code, modes })
}

#[extend::ext]
impl<T> Vec<T> {
    #[inline]
    fn reversed(mut self) -> Self {
        self.reverse();
        self
    }

    #[inline]
    fn resized(mut self, new_len: usize, value: T) -> Self
    where
        T: Clone,
    {
        self.resize(new_len, value);
        self
    }
}

#[test]
fn parse_opcode() {
    let ins = parse_instruction(1002).unwrap();

    assert_eq!(
        ins,
        Instruction {
            op_code: OpCode::Mul,
            modes: vec![Mode::Position, Mode::Immediate, Mode::Position],
        }
    )
}

#[test]
fn left_most_test() {
    assert_eq!(left_most(&vec![1, 2, 3, 4, 5], 2), vec![4, 5]);
    assert_eq!(left_most(&vec![1], 2), vec![1]);
}
