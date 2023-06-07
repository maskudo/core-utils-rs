fn main() {
    if let Err(e) = head_rs::get_args().and_then(head_rs::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

