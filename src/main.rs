use std::fs::File;
use std::io::{Write, BufReader, BufRead, Error};

fn get_seq() -> String {
    let mut output = String::new();

    for i in 0..10 {
        for _ in 0..i {
            output.push_str(" * ");
        }
        output.push_str("\n");
    }

    return output;
}

fn main() -> Result<(), Error> {
    // Writing hello.txt file
    let path = "hello.txt";

    let mut output = File::create(path)?;

    let seq_str = get_seq();
    write!(output, "{}", seq_str)?;

    // Reading from that file
    let input = File::open(path)?;
    let buffered = BufReader::new(input);
    
    for line in buffered.lines() {
        println!("{}", line?);
    }

    Ok(())
}
