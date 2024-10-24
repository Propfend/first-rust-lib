use std::{env, process};
use rrrryoza::Config;

fn main() {
    let args: env::Args = env::args();

    let config: Config = Config::build(args).unwrap_or_else(|err: &str| {
        eprintln!("{err}");
        process::exit(1);
    });

    let file_content: String = rrrryoza::get_file_content(config);

    println!("{}", file_content)
}
