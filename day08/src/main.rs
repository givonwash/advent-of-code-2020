use std::{
    collections::HashSet,
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

#[derive(Clone, Copy)]
enum ExecutingKind {
    Looping,
    NonLooping,
}

#[derive(Clone, Copy)]
enum ExecutorState {
    Executing(ExecutingKind),
    Halted,
}

struct Executor<'a> {
    accumulator: i32,
    executed: HashSet<usize>,
    head: usize,
    state: ExecutorState,
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
    fn init_exec(&self) -> Executor<'_> {
        self.into()
    }
}

impl<'a> From<&'a Tape> for Executor<'a> {
    fn from(tape: &'a Tape) -> Self {
        Self {
            tape,
            accumulator: 0,
            executed: HashSet::new(),
            head: 0,
            state: ExecutorState::Executing(ExecutingKind::NonLooping),
        }
    }
}

impl<'a> Iterator for Executor<'a> {
    type Item = (usize, i32, Option<Instruction>, ExecutorState);

    fn next(&mut self) -> Option<Self::Item> {
        match self.state {
            ExecutorState::Executing(ExecutingKind::NonLooping) => {
                if let Some(instr) = self.tape.get(self.head) {
                    let curr = (self.head, self.accumulator, Some(*instr), self.state);
                    self.execute(*instr);

                    if !self.executed.insert(self.head) {
                        self.state = ExecutorState::Executing(ExecutingKind::Looping);
                    }

                    Some(curr)
                } else {
                    self.state = ExecutorState::Halted;
                    Some((self.head, self.accumulator, None, self.state))
                }
            }
            ExecutorState::Executing(ExecutingKind::Looping) => {
                let instr = self.tape.get(self.head).unwrap();
                let curr = (self.head, self.accumulator, Some(*instr), self.state);
                self.execute(*instr);
                Some(curr)
            }
            ExecutorState::Halted => Some((self.head, self.accumulator, None, self.state)),
        }
    }
}

impl<'a> Executor<'a> {
    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Acc(arg) => {
                self.accumulator += arg;
                self.head += 1;
            }
            Instruction::Jmp(arg) => {
                if arg.is_negative() {
                    self.head -= arg.wrapping_abs() as usize;
                } else {
                    self.head += arg as usize;
                }
            }
            Instruction::Nop(_) => {
                self.head += 1;
            }
        }
    }

    fn try_finish(&mut self) -> (usize, i32, Option<Instruction>, ExecutorState) {
        self.by_ref()
            .find(|(.., state)| {
                !matches!(state, ExecutorState::Executing(ExecutingKind::NonLooping))
            })
            .unwrap()
    }
}

fn part_one<I: Iterator<Item = Instruction>>(instructions: I) {
    let tape = instructions.collect::<Tape>();
    let mut exec = tape.init_exec();

    let (_, acc, ..) = exec.try_finish();

    println!("Part One: {}", acc);
}

fn part_two<I: Iterator<Item = Instruction>>(instructions: I) {
    let mut tape = instructions.collect::<Tape>();

    let mut base_executor = tape.init_exec();
    base_executor.try_finish();

    for instr_num in base_executor.executed {
        match tape.get_mut(instr_num).unwrap() {
            Instruction::Acc(_) => {
                continue;
            }
            instr => instr.invert(),
        }

        let mut modified_executor = tape.init_exec();
        let (_, acc, _, state) = modified_executor.try_finish();

        match state {
            ExecutorState::Executing(_) => {
                tape.get_mut(instr_num).unwrap().invert();
            }
            ExecutorState::Halted => {
                println!("Part Two: {}", acc);
                return;
            }
        }
    }
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let instructions = input.lines().map(|instr| instr.parse().unwrap());

    part_one(instructions.clone());
    part_two(instructions);

    Ok(())
}
