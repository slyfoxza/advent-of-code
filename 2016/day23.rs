use std::io::{self, Read};

#[derive(Clone, Debug)]
enum Instr {
    CpyValVal(i32, i32),
    CpyRegVal(usize, i32),
    CpyValReg(i32, usize),
    CpyRegReg(usize, usize),
    Inc(usize),
    Dec(usize),
    JnzValVal(i32, i32),
    JnzRegVal(usize, i32),
    JnzValReg(i32, usize),
    JnzRegReg(usize, usize),
    Tgl(usize)
}

enum Operand {
    Val(i32),
    Reg(usize)
}

fn compile(input: &str) -> Vec<Instr> {
    let mut program = Vec::new();
    for line in input.lines() {
        let operands = &line[4..];
        let instr = if line.starts_with("inc ") {
            Instr::Inc((operands.as_bytes()[0] - b'a') as usize)
        } else if line.starts_with("dec ") {
            Instr::Dec((operands.as_bytes()[0] - b'a') as usize)
        } else if line.starts_with("tgl ") {
            Instr::Tgl((operands.as_bytes()[0] - b'a') as usize)
        } else {
            let operands = operands.split_whitespace().collect::<Vec<_>>();
            let x = match operands[0].parse() {
                Ok(x) => Operand::Val(x),
                Err(_) => Operand::Reg(
                    (operands[0].as_bytes()[0] - b'a') as usize)
            };
            let y = match operands[1].parse() {
                Ok(y) => Operand::Val(y),
                Err(_) => Operand::Reg(
                    (operands[1].as_bytes()[0] - b'a') as usize)
            };
            if line.starts_with("cpy ") {
                match x {
                    Operand::Val(x) => match y {
                        Operand::Val(y) => Instr::CpyValVal(x, y),
                        Operand::Reg(y) => Instr::CpyValReg(x, y)
                    },
                    Operand::Reg(x) => match y {
                        Operand::Val(y) => Instr::CpyRegVal(x, y),
                        Operand::Reg(y) => Instr::CpyRegReg(x, y)
                    }
                }
            } else if line.starts_with("jnz ") {
                match x {
                    Operand::Val(x) => match y {
                        Operand::Val(y) => Instr::JnzValVal(x, y),
                        Operand::Reg(y) => Instr::JnzValReg(x, y)
                    },
                    Operand::Reg(x) => match y {
                        Operand::Val(y) => Instr::JnzRegVal(x, y),
                        Operand::Reg(y) => Instr::JnzRegReg(x, y)
                    }
                }
            } else {
                continue;
            }
        };
        program.push(instr);
    }
    return program
}

fn execute(program_: &Vec<Instr>, state: [i32; 4]) -> [i32; 4] {
    let mut program = program_.clone();
    let mut state = state.clone();
    let mut ip = 0;
    while ip < program.len() {
        let instr = program[ip].clone();
        match instr {
            Instr::Inc(x) => state[x] += 1,
            Instr::Dec(x) => state[x] -= 1,
            Instr::CpyValVal(_, _) | Instr::CpyRegVal(_, _) => {},
            Instr::CpyValReg(x, y) => state[y] = x,
            Instr::CpyRegReg(x, y) => state[y] = state[x],
            Instr::JnzValVal(x, y) => if x != 0 {
                ip = (ip as i32 + y) as usize;
                continue;
            },
            Instr::JnzRegVal(x, y) => if state[x] != 0 {
                ip = (ip as i32 + y) as usize;
                continue;
            },
            Instr::JnzValReg(x, y) => if x != 0 {
                ip = (ip as i32 + state[y]) as usize;
                continue;
            },
            Instr::JnzRegReg(x, y) => if state[x] != 0 {
                ip = (ip as i32 + state[y]) as usize;
                continue;
            },
            Instr::Tgl(x) => {
                let ipt = (ip as i32 + state[x]) as usize;
                if ipt < program.len() {
                    let instr = match program[ipt] {
                        Instr::Inc(x) => Instr::Dec(x),
                        Instr::Dec(x) | Instr::Tgl(x) => Instr::Inc(x),
                        Instr::CpyValVal(x, y) => Instr::JnzValVal(x, y),
                        Instr::CpyValReg(x, y) => Instr::JnzValReg(x, y),
                        Instr::CpyRegVal(x, y) => Instr::JnzRegVal(x, y),
                        Instr::CpyRegReg(x, y) => Instr::JnzRegReg(x, y),
                        Instr::JnzValVal(x, y) => Instr::CpyValVal(x, y),
                        Instr::JnzValReg(x, y) => Instr::CpyValReg(x, y),
                        Instr::JnzRegVal(x, y) => Instr::CpyRegVal(x, y),
                        Instr::JnzRegReg(x, y) => Instr::CpyRegReg(x, y)
                    };
                    program[ipt] = instr;
                }
            }
        }
        ip += 1;
    }
    return state
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let program = compile(&input);
    let state = execute(&program, [7, 0, 0, 0]);
    println!("{:?}", state);
}
