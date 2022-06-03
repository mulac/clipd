mod cli;
mod clipboard;
use clipboard::Clipboard;
use structopt::StructOpt;

fn main() {
    // TODO: FACTORY!
    let mut cb = clipboard::clipd_fs::Container::from_file("/home/calum/.clipd/default/config.toml");
    let opt = cli::Opt::from_args();
    match opt.cmd {
        cli::Command::Copy { key, value} => {
            println!("created.. {:?}", cb);
            println!("Copying! Key: {:?}, Value: {:?}", key, value);
            cb.add(key, value);
        },

        cli::Command::Paste { key } => {
            println!("{}", cb.get(key));
        }
    };  
}