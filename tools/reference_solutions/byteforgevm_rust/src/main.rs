use std::env;
use std::fs;
use std::path::Path;
use std::process::ExitCode;

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), String> {
    let args = env::args().skip(1).collect::<Vec<_>>();
    match args.as_slice() {
        [command, path] if command == "disasm" => disasm(Path::new(path)),
        [command, path] if command == "run" => execute(Path::new(path), false),
        [command, path] if command == "trace" => execute(Path::new(path), true),
        _ => Err("usage: byteforgevm <disasm|run|trace> <program-file>".to_string()),
    }
}

#[derive(Debug, Clone)]
struct Instruction {
    op: String,
    arg: Option<i64>,
}

fn disasm(path: &Path) -> Result<(), String> {
    let program = parse_program(path)?;
    for (ip, instruction) in program.iter().enumerate() {
        if let Some(arg) = instruction.arg {
            println!("{ip:04} {} {arg}", instruction.op);
        } else {
            println!("{ip:04} {}", instruction.op);
        }
    }
    Ok(())
}

fn execute(path: &Path, trace: bool) -> Result<(), String> {
    let program = parse_program(path)?;
    let mut ip = 0usize;
    let mut stack = Vec::<i64>::new();
    let mut calls = Vec::<usize>::new();

    while ip < program.len() {
        let instruction = &program[ip];
        if trace {
            println!(
                "ip={ip} op={} stack={}",
                instruction.op,
                format_stack(&stack)
            );
        }
        match instruction.op.as_str() {
            "PUSH" => {
                stack.push(required_arg(instruction, ip)?);
                ip += 1;
            }
            "ADD" => {
                let (left, right) = pop_two(&mut stack, ip)?;
                stack.push(left + right);
                ip += 1;
            }
            "SUB" => {
                let (left, right) = pop_two(&mut stack, ip)?;
                stack.push(left - right);
                ip += 1;
            }
            "MUL" => {
                let (left, right) = pop_two(&mut stack, ip)?;
                stack.push(left * right);
                ip += 1;
            }
            "PRINT" => {
                let value = stack
                    .pop()
                    .ok_or_else(|| format!("stack underflow at {ip}"))?;
                println!("{value}");
                ip += 1;
            }
            "JMP" => {
                ip = valid_target(required_arg(instruction, ip)?, program.len())?;
            }
            "JZ" => {
                let value = stack
                    .pop()
                    .ok_or_else(|| format!("stack underflow at {ip}"))?;
                if value == 0 {
                    ip = valid_target(required_arg(instruction, ip)?, program.len())?;
                } else {
                    ip += 1;
                }
            }
            "CALL" => {
                let target = valid_target(required_arg(instruction, ip)?, program.len())?;
                calls.push(ip + 1);
                ip = target;
            }
            "RET" => {
                ip = calls
                    .pop()
                    .ok_or_else(|| format!("call stack underflow at {ip}"))?;
            }
            "HALT" => break,
            other => return Err(format!("unknown opcode {other} at {ip}")),
        }
    }

    Ok(())
}

fn parse_program(path: &Path) -> Result<Vec<Instruction>, String> {
    let source = fs::read_to_string(path).map_err(|error| error.to_string())?;
    source
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let mut parts = line.split_whitespace();
            let op = parts
                .next()
                .ok_or_else(|| "empty instruction".to_string())?
                .to_string();
            let arg = parts
                .next()
                .map(|value| {
                    value
                        .parse::<i64>()
                        .map_err(|_| format!("invalid argument {value}"))
                })
                .transpose()?;
            Ok(Instruction { op, arg })
        })
        .collect()
}

fn required_arg(instruction: &Instruction, ip: usize) -> Result<i64, String> {
    instruction
        .arg
        .ok_or_else(|| format!("missing argument at {ip}"))
}

fn pop_two(stack: &mut Vec<i64>, ip: usize) -> Result<(i64, i64), String> {
    let right = stack
        .pop()
        .ok_or_else(|| format!("stack underflow at {ip}"))?;
    let left = stack
        .pop()
        .ok_or_else(|| format!("stack underflow at {ip}"))?;
    Ok((left, right))
}

fn valid_target(target: i64, program_len: usize) -> Result<usize, String> {
    if target < 0 || target as usize >= program_len {
        Err(format!("invalid jump target {target}"))
    } else {
        Ok(target as usize)
    }
}

fn format_stack(stack: &[i64]) -> String {
    let values = stack
        .iter()
        .map(i64::to_string)
        .collect::<Vec<_>>()
        .join(", ");
    format!("[{values}]")
}
