use clipd::cli;

fn main() {
    cli::run();

    // TODO: we should be calling as below
    //
    // if let Err(e) = cli::run() {
    //     eprintln!("{:?}", e);
    //     std::process::exit(1);
    // }
}