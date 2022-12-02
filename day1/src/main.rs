use clap::Parser;
mod aoc;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file
    file: String,
}


fn main() {
    let args = Args::parse();
    println!("Processing file: {}", args.file);

    aoc::run(args.file);
}
