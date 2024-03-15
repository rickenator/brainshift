use std::env;
use std::io::{self, Read, Write};

// Define the memory layout
// Cell 0: Carry Flag
// Cell 1: Zero Flag
// Cell 2: Sign Flag
// Cell 3: General Purpose Register (GPR)
// Cell 4: Input/Output Register (I/O)
// Cell 5: Stack Pointer (SP)
// Cell 6: Program Counter (PC)
// Cell 7: Status Register (SR)
// Cell 8: Instruction Register (IR)
// Cell 9: Base Pointer (BP)
// Cell 10: Flags Register (FR)
// Cell 11: Return Address Register (RAR)
// Cell 12: Compare Register (CR)
// Cell 13: TEMP Register 0
// Cell 14: TEMP Register 1
// Cell 15: TEMP Register 2

const DEFAULT_MEMORY_SIZE: usize = 65536;
const USER_DATA_START: usize = 16;
const REGISTERS: usize = 16;
const REG_CARRY: usize = 0;
const REG_ZERO: usize = 1;
const REG_SIGN: usize = 2;
const REG_GPR: usize = 3;
const REG_IO: usize = 4;
const REG_SP: usize = 5;
const REG_PC: usize = 6;
const REG_SR: usize = 7;
const REG_IR: usize = 8;
const REG_BP: usize = 9;
const REG_FR: usize = 10;
const REG_RAR: usize = 11;
const REG_CR: usize = 12;
const REG_TEMP0: usize = 13;
const REG_TEMP1: usize = 14;
const REG_TEMP2: usize = 15;


// Define a macro to adjust memory access for the reserved system area
macro_rules! mem {
    ($memory:expr, $index:expr) => {
        $memory[USER_DATA_START + $index]
    };
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: brainshift <program> [memory size in bytes]");
        std::process::exit(1);
    }
    
    // Parse the optional memory size argument, defaulting to 64K if not specified
    let memory_size = args.get(2)
                          .and_then(|size| size.parse::<usize>().ok())
                          .unwrap_or(DEFAULT_MEMORY_SIZE);

    // Ensure the memory size is at least large enough to accommodate the system area
    if memory_size < USER_DATA_START {
        eprintln!("Memory size must be at least {}", USER_DATA_START);
        std::process::exit(1);
    }

    let program = &args[1];
    execute(program, memory_size);
}

fn execute(program: &str, memory_size: usize) {
    // Allocate memory based on the specified size
    let mut memory = vec![0u8; memory_size];
    let mut ptr = USER_DATA_START; // Adjusted to start after system area
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
