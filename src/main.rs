use clap::Parser;
use std::fs;

#[derive(Parser, Debug)]
#[command(author = "Alex K.", version = "0.1", about = "A simple translator from text to Brainfuck code, built with Rust")]
pub struct Args {
    #[arg(short, long)]
    /// Filename of your source code
    pub input: Option<String>,
    
    #[arg(short, long)]
    pub output: Option<String>,
}

fn main() {
    let args = Args::parse();
    let mut output = String::new();
    if !args.input.is_some() {
        println!("Error: you didn't provided text for translation! Use option --input to specify text");
        return;
    }
    let input = args.input.unwrap();
    let symbols = input.as_bytes();

    for symbol in symbols {
        for i in 0..*symbol {
            output += "+"
        }
        output += ".>"
    }

    if !args.output.is_some() {
        println!("{}", output);
    }
    else {
        let output_dir = args.output.unwrap();
        let result = fs::write(&output_dir, output);
        if result.is_err() {
            println!("Error: cannot write output into file {}", output_dir);
            return;
        }
    }
}
