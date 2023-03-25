use clap::Parser;
use std::fs;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(index = 1)]
    from: String,
    #[arg(index = 2)]
    to: String
}

fn main() {
    let args = Args::parse();
    
    match fs::copy(args.from, args.to) {
        Ok(_) => println!("File succesfully copied"),
        Err(e) => println!("Failed to copy the file with error: {}", e)
    }

}
