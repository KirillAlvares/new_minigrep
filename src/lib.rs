use std::error::Error;
use std::fs;

pub struct Config {
    query: String,
    file_path: String,
}
impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        Ok (Config {query, file_path})
    }
}

pub fn run(config: Config)-> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    for line in search(&config.query, &contents){
        println!("{line}");
    }
    
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn good_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
            assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn bad_result() {
        let query = "peperoni";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
            assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}