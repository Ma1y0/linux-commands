use clap::Parser;
use std::fs;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(index = 1)]
    paths: Vec<String>,
    #[arg(short, long, default_value_t = false)]
    numbers: bool
}

fn main() {
    let args = Args::parse();

    let files: Vec<String> = args.paths.iter().map(|x| fs::read_to_string(x).unwrap()).collect();


    for (f_i, file) in files.clone().iter().enumerate() {
        if args.numbers {
            let file = file.split("\n");
            for (i, line) in file.enumerate() {
                if !line.is_empty() {
                    println!("      {} {}", i + 1, line);
                }
            }
            if files.len() > 1 && f_i != files.len() - 1 {
                println!();
                println!();
                println!();
            }
        } else {
            println!("{}", file);
        }
    }
}
