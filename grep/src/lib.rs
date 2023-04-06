use std::fs;

pub struct Config {
    pub query: String,
    pub path: String
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enought arguments");
        }

        let query = args[1].clone();
        let path = args[2].clone();

        Ok(Config { query, path})
    }
}

fn search<'a>(query: &'a str, file: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    file.lines().filter(|x| x.to_lowercase().contains(&query)).collect()
}

pub fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let file = fs::read_to_string(&config.path)?;
    let result = search(&config.query, &file);
    
    for line in result {
        println!("{}", line);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search() {
        let text = "Hello how are you \n 15 \nsun is watching";
        let query = "sun";

        assert_eq!(vec!["sun is watching"], search(&query, &text));
    }
}
