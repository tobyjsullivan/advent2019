use std::env;
use std::io;
use std::io::Write;
use std::fs;

type Base = i64;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Must supply intcode file.");
    }

    let filename = &args[1];
    let input = fs::read_to_string(filename).expect("Failed to read file.");
    let mem = parse_mem_file(&input);

    let mut cmp = Computer {
        read_fn: Box::new(read_user_input),
        mem: mem,
        pc: 0,
    };

    cmp.compute();
}

fn read_user_input() -> Base {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input.trim().parse().expect("Failed to parse input.")
}

fn parse_mem_file(input: &str) -> Vec<Base> {
    // Split the input on commas
    input
        .split(",")
        .map(|s| s.trim())
        .map(|s| s.parse().expect("Not a number."))
        .collect()
}

struct Computer {
    read_fn: Box<Fn() -> Base>,
    mem: Vec<Base>,
    pc: usize,
}

impl Computer {
    fn compute(&mut self) {
        let pc = self.pc;
        let cmd = parse_instruction(self.mem[pc]).expect("Invalid OpCode.");

        match cmd {
            OpCode::Add { p_modes } => {
                op_add(&mut self.mem, p_modes, pc);
                self.pc += 4;
                self.compute();
            }
            OpCode::Multiply { p_modes } => {
                op_mult(&mut self.mem, p_modes, pc);
                self.pc += 4;
                self.compute();
            }
            OpCode::Input => {
                self.op_input(pc);
                self.pc += 2;
                self.compute();
            }
            OpCode::Output { p_modes } => {
                op_output(&mut self.mem, p_modes, pc);
                self.pc += 2;
                self.compute();
            }
            OpCode::JmpT { p_modes } => {
                self.op_jump_t(p_modes);
                self.compute();
            }
            OpCode::JmpF { p_modes } => {
                self.op_jump_f(p_modes);
                self.compute();
            }
            OpCode::Less { p_modes } => {
                self.op_less(p_modes);
                self.compute();
            }
            OpCode::Eq { p_modes } => {
                self.op_eq(p_modes);
                self.compute();

            }
            OpCode::Halt => {},
        }
    }

    fn op_input(&mut self, pc: usize) {
        let out = self.mem[pc + 1] as usize;
        self.mem[out] = (*self.read_fn)();
    }

    fn op_jump_t(&mut self, p_modes: [PMode; 2]) {
        let cmp = get(&self.mem, p_modes[0], self.pc + 1);
        let nxt = get(&self.mem, p_modes[1], self.pc + 2) as usize;
        if cmp != 0 {
            self.pc = nxt;
        } else {
            self.pc += 3;
        }
    }

    fn op_jump_f(&mut self, p_modes: [PMode; 2]) {
        let cmp = get(&self.mem, p_modes[0], self.pc + 1);
        let nxt = get(&self.mem, p_modes[1], self.pc + 2) as usize;
        if cmp == 0 {
            self.pc = nxt;
        } else {
            self.pc += 3;
        }
    }

    fn op_less(&mut self, p_modes: [PMode; 2]) {
        let a = get(&self.mem, p_modes[0], self.pc + 1);
        let b = get(&self.mem, p_modes[1], self.pc + 2);
        let out = self.mem[self.pc + 3] as usize;

        if a < b {
            self.mem[out] = 1;
        } else {
            self.mem[out] = 0;
        }
        self.pc += 4;
    }

    fn op_eq(&mut self, p_modes: [PMode; 2]) {
        let a = get(&self.mem, p_modes[0], self.pc + 1);
        let b = get(&self.mem, p_modes[1], self.pc + 2);
        let out = self.mem[self.pc + 3] as usize;

        if a == b {
            self.mem[out] = 1;
        } else {
            self.mem[out] = 0;
        }
        self.pc += 4;
    }
}

fn get(input: &Vec<Base>, p_mode: PMode, idx: usize) -> Base {
    match p_mode {
        PMode::Position => {
            let pos = input[idx] as usize;
            input[pos]
        }
        PMode::Immediate => {
            input[idx]
        }
    }
}

fn op_add(input: &mut Vec<Base>, p_modes: [PMode; 2], pc: usize) {
    let a = get(input, p_modes[0], pc + 1);
    let b = get(input, p_modes[1], pc + 2);
    let out = input[pc + 3] as usize;
    input[out] = a + b;
}

fn op_mult(input: &mut Vec<Base>, p_modes: [PMode; 2], pc: usize) {
    let a = get(input, p_modes[0], pc + 1);
    let b = get(input, p_modes[1], pc + 2);
    let out = input[pc + 3] as usize;
    input[out] = a * b;
}

fn op_output(input: &Vec<Base>, p_modes: [PMode; 1], pc: usize) {
    let a = get(input, p_modes[0], pc + 1);
    println!("{}", a);
}

