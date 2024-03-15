use std::collections::HashMap;
use std::env;
use std::io::{self, Read, Write};

// Define the memory layout

// Cell 0: Zero Flag
// Cell 1: Sign Flag
// Cell 2: General Purpose Register (GPR)
// Cell 3: Input/Output Register (I/O)
// Cell 4: Stack Pointer (SP)
// Cell 5: Program Counter (PC)
// Cell 6: Status Register (SR)
// Cell 7: Instruction Register (IR)
// Cell 8: Base Pointer (BP)
// Cell 9: Flags Register (FR)
// Cell 10: Return Address Register (RAR)
// Cell 11: Compare Register (CR)
// Cell 12: TEMP Register 0
// Cell 13: TEMP Register 1
// Cell 14: TEMP Register 2
// Cell 15: TEMP Register 3

const DEFAULT_MEMORY_SIZE: usize = 65536;
const USER_DATA_START: usize = 16;
const REGISTERS: usize = 16;
const REG_ZERO: usize = 0;
const REG_SIGN: usize = 1;
const REG_GPR: usize = 2;
const REG_IO: usize = 3;
const REG_SP: usize = 4;
const REG_PC: usize = 5;
const REG_SR: usize = 6;
const REG_IR: usize = 7;
const REG_BP: usize = 8;
const REG_FR: usize = 9;
const REG_RAR: usize = 10;
const REG_CR: usize = 11;
const REG_TEMP0: usize = 12;
const REG_TEMP1: usize = 13;
const REG_TEMP2: usize = 14;
const REG_TEMP3: usize = 15;

// Status Register Flags
const SR_OVERFLOW: u8 = 1 << 0;
const SR_UNDERFLOW: u8 = 1 << 1;
const SR_DIVIDE_BY_ZERO: u8 = 1 << 2;
const SR_INVALID_INSTRUCTION: u8 = 1 << 3;
const SR_INVALID_MEMORY_ACCESS: u8 = 1 << 4;
const SR_INVALID_MEMORY_ALIGNMENT: u8 = 1 << 5;
const SR_INVALID_MEMORY_SIZE: u8 = 1 << 6;
const SR_INVALID_MEMORY_RANGE: u8 = 1 << 7;

// Define a macro to adjust memory access for the reserved system area
macro_rules! mem {
    ($memory:expr, $index:expr) => {
        $memory[USER_DATA_START + $index]
    };
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut memory_size: usize = 65536; // Default memory size
    let mut program_source: Option<String> = None;

    let mut args_iter = args.iter().skip(1); // Skip the program name
    while let Some(arg) = args_iter.next() {
        match arg.as_str() {
            "-m" => {
                if let Some(m) = args_iter.next() {
                    memory_size = m.parse().unwrap_or(65536);
                }
            },
            "-p" => {
                if let Some(p) = args_iter.next() {
                    program_source = Some(p.clone());
                }
            },
            _ => {}
        }
    }

    let program = match program_source {
        Some(prog) => prog,
        None => {
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer)
                       .expect("Failed to read program from stdin");
            buffer
        },
    };

    // Use `memory_size` and `program` as needed
    println!("Memory Size: {} bytes", memory_size);
    println!("Program: {}", program);
    
    execute(&program, memory_size);
}

