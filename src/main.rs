use std::env;
use std::io::{self, Read, Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: brainshift <program>");
        std::process::exit(1);
    }
    let program = &args[1];
    execute(program);
}

fn execute(program: &str) {
    let mut memory = vec![0u8; 30000];
    let mut ptr = 0usize;
    let mut pc = 0usize;
    let mut input_buffer = Vec::new();

    while pc < program.len() {
        match program.as_bytes()[pc] {
            b'>' => ptr = ptr.wrapping_add(1),
            b'<' => ptr = ptr.wrapping_sub(1),
            b'+' => memory[ptr] = memory[ptr].wrapping_add(1),
            b'-' => memory[ptr] = memory[ptr].wrapping_sub(1),
            b'.' => print!("{}", memory[ptr] as char),
            b',' => {
                if input_buffer.is_empty() {
                    let mut buffer = [0; 1];
                    io::stdin().read_exact(&mut buffer).unwrap();
                    input_buffer.push(buffer[0]);
                }
                memory[ptr] = input_buffer.remove(0);
            },
            b'[' => {
                if memory[ptr] == 0 {
                    let mut loop_counter = 1;
                    while loop_counter > 0 {
                        pc = pc.wrapping_add(1);
                        match program.as_bytes()[pc] {
                            b'[' => loop_counter += 1,
                            b']' => loop_counter -= 1,
                            _ => (),
                        }
                    }
                }
            },
            b']' => {
                if memory[ptr] != 0 {
                    let mut loop_counter = 1;
                    while loop_counter > 0 {
                        pc = pc.wrapping_sub(1);
                        match program.as_bytes()[pc] {
                            b']' => loop_counter += 1,
                            b'[' => loop_counter -= 1,
                            _ => (),
                        }
                    }
                }
            },
            b'&' => {
                memory[ptr] &= memory[ptr.wrapping_add(1)];
            },
            b'|' => {
                memory[ptr] |= memory[ptr.wrapping_add(1)];
            },
            b'^' => {
                memory[ptr] ^= memory[ptr.wrapping_add(1)];
            },
            b'~' => {
                memory[ptr] = !memory[ptr];
            },
            b'#' => {
                memory[ptr] >>= 1; // Right bit shift
            },
            _ => {}, // Ignore any other characters
        }
        pc = pc.wrapping_add(1);
    }
}
