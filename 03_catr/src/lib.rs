use std::{error::Error, io::{BufRead, BufReader, self}, fs::File};
use clap::{App, Arg};
/*
Usage: cat [OPTION]... [FILE]...
Concatenate FILE(s) to standard output.

With no FILE, or when FILE is -, read standard input.

  -A, --show-all           equivalent to -vET
  -b, --number-nonblank    number nonempty output lines, overrides -n    
  -e                       equivalent to -vE
  -E, --show-ends          display $ at end of each line
  -n, --number             number all output lines
  -s, --squeeze-blank      suppress repeated empty output lines
  -t                       equivalent to -vT
  -T, --show-tabs          display TAB characters as ^I
  -u                       (ignored)
  -v, --show-nonprinting   use ^ and M- notation, except for LFD and TAB 
      --help     display this help and exit
      --version  output version information and exit

Examples:
  cat f - g  Output f's contents, then standard input, then g's contents.
  cat        Copy standard input to standard output.

GNU coreutils online help: <https://www.gnu.org/software/coreutils/>
Report any translation bugs to <https://translationproject.org/team/>
Full documentation <https://www.gnu.org/software/coreutils/cat>
or available locally via: info '(coreutils) cat invocation'

da vedere come ispirazione https://github.com/sharkdp/bat

poi procedere per implementare come esercizio more e less andando a guardare il man
 */

type MyResult<T> = Result<T, Box<dyn Error>>;

const TAB_ESCAPE: &str = "^I";

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
    show_ends: bool,
    show_tabs: bool,
}

// --------------------------------------------------
pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
        .version("0.1.0")
        .author("j4bberwocky <@j4bberwocky@mastodon.uno>")
        .about("Rust cat")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file(s)")
                .multiple(true)
                .default_value("-"),
        )
        .arg(
            Arg::with_name("number")
                .short("n")
                .long("number")
                .help("Number lines")
                .takes_value(false)
                .conflicts_with("number_nonblank"),
        )
        .arg(
            Arg::with_name("number_nonblank")
                .short("b")
                .long("number-nonblank")
                .help("Number non-blank lines")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("show_ends")
                .short("E")
                .long("show-ends")
                .help("display $ at end of each line")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("show_tabs")
                .short("T")
                .long("show-tabs")
                .help("display TAB characters as ^I")
                .takes_value(false),
        )
        .get_matches();

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        number_lines: matches.is_present("number"),
        number_nonblank_lines: matches.is_present("number_nonblank"),
        show_ends: matches.is_present("show_ends"),
        show_tabs: matches.is_present("show_tabs"),
    })
}

// --------------------------------------------------
pub fn run(config: Config) -> MyResult<()> {
    let mut last_num = 0;
    for filename in config.files {
        match open(&filename) {
            Err(e) => eprintln!("{}: {}", filename, e),
            Ok(file) => {
                for (_, line_result) in file.lines().enumerate() {
                    let line = line_result?;
                    let mut line_to_be_printed = format!("{}", line);

                    if config.show_tabs {
                        line_to_be_printed = line_to_be_printed.replace("\t", TAB_ESCAPE);
                    }

                    if config.number_lines {
                        last_num += 1;
                        line_to_be_printed = format!("{:6}\t{}", last_num, line_to_be_printed);
                    } else if config.number_nonblank_lines {
                        if !line.is_empty() {
                            last_num += 1;
                            line_to_be_printed = format!("{:6}\t{}", last_num, line_to_be_printed);
                        } 
                    }

                    if config.show_ends {
                        line_to_be_printed = format!("{}$", line_to_be_printed);
                    }

                    println!("{}",line_to_be_printed);
                }
            }
        }
    }
    Ok(())
}

// --------------------------------------------------
fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}