fn execute(program: &str, memory_size: usize) {
    // Allocate memory based on the specified size
    let mut memory = vec![0u8; memory_size];
    let mut ptr = USER_DATA_START; // Adjusted to start after system area
    let mut pc = 0usize; // Adjusted to start after system area;
    let mut sp = memory_size; // Adjusted to just beyond the top of the memory
    let mut input_buffer = Vec::new();
    let labels = parse_labels(program);

    // Initialize the stack pointer at the top of the memory
    let stack_start = memory_size - 1;
    memory[REG_SP] = stack_start as u8;

    while pc < program.len() {
        // Execute the current instruction
        memory[REG_PC] = pc as u8;

        let ir = program.as_bytes()[pc];
        memory[REG_IR] = ir;

        match ir {
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
            }
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
            }
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
            }

            // BrainShift extensions
            b'0' => {
                memory[ptr] = 0;
            }
            b'&' => {
                // Logical AND
                memory[ptr] &= memory[ptr.wrapping_add(1)];
            }
            b'|' => {
                // Logical OR
                memory[ptr] |= memory[ptr.wrapping_add(1)];
            }
            b'^' => {
                // Logical XOR
                memory[ptr] ^= memory[ptr.wrapping_add(1)];
            }
            b'~' => {
                // Logical NOT
                memory[ptr] = !memory[ptr];
            }
            b'#' => {
                memory[ptr] >>= 1; // Right bit shift
            }
            b'@' => {
                memory[ptr] <<= 1; // Left bit shift
            }

            // Arithmetic operations
            b'A' => {
                // Addition with Carry
                let (result, overflow) = memory[ptr].overflowing_add(memory[ptr.wrapping_add(1)]);
                memory[ptr] = result;
                if overflow {
                    memory[REG_SR] |= SR_OVERFLOW;
                } else {
                    memory[REG_SR] &= !SR_OVERFLOW; // Clear the overflow flag if not overflowed
                }
            }
            b'M' => {
                // Multiplication with Overflow
                let (result, overflow) = memory[ptr].overflowing_mul(memory[ptr.wrapping_add(1)]);
                memory[ptr] = result;
                if overflow {
                    memory[REG_SR] |= SR_OVERFLOW;
                } else {
                    memory[REG_SR] &= !SR_OVERFLOW; // Clear the overflow flag if not overflowed
                }
            }
            b'S' => {
                // Subtraction with Underflow
                let (result, overflow) = memory[ptr].overflowing_sub(memory[ptr.wrapping_add(1)]);
                memory[ptr] = result;
                if overflow {
                    memory[REG_SR] |= SR_OVERFLOW;
                } else {
                    memory[REG_SR] &= !SR_OVERFLOW; // Clear the overflow flag if not overflowed
                }
            }
            b'D' => {
                // Division with Remainder
                if memory[ptr.wrapping_add(1)] != 0 {
                    let divisor = memory[ptr.wrapping_add(1)];
                    let quotient = memory[ptr] / divisor;
                    let remainder = memory[ptr] % divisor;
                    memory[ptr] = quotient;
                    memory[ptr.wrapping_add(1)] = remainder; // Store the remainder in the next cell
                } else {
                    memory[REG_SR] |= SR_DIVIDE_BY_ZERO;
                }
            }
            b'%' => {
                // Modulus
                if memory[ptr.wrapping_add(1)] != 0 {
                    memory[ptr] %= memory[ptr.wrapping_add(1)];
                }
            }
            b'!' => {
                // Negation
                memory[ptr] = !memory[ptr];
            },

            // Control flow operations
            
            b'J' | b'C' => {
                // Find the end of the current instruction based on a space or newline
                let remaining_program = &program[pc..];
                let end_of_instruction = remaining_program.find(' ')
                                        .or_else(|| remaining_program.find('\n'))
                                        .unwrap_or(remaining_program.len());
                
                let instruction = &remaining_program[..end_of_instruction];
                let label_name = instruction.split_whitespace().nth(1).unwrap().trim_start_matches('*');
                
                if let Some(&address) = labels.get(label_name) {
                    if ir == b'J' {
                        pc = address; // For 'J', jump to the address
                    } else {
                        // For 'C', call subroutine (example simplified logic)
                        // Push return address to stack (not shown here)
                        // Then jump to the subroutine's starting address
                        pc = address;
                    }
                    continue; // Skip the automatic pc increment at the end of the loop
                } else {
                    println!("Label '{}' not found.", label_name);
                    // Handle error, potentially halting execution
                }
            },
            b'R' => {
                // Return from subroutine
                pc = pop_from_stack(&mut memory, &mut sp);
            }
            b';' => { // End-of-sequence opcode
                println!("End of program sequence reached.");
                break; // Exit the execution loop
            },

            _ => {} // Ignore any other characters
        }
        pc = pc.wrapping_add(1);
    }
}

fn push_to_stack(memory: &mut Vec<u8>, sp: &mut usize, value: u8) {
    if *sp == 0 {
        panic!("Stack overflow");
    }
    *sp -= 1; // Decrement SP first to get to a valid index
    memory[*sp] = value; // Use SP to index into memory and store value
}

fn pop_from_stack(memory: &mut Vec<u8>, sp: &mut usize) -> usize {
    // Assuming addresses are stored in a larger format (e.g., two bytes)
    // This is a simplified example; adjust according to how you're storing addresses
    if *sp >= memory.len() - 2 {
        panic!("Stack underflow");
    }
    let high_byte = memory[*sp] as usize;
    let low_byte = memory[*sp + 1] as usize;
    *sp += 2; // Adjust based on the size of addresses in your stack
    (high_byte << 8) | low_byte
}

fn resolve_address(labels: &HashMap<String, usize>, label: &str) -> usize {
    *labels.get(label).expect("Label not found")
}

fn parse_labels(program: &str) -> HashMap<String, usize> {
    let mut labels = HashMap::new();
    let mut address = 0;

    for line in program.lines() {
        if let Some((label, _)) = line.split_once(':') {
            labels.insert(label.trim().to_string(), address);
        } else {
            address += line.len(); // Adjust how addresses are calculated based on your needs
        }
    }

    labels
}

