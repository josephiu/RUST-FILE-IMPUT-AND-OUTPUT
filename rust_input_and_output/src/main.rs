use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

// Function that processes lines using slicing and returns lines longer than the specified number
fn filter_long_lines(lines: &[String], min_length: usize) -> Vec<String> {
    let mut filtered = Vec::new();

    for line in lines {
        let slice = &line[..]; // using slicing to slice the  whole string
        if slice.len() > min_length {
            filtered.push(slice.to_string()); // cloning into new String
        }
    }

    filtered
}

fn main() -> io::Result<()> {
    // Immutable variable (file path)
    let input_file = "input.txt";
    let output_file = "output.txt";

    // Open input file for reading
    let file = File::open(input_file)?;
    let reader = BufReader::new(file);

    // Mutable variable to store lines
    let mut lines = Vec::new();

    // Read lines using a loop
    for line_result in reader.lines() {
        let line = line_result?; // handle Result
        lines.push(line); // push into vector
    }

    // Set minimum length (mutable variable as we might change it later)
    let min_length = 5;

    // Expression that filters lines
    let long_lines = filter_long_lines(&lines, min_length);

    // Check conditionally if any lines are found
    if long_lines.is_empty() {
        println!("No lines longer than {} characters.", min_length);
    } else {
        println!("Writing lines longer than {} characters to '{}'", min_length, output_file);

        // Open output file
        let mut output = File::create(output_file)?;

        // Loop through filtered lines and write
        for line in &long_lines {
            writeln!(output, "{}", line)?;
        }
    }

    Ok(())
}
