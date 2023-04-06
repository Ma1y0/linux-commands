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
    fn new(path: &str) -> Result<Table, Box<dyn std::error::Error>> {
        let mut lines: Vec<PathBuf> = Vec::new();
        let name = path.clone().to_string();
        let content = fs::read_dir(path)?;
        // TODO: Change to iter
        for item in content {
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
        write!(f, "[{}]\n", self.name)?;
        // TODO: iter
        for i in &self.lines {
            write!(f, "{:?}\n", i)?;
        }
        Ok(())
    }
}

pub fn run(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let out: Vec<Table> = config.paths.iter().map(|x| Table::new(x).unwrap()).collect();
    
    for i in out {
        println!("{}", i);
    }
    Ok(())
}
