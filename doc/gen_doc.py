from fpdf import FPDF

# Create instance of FPDF class
pdf = FPDF()

# Add a page
pdf.add_page()

# Set font
pdf.set_font("Arial", size = 12)

# Adding a cell
pdf.cell(200, 10, txt = "Brainshift VM Operator Reference", ln = True, align = 'C')

# Add a line break
pdf.ln(10)

# Operator descriptions
operators = [
    (">", "Move the pointer to the right."),
    ("<", "Move the pointer to the left."),
    ("+", "Increment the byte at the pointer."),
    ("-", "Decrement the byte at the pointer."),
    (".", "Output the character signified by the byte at the pointer."),
    (",", "Input a character and store it in the byte at the pointer."),
    ("[", "Jump past the matching `]` if the byte at the pointer is 0."),
    ("]", "Jump back to the matching `[` if the byte at the pointer is nonzero."),
    ("&", "Logical AND operation between the current cell and the next cell."),
    ("|", "Logical OR operation between the current cell and the next cell."),
    ("^", "Logical XOR operation between the current cell and the next cell."),
    ("~", "Logical NOT operation on the current cell."),
    ("#", "Right bit shift on the current cell."),
    ("@", "Left bit shift on the current cell."),
    ("A", "Addition with carry: Adds the next cell to the current cell, checks for overflow."),
    ("M", "Multiplication with overflow: Multiplies the current cell with the next cell."),
    ("S", "Subtraction with underflow: Subtracts the next cell from the current cell."),
    ("D", "Division with remainder: Divides the current cell by the next cell."),
    ("%", "Modulus operation: Finds the remainder when the current cell is divided by the next cell."),
    ("!", "Negation: Flips all bits in the current cell."),
    ("J", "Jump to a label if the condition is met."),
    ("C", "Call a subroutine at a label, saving the return address on the stack."),
    ("R", "Return from a subroutine, using the address at the top of the stack."),
    ("Z", "Set the Zero flag if the current cell is 0."),
    ("z", "Clear the Zero flag."),
    ("j", "Jump to a label if the Zero flag is set."),
    ("n", "Jump to a label if the Zero flag is not set."),
    (";", "Denotes the end of the program sequence."),
    ('"', "Used to denote the beginning and end of a comment."),
    ("*", "Used to denote a label for jumps and calls.")
]

# Writing each operator to the PDF
for op, desc in operators:
    pdf.cell(0, 10, f"{op} - {desc}", 0, 1)

# Adding section on comments
pdf.ln(10)  # Add a line break
pdf.set_font("Arial", 'B', 12)
pdf.cell(0, 10, "Special Syntax", 0, 1)
pdf.set_font("Arial", size = 12)
pdf.cell(0, 10, '" - Encloses comments. Everything between two " characters is ignored.', 0, 1)
pdf.cell(0, 10, '* - Precedes label names used in jump (J) and call (C) instructions',0,1)
pdf.cell(0, 10, '  - and jump if zero sb (j) and (n) instructions for control flow. ',0,1)
pdf.cell(0, 10, '  - Label is terminated by a space, tab or newline .', 0, 1)

# Save the PDF to a file
pdf.output("./Brainshift_VM_Operators.pdf")

#"./Brainshift_VM_Operators.pdf"


