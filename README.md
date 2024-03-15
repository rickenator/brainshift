# Brainshift

Brainshift extends the minimalist esoteric programming language Brainfuck, adding enhanced capabilities for a VM in the interpreter. It retains the simplicity of Brainfuck while extending additional features for more complex computations. The goal is to be light enough to run on an MCU. Existing Brainfuck programs should work the same without modification.

Additionally, Brainshift code is intended to be completely machine generated. You're already nuts if you learned Brainfuck, so if you feel the urge, it might be possible to learn the instuction set and be a Brainshift ninja. For this purpose, find SPEC.AI which you can give to your favorite AI to have it generate Brainshift code.

## Specification

### Original Brainfuck Commands

Brainshift supports all original Brainfuck commands:

- `>` - Increment the data pointer (to the next cell to the right).
- `<` - Decrement the data pointer (to the next cell to the left).
- `+` - Increment (increase by one) the byte at the data pointer.
- `-` - Decrement (decrease by one) the byte at the data pointer.
- `.` - Output the byte at the data pointer as an ASCII character.
- `,` - Input a byte, storing it in the byte at the data pointer.
- `[` - If the byte at the data pointer is zero, jump forward to the command after the matching `]`.
- `]` - If the byte at the data pointer is nonzero, jump back to the command after the matching `[`.

### Brainshift Extensions

New commands introduced in Brainshift for bitwise operations and a bit shift:

- `&` - Bitwise AND on the current cell and the next, storing the result in the current cell.
- `|` - Bitwise OR on the current cell and the next, storing the result in the current cell.
- `^` - Bitwise XOR on the current cell and the next, storing the result in the current cell.
- `~` - Bitwise NOT on the current cell.
- `#` - Right bit shift on the current cell.

...

### Understanding the Registers

- **Carry, Zero, Sign Flags**: Essential for arithmetic operations and flow control, enabling conditional logic based on arithmetic results.
- **General Purpose Register (GPR)**: Offers versatile storage for temporary data or intermediate calculations.
- **Input/Output Register (I/O)**: Facilitates streamlined input and output operations, possibly buffering data before processing or output.
- **Stack Pointer (SP)**: Crucial for managing a stack, enabling function calls, local variables, and recursion by tracking the top of the stack.
- **Program Counter (PC)**: Tracks the current position in the code being executed, essential for loops, jumps, and function calls.
- **Status Register (SR)**: Could aggregate various condition flags into one register, simplifying condition checks and status updates.
- **Instruction Register (IR)**: Holds the current instruction code, useful for debugging, instruction decoding, and executing multi-step operations.
- **Base Pointer (BP)**: Supports structured memory access within subroutines by pointing to the base of the current stack frame.
- **Flags Register (FR)**: A compact way to manage multiple status flags, streamlining operations that depend on multiple conditions.
- **Return Address Register (RAR)**: Stores the return address for subroutine calls, simplifying subroutine management and returns.
- **Compare Register (CR)**: Holds the result of comparison operations, supporting conditional branching based on comparisons beyond just zero or non-zero.
- **TEMP Registers 0-2**: Provide scratch space for complex computations, reducing the need to frequently access and modify stack or memory data.


## Usage

### Compilation

Ensure you have Rust installed. Compile the Brainshift interpreter using Cargo:

```bash
cargo build --release
```

This creates an executable in `target/release/`.

### Running Programs

To run a Brainshift program from a file:

```bash
./target/release/brainshift path/to/your/program.bf
```

To directly pass a Brainshift program as a command-line argument:

```bash
./target/release/brainshift "+[----->+++<]>."
```

This will execute the provided Brainfuck/Brainshift code.

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests to help improve Brainshift.

## License

This project is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) file for details.
