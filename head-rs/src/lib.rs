use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader, Read},
};

use clap::Parser;
type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Parser, Debug)]
#[command(author="maskudo", version=env!("CARGO_PKG_VERSION"), about="rust implementation of head util")]
pub struct Cli {
    #[arg(default_values_t = ["-".to_string()])]
    pub files: Vec<String>,
    #[arg(short = 'n', long = "lines", group = "number", default_value_t = 10)]
    /// Number of line [default:10]
    pub lines: usize,
    #[arg(short = 'c', long = "bytes", group = "number")]
    /// Number of bytes
    pub bytes: Option<usize>,
}

pub fn run(cli: Cli) -> MyResult<()> {
    for (num_file, filename) in cli.files.iter().enumerate() {
        match open(&filename) {
            Err(_) => eprintln!("head: {}: No such file or directory", filename),
            Ok(mut reader) => {
                if cli.files.len() > 1 {
                    println!(
                        "{}==> {} <==",
                        if num_file > 0 { "\n" } else { "" },
                        filename
                    );
                }
                if let Some(num_bytes) = cli.bytes {
                    let mut handle = reader.take(num_bytes as u64);
                    let mut buffer = vec![0; num_bytes];
                    let bytes_read = handle.read(&mut buffer)?;
                    print!("{}", String::from_utf8_lossy(&buffer[..bytes_read]));
                } else {
                    let mut line = String::new();
                    for _ in 0..cli.lines {
                        let bytes = reader.read_line(&mut line)?;
                        if bytes == 0 {
                            break;
                        }
                        print!("{}", line);
                        line.clear();
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
