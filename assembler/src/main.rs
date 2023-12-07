use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// Define your instruction set here
struct InstructionSet {
    // Instruction mappings and properties
}

fn parse_line(line: &str) -> Result<String, &'static str> {
    // Parse and tokenize the assembly line
    // Return Result with Ok(instruction) or Err(error)
    Ok(String::from(line)) // Placeholder
}

fn assemble_instruction(instruction: &str) -> Result<String, &'static str> {
    // Convert instruction to binary format
    Ok(String::from(instruction)) // Placeholder
}

fn binary_to_hex(binary_str: &str) -> String {
    // Convert binary string to hexadecimal
    format!("{:X}", isize::from_str_radix(binary_str, 2).unwrap())
}

fn main() -> io::Result<()> {
    // Example: reading lines from a file
    let path = Path::new("input.asm");
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    for line in io::BufReader::new(file).lines() {
        let line = line?;
        let instruction = parse_line(&line)?;
        let binary_instr = assemble_instruction(&instruction)?;
        let hex_instr = binary_to_hex(&binary_instr);
        println!("{}", hex_instr);
    }

    Ok(())
}
