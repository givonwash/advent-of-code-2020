use std::{
    io::{self, Read},
    num::ParseIntError,
    ops::{Deref, DerefMut},
    str::FromStr,
};

#[derive(Clone, Copy)]
enum Instruction {
    Acc(i32),
    Jmp(isize),
    Nop(isize),
}

#[derive(Debug)]
enum ParseInstructionError {
    IncompleteInstruction,
    InvalidOperation,
    InvalidArgument,
}

struct Tape(Vec<Instruction>);

struct Executor<'a> {
    accumulator: i32,
    /// A vector where element `i` holds an `Option` indicating whether `tape[i]` has been executed
    executed: Vec<Option<Instruction>>,
    head: usize,
    tape: &'a Tape,
}

impl From<ParseIntError> for ParseInstructionError {
    fn from(_: ParseIntError) -> Self {
        Self::InvalidArgument
    }
}

impl FromStr for Instruction {
    type Err = ParseInstructionError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.split_once(' ') {
            Some(("acc", arg)) => Ok(Self::Acc(arg.parse()?)),
            Some(("jmp", arg)) => Ok(Self::Jmp(arg.parse()?)),
            Some(("nop", arg)) => Ok(Self::Nop(arg.parse()?)),
            Some((_, _)) => Err(Self::Err::InvalidOperation),
            None => Err(Self::Err::IncompleteInstruction),
        }
    }
}

impl Instruction {
    fn invert(&mut self) {
        match self {
            Self::Jmp(arg) => {
                *self = Self::Nop(*arg);
            }
            Self::Nop(arg) => {
                *self = Self::Jmp(*arg);
            }
            Self::Acc(_) => {}
        }
    }
}

impl Deref for Tape {
    type Target = Vec<Instruction>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Tape {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl FromIterator<Instruction> for Tape {
    fn from_iter<T: IntoIterator<Item = Instruction>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl Tape {
    fn evaluate(&self) -> Executor<'_> {
        let mut executor = Executor::from(self);
        executor.by_ref().last();
        executor
    }
}

impl<'a> From<&'a Tape> for Executor<'a> {
    fn from(tape: &'a Tape) -> Self {
        Self {
            accumulator: 0,
            head: 0,
            tape,
            executed: tape.iter().map(|_| None).collect(),
        }
    }
}

impl<'a> Iterator for Executor<'a> {
    type Item = (usize, i32);

    fn next(&mut self) -> Option<Self::Item> {
        use Instruction::*;

        self.executed
            .get_mut(self.head)
            .zip(self.tape.get(self.head))
            .and_then(|(executed, instruction)| match executed {
                Some(_) => None,
                None => {
                    match instruction {
                        Acc(arg) => {
                            self.accumulator += arg;
                            self.head += 1;
                        }
                        Jmp(arg) => {
                            if arg.is_negative() {
                                self.head -= arg.wrapping_abs() as usize;
                            } else {
                                self.head += *arg as usize;
                            }
                        }
                        Nop(_) => {
                            self.head += 1;
                        }
                    };

                    *executed = Some(*instruction);

                    Some((self.head, self.accumulator))
                }
            })
    }
}

impl<'a> Executor<'a> {
    fn is_looping(&self) -> bool {
        matches!(self.executed.get(self.head), Some(Some(_)))
    }
}

fn part_one<I: Iterator<Item = Instruction>>(instructions: I) {
    let tape = instructions.collect::<Tape>();
    let executor = Executor::from(&tape);
    let answer = executor.last().map(|(_, acc)| acc);

    println!("Part One: {answer:?}");
}

fn part_two<I: Iterator<Item = Instruction>>(instructions: I) {
    use Instruction::*;

    let mut tape = instructions.collect::<Tape>();
    let answer = tape
        .evaluate()
        .executed
        .into_iter()
        .enumerate()
        .find_map(|(ptr, instruction)| match instruction {
            Some(Jmp(_) | Nop(_)) => {
                tape.get_mut(ptr).unwrap().invert();
                let executor = tape.evaluate();

                if executor.is_looping() {
                    tape.get_mut(ptr).unwrap().invert();
                    None
                } else {
                    Some(executor.accumulator)
                }
            }
            _ => None,
        });

    println!("Part Two: {answer:?}");
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let instructions = input
        .lines()
        .map(|instruction| instruction.parse().unwrap());

    part_one(instructions.clone());
    part_two(instructions);

    Ok(())
}
