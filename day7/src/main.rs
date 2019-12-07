use std::env;
use std::fs;
use std::io;
use std::sync::mpsc::{channel, Receiver, RecvError, SendError, Sender};
use std::thread;

type Base = i64;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 7 {
        panic!("Must supply intcode file and phase values.");
    }

    let filename = &args[1];
    let input = fs::read_to_string(filename).expect("Failed to read file.");

    let (tx_a, rx_b) = channel();
    let (tx_b, rx_c) = channel();
    let (tx_c, rx_d) = channel();
    let (tx_d, rx_e) = channel();
    let (tx_e, rx_a) = channel();

    // Initialization values
    let phase_a = args[2].parse::<Base>().unwrap();
    let phase_b = args[3].parse::<Base>().unwrap();
    let phase_c = args[4].parse::<Base>().unwrap();
    let phase_d = args[5].parse::<Base>().unwrap();
    let phase_e = args[6].parse::<Base>().unwrap();
    tx_a.send(phase_a).unwrap();
    tx_b.send(phase_b).unwrap();
    tx_c.send(phase_c).unwrap();
    tx_d.send(phase_d).unwrap();
    tx_e.send(phase_e).unwrap();

    tx_a.send(0).unwrap();

    let mut cmp_a = Computer {
        in_buf: rx_a,
        out_buf: tx_a,
        mem: parse_mem_file(&input),
        pc: 0,
        halted: false,
        name: "A",
    };

    let t_a = thread::spawn(move || {
        while !cmp_a.halted {
            cmp_a.step();
        }
    });

    let mut cmp_b = Computer {
        in_buf: rx_b,
        out_buf: tx_b,
        mem: parse_mem_file(&input),
        pc: 0,
        halted: false,
        name: "B",
    };

    let t_b = thread::spawn(move || {
        while !cmp_b.halted {
            cmp_b.step();
        }

        let end_val = cmp_b.in_buf.recv().expect("Failed to receive final output");
        println!("{}", end_val);
    });

    let mut cmp_c = Computer {
        in_buf: rx_c,
        out_buf: tx_c,
        mem: parse_mem_file(&input),
        pc: 0,
        halted: false,
        name: "C",
    };

    let t_c = thread::spawn(move || {
        while !cmp_c.halted {
            cmp_c.step();
        }
    });

    let mut cmp_d = Computer {
        in_buf: rx_d,
        out_buf: tx_d,
        mem: parse_mem_file(&input),
        pc: 0,
        halted: false,
        name: "D",
    };

    let t_d = thread::spawn(move || {
        while !cmp_d.halted {
            cmp_d.step();
        }
    });

    let mut cmp_e = Computer {
        in_buf: rx_e,
        out_buf: tx_e,
        mem: parse_mem_file(&input),
        pc: 0,
        halted: false,
        name: "E",
    };

    let t_e = thread::spawn(move || {
        while !cmp_e.halted {
            cmp_e.step();
        }
    });

    t_a.join().unwrap();
    t_b.join().unwrap();
    t_c.join().unwrap();
    t_d.join().unwrap();
    t_e.join().unwrap();
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
    in_buf: Receiver<Base>,
    out_buf: Sender<Base>,
    mem: Vec<Base>,
    pc: usize,
    halted: bool,
    name: &'static str,
}

impl Computer {
    fn step(&mut self) {
        if self.halted {
            panic!("Attempted to step halted computer.");
        }

        let pc = self.pc;
        let cmd = parse_instruction(self.mem[pc]).expect("Invalid OpCode.");

        match cmd {
            OpCode::Add { p_modes } => {
                op_add(&mut self.mem, p_modes, pc);
                self.pc += 4;
            }
            OpCode::Multiply { p_modes } => {
                op_mult(&mut self.mem, p_modes, pc);
                self.pc += 4;
            }
            OpCode::Input => {
                self.op_input(pc);
                self.pc += 2;
            }
            OpCode::Output { p_modes } => {
                self.op_output(p_modes, pc);
                self.pc += 2;
            }
            OpCode::JmpT { p_modes } => {
                self.op_jump_t(p_modes);
            }
            OpCode::JmpF { p_modes } => {
                self.op_jump_f(p_modes);
            }
            OpCode::Less { p_modes } => {
                self.op_less(p_modes);
            }
            OpCode::Eq { p_modes } => {
                self.op_eq(p_modes);
            }
            OpCode::Halt => {
                self.halted = true;
            }
        }
    }

