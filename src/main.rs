use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "clipd", about = "A slightly smart clipboard.")]
struct Opt {
    #[structopt(parse(from_os_str))]
    pub file: PathBuf,
}

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt.file);
}