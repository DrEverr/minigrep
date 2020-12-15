use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let contents = fs::read_to_string(config.filename)?;

  let results = if config.case_sensitive {
    search(&config.query, &contents)
  } else {
    search_case_insensitive(&config.query, &contents)
  };

  for line in results {
    println!("{}", line);
  }
  Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let mut result = Vec::new();
  for line in contents.lines() {
    if line.contains(query) {
      result.push(line);
    }
  }
  result
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let mut result = Vec::new();
  for line in contents.lines() {
    if line.to_lowercase().contains(&query.to_lowercase()) {
      result.push(line);
    }
  }
  result
}

pub struct Config {
  pub query: String,
  pub filename: String,
  pub case_sensitive: bool,
}

impl Config {
  pub fn new(args: &[String]) -> Result<Config, &'static str> {
    if args.len() < 3 {
      return Err("not enough arguments");
    }
    let query = args[1].clone();
    let filename = args[2].clone();
    let mut case_sensitive = true;
    for arg in args {
      if arg == "-i" {
        case_sensitive = false;
      }
    }
    Ok(Config { 
      query,
      filename,
      case_sensitive
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn one_result() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.";
    assert_eq!(vec!["safe, fast, productive."], search(query, contents));
  }

  #[test]
  fn two_results() {
    let query = "jeden";
    let contents = "\
Jeden to malo.
Dwa to wiecej niz jeden,
ale jeden jest wazne!
Z zerem moze stworzyc kazda liczbe.
Piekny jest system binarny.";
    assert_eq!(vec!["Dwa to wiecej niz jeden,", "ale jeden jest wazne!"], search(query, contents));
  }

  #[test]
  fn three_results_case_insensitive() {
    let query = "jeden";
    let contents = "\
Jeden to malo.
Dwa to wiecej niz jeden,
ale jeden jest wazne!
Z zerem moze stworzyc kazda liczbe.
Piekny jest system binarny.";
    assert_eq!(vec!["Jeden to malo.", "Dwa to wiecej niz jeden,", "ale jeden jest wazne!"], search_case_insensitive(query, contents));
  }

  #[test]
  fn no_results() {
    let query = "akimba";
    let contents = "\
Jeden to malo.
Dwa to wiecej niz jeden,
ale jeden jest wazne!
Z zerem moze stworzyc kazda liczbe.
Piekny jest system binarny.";
    assert_eq!(Vec::<String>::new(), search(query, contents));
  }
}
