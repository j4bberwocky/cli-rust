use std::error::Error;
use clap::{App, Arg};

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()> {
    dbg!(config);
    Ok(())
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr") 
        .version("0.1.0")
        .author("@j4bberwocky@mastodon.uno")
        .about("Rust simplified version of cat program")
        .arg(
            Arg::with_name("files")
                .value_name("FILES")
                .help("Input files")
                .required(false)
                .min_values(1),
        )
        .arg(
            Arg::with_name("number_lines")
                .short("n")
                .help("number all output lines")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("number_nonblank_lines")
                .short("b")
                .help("number nonempty output lines, overrides -n")
                .takes_value(false),
        )
        .get_matches();

    Ok(
        Config {
            files: matches.values_of_lossy("files").unwrap(),
            number_lines: matches.is_present("number_lines"),
            number_nonblank_lines: matches.is_present("number_nonblank_lines"),
        }
    )

}