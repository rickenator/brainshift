# Brainshift

Brainshift extends the minimalist esoteric programming language Brainfuck, adding enhanced capabilities for bitwise operations and bit shifts. It retains the simplicity of Brainfuck while introducing additional features for more complex computations.

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
