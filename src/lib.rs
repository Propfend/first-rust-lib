use std::{fs, process};

/// Config is the struct that stores the
/// configuration of the query to the file.
/// 
#[derive(PartialEq, Debug)]
pub struct Config {
    pub query: String,
    pub path: String,
}

impl Config {
    /// The function `build` will construct the
    /// Config object so we can get our files content.
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query: String = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a command"),
        };
        
        let path: String = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file"),
        };

        Ok(Config { query, path })
    }
}

/// The function `get_file_content` gather the file content
/// of the specified file after the query flag.
pub fn get_file_content(config: Config) -> String {
    fs::read_to_string(config.path).unwrap_or_else(|err| {
        eprintln!("Couldn't find file path: {err}");
        process::exit(1);
    })
}
