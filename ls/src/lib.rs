use clap::Parser;
use std::{fs, path::PathBuf, fmt};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    #[arg(index = 1)]
    pub paths: Vec<String>,
    #[arg(short, long, default_value_t = false)]
    pub all: bool,
    #[arg(short, long, default_value_t = false)]
    pub long: bool
}

struct Table {
    name: String,
    lines: Vec<PathBuf>
}

impl Table {
    fn new(config: &Config) -> Result<Table, Box<dyn std::error::Error>> {
        let name = config.name.clone().to_string();
        let mut lines: Vec<PathBuf> = Vec::new();
        let contetnt = fs::read_dir(config.path[0])?;
        for item in contetnt {
            let item = item?;
            lines.push(item.path());
        }

        Ok(Table {
            name,
            lines
        })
        
    }
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]\n{:?}", self.name, self.lines)
    }
}

pub fn run(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let a = Table::new(&config)?;

    println!("{:?}", contetns);
    Ok(())
}
