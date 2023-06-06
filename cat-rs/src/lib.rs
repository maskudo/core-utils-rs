use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
};

use clap::Parser;
type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Parser, Debug)]
#[command(author="maskudo", version=env!("CARGO_PKG_VERSION"), about="rust implementation of cat util")]
pub struct Cli {
    #[arg(default_values_t = ["-".to_string()])]
    pub files: Vec<String>,
    #[arg(short = 'n', long = "nonempty-linenums", group = "number")]
    /// Number lines
    pub number_lines: bool,
    #[arg(short = 'e', long = "empty-linenums", group = "number")]
    /// Number nonblank lines
    pub number_nonblank_lines: bool,
}

pub fn run(cli: Cli) -> MyResult<()> {
    for filename in cli.files {
        match open(&filename) {
            Err(_) => eprintln!("{} >> error.", filename),
            Ok(reader) => {
                let mut line_num = 0;
                for (index, line) in reader.lines().enumerate() {
                    let line = line?;
                    if cli.number_lines {
                        println!("{:>6}\t{}", index + 1, line);
                    } else if cli.number_nonblank_lines {
                        if !line.is_empty() {
                            line_num += 1;
                            println!("{:>6}\t{}", line_num, line);
                        } else {
                            println!("");
                        }
                    } else {
                        println!("{}", line);
                    }
                }
            }
        }
    }
    Ok(())
}

pub fn get_args() -> MyResult<Cli> {
    Ok(Cli::parse())
}

pub fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