fn parse_instruction(input: Base) -> Result<OpCode, String> {
    let opcode = input % 100;
    let mut rem = input / 100;
    let mut p_modes = Vec::<PMode>::new();
    while rem > 0 {
        let p_mode = match rem % 10 {
            0 => Ok(PMode::Position),
            1 => Ok(PMode::Immediate),
            _ => Err("Unknown PMODE"),
        }?;
        p_modes.push(p_mode);
        rem /= 10;
    }

    match opcode {
        1 => Ok(OpCode::Add {
            p_modes: [
                *p_modes.get(0).unwrap_or(&PMode::Position),
                *p_modes.get(1).unwrap_or(&PMode::Position),
            ],
        }),
        2 => Ok(OpCode::Multiply{
            p_modes: [
                *p_modes.get(0).unwrap_or(&PMode::Position),
                *p_modes.get(1).unwrap_or(&PMode::Position),
            ],
        }),
        3 => Ok(OpCode::Input),
        4 => Ok(OpCode::Output {
            p_modes: [
                *p_modes.get(0).unwrap_or(&PMode::Position),
            ]
        }),
        5 => Ok(OpCode::JmpT {
            p_modes: [
                *p_modes.get(0).unwrap_or(&PMode::Position),
                *p_modes.get(1).unwrap_or(&PMode::Position),
            ],
        }),
        6 => Ok(OpCode::JmpF {
            p_modes: [
                *p_modes.get(0).unwrap_or(&PMode::Position),
                *p_modes.get(1).unwrap_or(&PMode::Position),
            ],
        }),
        7 => Ok(OpCode::Less {
            p_modes: [
                *p_modes.get(0).unwrap_or(&PMode::Position),
                *p_modes.get(1).unwrap_or(&PMode::Position),
            ],
        }),
        8 => Ok(OpCode::Eq {
            p_modes: [
                *p_modes.get(0).unwrap_or(&PMode::Position),
                *p_modes.get(1).unwrap_or(&PMode::Position),
            ],
        }),
        99 => Ok(OpCode::Halt),
        _ => Err("Unknown OPCODE".to_owned()),
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum OpCode {
    Add {
        p_modes: [PMode; 2],
    },
    Multiply {
        p_modes: [PMode; 2],
    },
    Input,
    Output {
        p_modes: [PMode; 1],
    },
    JmpT {
        p_modes: [PMode; 2],
    },
    JmpF {
        p_modes: [PMode; 2],
    },
    Less {
        p_modes: [PMode; 2],
    },
    Eq {
        p_modes: [PMode; 2],
    },
    Halt,
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum PMode {
    Position,
    Immediate,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_cmd() {
        let expected = Ok(OpCode::Add {
            p_modes: [
            PMode::Position,
            PMode::Position,
        ]});
        assert_eq!(parse_instruction(1), expected);
    }

    #[test]
    fn test_parse_cmd_modes() {
        let expected = Ok(OpCode::Multiply {
            p_modes: [
            PMode::Position,
            PMode::Immediate,
        ]});
        assert_eq!(parse_instruction(1002), expected);
    }

    #[test]
    fn test_add() {
        let mut input = vec![1, 5, 6, 0, 99, 2, 3];
        op_add(&mut input, [PMode::Position, PMode::Position], 0);

        assert_eq!(input, &[5, 5, 6, 0, 99, 2, 3]);
    }

    #[test]
    fn test_add_neg() {
        let mut input = vec![1, 5, 6, 0, 99, 2, -3];
        op_add(&mut input, [PMode::Position, PMode::Position], 0);

        assert_eq!(input, &[-1, 5, 6, 0, 99, 2, -3]);
    }

    #[test]
    fn test_add_imm() {
        let mut input = vec![1, 5, 6, 3, 99, 2, 3];
        op_add(&mut input, [PMode::Immediate, PMode::Position], 0);

        assert_eq!(input, &[1, 5, 6, 8, 99, 2, 3]);
    }

    #[test]
    fn test_mult() {
        let mut input = vec![1, 5, 6, 0, 99, 2, 3];
        op_mult(&mut input, [PMode::Position, PMode::Position], 0);

        assert_eq!(input, &[6, 5, 6, 0, 99, 2, 3]);
    }

    #[test]
    fn test_op_input() {
        let mut cmp = Computer{
            read_fn: Box::new(|| 42),
            mem: vec![3, 5, 6, 0, 99, 2, 3],
            pc: 0,
        };
        cmp.op_input(0);

        assert_eq!(cmp.mem, &[3, 5, 6, 0, 99, 42, 3]);
    }

    #[test]
    fn test_parse() {
        let res = parse_mem_file("1,0,0,0,99");
        assert_eq!(res, &[1, 0, 0, 0, 99]);
    }

    #[test]
    fn test_interpret_case1() {
        let mut cmp = Computer{
            read_fn: Box::new(|| 0),
            mem: vec![1, 0, 0, 0, 99],
            pc: 0,
        };
        cmp.compute();
        assert_eq!(cmp.mem, &[2, 0, 0, 0, 99]);
    }

    #[test]
    fn test_interpret_case2() {
        let mut cmp = Computer{
            read_fn: Box::new(|| 0),
            mem: vec![2, 3, 0, 3, 99],
            pc: 0,
        };
        cmp.compute();
        assert_eq!(cmp.mem, &[2, 3, 0, 6, 99]);
    }

    #[test]
    fn test_interpret_case3() {
        let mut cmp = Computer{
            read_fn: Box::new(|| 0),
            mem: vec![2, 4, 4, 5, 99, 0],
            pc: 0,
        };
        cmp.compute();
        assert_eq!(cmp.mem, &[2, 4, 4, 5, 99, 9801]);
    }

    #[test]
    fn test_interpret_case4() {
        let mut cmp = Computer{
            read_fn: Box::new(|| 0),
            mem: vec![1,1,1,4,99,5,6,0,99],
            pc: 0,
        };
        cmp.compute();
        assert_eq!(cmp.mem, &[30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }
}
