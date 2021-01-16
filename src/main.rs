use std::env::args;
use std::process;
use minigrep::Config;

fn main() {
  let config = Config::new(args()).unwrap_or_else(|err| {
    eprintln!("Problem occured: {}", err);
    println!("Usage: minigrep \"phrase_to_look_for\" \"path_to_file\"");
    process::exit(1);
  });

  if let Err(e) = minigrep::run(config) {
    eprintln!("Application error: {}", e);
    process::exit(2);
  }
}
