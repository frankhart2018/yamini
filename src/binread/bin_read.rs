use std::fs::File;
use std::io::{prelude::*, BufReader};

use crate::instructions::InstructionSet;
use crate::memory::InnerData;


fn read_file_line_by_line(filepath: &str) -> Result<Vec<isize>, Box<dyn std::error::Error>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);
    let mut file_lines = Vec::new();
    let mut dec_val;

    for line in reader.lines() {
        match line {
            Ok(line) => {
                dec_val = isize::from_str_radix(&line, 2)?;
                file_lines.push(dec_val)
            },
            Err(e) => println!("Error: {}", e),
        }
    }

    Ok(file_lines)
}

pub fn read_from_file(filepath: &str) -> Vec<InstructionSet> {
    let mut buffer = Vec::new();
    match read_file_line_by_line(filepath) {
        Ok(lines) => {
            buffer = lines;
        },
        Err(e) => println!("Error: {}", e),
    }

    let mut program = Vec::new();
    let mut is_reading_arg = false;
    let mut arg: InnerData;

    for byte in buffer {
        if is_reading_arg {
            arg = byte as InnerData;
            let instruction = InstructionSet::from_int(0, Some(arg));
            program.push(instruction);
            is_reading_arg = false;
            continue;
        }

        if byte as InnerData == 0 {
            is_reading_arg = true;
        } else {
            let instruction = InstructionSet::from_int(byte as u8, None);
            program.push(instruction);
        }
    }
    program
}