fn main() {
    if let Err(e) = wc_rs::get_args().and_then(wc_rs::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
