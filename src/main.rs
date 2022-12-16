use clap::Parser;

#[derive(Parser, Debug)]
#[command(author = "Alex K.", version = "0.1", about = "A simple translator from text to Brainfuck code, built with Rust")]
pub struct Args {
    #[arg(short, long)]
    /// Filename of your source code
    pub input: Option<String>,
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

    println!("{}", output);
}
