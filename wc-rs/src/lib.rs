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
    /// Show line count
    #[arg(short = 'l', long = "lines")]
    pub lines: bool,
    /// Show bytes count
    #[arg(short = 'c', long = "bytes", group = "char")]
    pub bytes: bool,
    /// Show chars count
    #[arg(short = 'm', long = "chars", group = "char")]
    pub chars: bool,
    /// Show words count
    #[arg(short = 'w', long = "words")]
    pub words: bool,
}

pub fn run(cli: Cli) -> MyResult<()> {
    let mut total_lines = 0;
    let mut total_words = 0;
    let mut total_bytes = 0;
    let mut total_chars = 0;
    for filename in &cli.files {
        match open(filename) {
            Err(e) => eprintln!("wc: {}: {} ", filename, e),
            Ok(mut reader) => {
                let mut lines = 0;
                let mut words = 0;
                let mut bytes = 0;
                let mut chars = 0;

                loop {
                    let mut buf = String::new();
                    let line_bytes = reader.read_line(&mut buf)?;
                    if line_bytes == 0 {
                        break;
                    }
                    bytes += line_bytes;
                    lines += 1;
                    words += buf.split_whitespace().count();
                    chars += buf.chars().count();
                }
                println!(
                    "{}{}{}{}{}",
                    format_field(lines, cli.lines),
                    format_field(words, cli.words),
                    format_field(bytes, cli.bytes),
                    format_field(chars, cli.chars),
                    if filename == "-" {
                        "".to_string()
                    } else {
                        format!(" {}", filename)
                    }
                );
                total_lines += lines;
                total_words += words;
                total_bytes += bytes;
                total_chars += chars;
            }
        }
    }
    if cli.files.len() > 1 {
        println!(
            "{}{}{}{} total",
            format_field(total_lines, cli.lines),
            format_field(total_words, cli.words),
            format_field(total_bytes, cli.bytes),
            format_field(total_chars, cli.chars)
        );
    }
    Ok(())
}

pub fn get_args() -> MyResult<Cli> {
    let mut cli = Cli::parse();
    if [cli.words, cli.lines, cli.chars, cli.bytes]
        .iter()
        .all(|v| !(*v))
    {
        cli.words = true;
        cli.lines = true;
        cli.bytes = true;
    }
    Ok(cli)
}

pub fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn format_field(value: usize, show: bool) -> String {
    if show {
        format!("{:>8}", value)
    } else {
        String::new()
    }
}