    fn op_input(&mut self, pc: usize) {
        let out = self.mem[pc + 1] as usize;
        let val = match self.in_buf.recv() {
            Ok(v) => v,
            Err(err) => {
                println!("[{}] SendError: {:?}", self.name, err);
                panic!(err);
            }
        };
        self.mem[out] = val;
    }

    fn op_output(&mut self, p_modes: [PMode; 1], pc: usize) {
        let a = get(&self.mem, p_modes[0], pc + 1);
        match self.out_buf.send(a) {
            Ok(()) => {}
            Err(SendError(err)) => println!("[{}] SendError: {:?}", self.name, err),
        }
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
        PMode::Immediate => input[idx],
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
        2 => Ok(OpCode::Multiply {
            p_modes: [
                *p_modes.get(0).unwrap_or(&PMode::Position),
                *p_modes.get(1).unwrap_or(&PMode::Position),
            ],
        }),
        3 => Ok(OpCode::Input),
        4 => Ok(OpCode::Output {
            p_modes: [*p_modes.get(0).unwrap_or(&PMode::Position)],
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
    Add { p_modes: [PMode; 2] },
    Multiply { p_modes: [PMode; 2] },
    Input,
    Output { p_modes: [PMode; 1] },
    JmpT { p_modes: [PMode; 2] },
    JmpF { p_modes: [PMode; 2] },
    Less { p_modes: [PMode; 2] },
    Eq { p_modes: [PMode; 2] },
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
            p_modes: [PMode::Position, PMode::Position],
        });
        assert_eq!(parse_instruction(1), expected);
    }

    #[test]
    fn test_parse_cmd_modes() {
        let expected = Ok(OpCode::Multiply {
            p_modes: [PMode::Position, PMode::Immediate],
        });
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
        let (tx, rx) = channel();
        tx.send(42).unwrap();
        let mut cmp = Computer {
            in_buf: rx,
            out_buf: tx,
            mem: vec![3, 5, 6, 0, 99, 2, 3],
            pc: 0,
            halted: false,
            name: "TestUnit",
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
        let (tx, rx) = channel();
        let mut cmp = Computer {
            in_buf: rx,
            out_buf: tx,
            mem: vec![1, 0, 0, 0, 99],
            pc: 0,
            halted: false,
            name: "TestUnit",
        };
        while !cmp.halted {
            cmp.step();
        }
        assert_eq!(cmp.halted, true);
        assert_eq!(cmp.mem, &[2, 0, 0, 0, 99]);
    }

    #[test]
    fn test_interpret_case2() {
        let (tx, rx) = channel();
        let mut cmp = Computer {
            in_buf: rx,
            out_buf: tx,
            mem: vec![2, 3, 0, 3, 99],
            pc: 0,
            halted: false,
            name: "TestUnit",
        };
        while !cmp.halted {
            cmp.step();
        }
        assert_eq!(cmp.halted, true);
        assert_eq!(cmp.mem, &[2, 3, 0, 6, 99]);
    }

    #[test]
    fn test_interpret_case3() {
        let (tx, rx) = channel();
        let mut cmp = Computer {
            in_buf: rx,
            out_buf: tx,
            mem: vec![2, 4, 4, 5, 99, 0],
            pc: 0,
            halted: false,
            name: "TestUnit",
        };
        while !cmp.halted {
            cmp.step();
        }
        assert_eq!(cmp.halted, true);
        assert_eq!(cmp.mem, &[2, 4, 4, 5, 99, 9801]);
    }

    #[test]
    fn test_interpret_case4() {
        let (tx, rx) = channel();
        let mut cmp = Computer {
            in_buf: rx,
            out_buf: tx,
            mem: vec![1, 1, 1, 4, 99, 5, 6, 0, 99],
            pc: 0,
            halted: false,
            name: "TestUnit",
        };
        while !cmp.halted {
            cmp.step();
        }
        assert_eq!(cmp.halted, true);
        assert_eq!(cmp.mem, &[30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }
}
