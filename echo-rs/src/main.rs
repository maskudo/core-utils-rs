use clap::Parser;
use echo_rs::Cli;

fn main() {
    let cli = Cli::parse();
    let omit_newline = cli.omit_newline;
    let text = cli.text.unwrap();
    print!("{}", text.join(" "));
    if !omit_newline {
        println!("")
    }
}
