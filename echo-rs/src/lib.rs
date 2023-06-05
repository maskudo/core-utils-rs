use clap::Parser;

#[derive(Parser)]
#[command(author="maskudo", version=env!("CARGO_PKG_VERSION"), about="rust implementation of echo util", long_about = None, arg_required_else_help(true))]
pub struct Cli {
    #[arg()]
    pub text: Option<Vec<String>>,
    #[arg(short = 'n')]
    pub omit_newline: bool,
}
