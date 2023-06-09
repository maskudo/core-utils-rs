use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader, Write},
};

use clap::Parser;
type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Parser, Debug)]
#[command(author="maskudo", version=env!("CARGO_PKG_VERSION"), about="rust implementation of cat util")]
pub struct Cli {
    #[arg(default_value_t = format!("-"))]
    pub in_file: String,
    #[arg()]
    pub out_file: Option<String>,
    /// Show line count
    #[arg(short = 'c', long = "count")]
    pub count: bool,
}

pub fn run(cli: Cli) -> MyResult<()> {
    let mut in_file = open(&cli.in_file).map_err(|e| format!("{}: {}", cli.in_file, e))?;
    let mut out_file = open_write(cli.out_file)?;
    let mut line = String::new();
    let mut last_line = String::from("");
    let mut cur_count = 0;

    let mut print = |count: usize, text: &str| -> MyResult<()> {
        if count > 0 {
            write!(out_file, "{}{}", format_field(count, cli.count), text)?;
        }
        Ok(())
    };
    loop {
        let bytes = in_file.read_line(&mut line)?;
        if bytes == 0 as usize {
            break;
        }
        if last_line.trim_end() != line.trim_end() {
            print(cur_count, &last_line)?;
            last_line = line.clone();
            cur_count = 0;
        }
        cur_count += 1;
        line.clear();
    }
    print(cur_count, &last_line)?;
    Ok(())
}

pub fn get_args() -> MyResult<Cli> {
    let cli = Cli::parse();
    Ok(cli)
}

pub fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn open_write(filename: Option<String>) -> MyResult<Box<dyn Write>> {
    match filename {
        Some(file) => Ok(Box::new(File::create(file)?)),
        None => Ok(Box::new(io::stdout())),
    }
}

fn format_field(value: usize, show: bool) -> String {
    if show {
        format!("{:>4} ", value)
    } else {
        String::new()
    }
}
