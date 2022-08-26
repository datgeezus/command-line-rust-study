fn main() {
    if let Err(e) = catr::run() {
        eprint!("{}", e);
        std::process::exit(1);
    }

    let args = catr::get_args();
}
