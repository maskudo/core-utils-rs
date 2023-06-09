fn main() {
    if let Err(e) = uniq_rs::get_args().and_then(uniq_rs::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
