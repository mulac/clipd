pub mod cli;
use structopt::StructOpt;

fn main() {
    let opt = cli::Opt::from_args();
    match opt.cmd {
        cli::Command::Copy { key, value} => {
            println!("Copying! Key: {:?}, Value: {:?}", key, value)
        },

        cli::Command::Paste { key } => {
            println!("Pasting {:?}!", key)
        }
    };  
}