use std::io::Read;
use structopt::StructOpt;
use clipd::{clipboard, container};
use clipboard::{Clipboard, ClipboardType::X11};
use container::{Container, ContainerType::ClipdFs};

#[derive(Debug, StructOpt)]
#[structopt(name = "clipd", about = "A slightly smart clipboard.")]
pub struct Opt {
    #[structopt(subcommand)]
    pub cmd: Command,

    #[structopt(default_value = "default",
        help = "Specify a container to organise values into different clipboards.")]
    pub container: String,
}

#[derive(Debug, StructOpt)]
#[structopt(flatten)]
pub enum Command {
    #[structopt(visible_alias = "c", about = "Copy string into the clipboard")]
    Copy {
        
        #[structopt(short, long,
            help = "Optionally associate a custom key with the value copied to the clipboard. \
            The numbers {0, ..., n-1} are always used as keys for the last n items clipped, \
            with key = 0 being the most recent item.")]
        key: Option<String>,

        #[structopt(parse(from_str), 
            help = "Specifies the value to be copied to the clipboard")]
        value: Option<String>
    },

    #[structopt(visible_alias = "p", about = "Paste string from clipboard to stdout")]
    Paste {

        #[structopt(short, long, default_value = "0", 
            help = "Use the associated key to retrieve items from the clipboard. \
            The numbers {0, ..., n-1} can be used for the last n items added, \
            with key = 0 being the most recent item.")]
        key: String,
    },

    Clear,

    Show,
}

fn main() {
    let opt = Opt::from_args();
    let mut cnt = container::create(ClipdFs, opt.container);

    match opt.cmd {
        Command::Copy { key, value } => {
            cnt.add(key, value.unwrap_or_else(|| {
                // Copy from system clipboard if there isn't already bytes waiting from stdin (piped input)
                if atty::is(atty::Stream::Stdin) {
                    return clipboard::create(X11).paste()
                }
                
                let mut buf = String::new();
                std::io::stdin().read_to_string(&mut buf).expect("couldn't read from stdin");
                buf
            }));
        },

        Command::Paste { key } => {
            println!("{}", cnt.get(key).unwrap());
        }

        Command::Clear => { cnt.clear() } // TODO: add a warning

        Command::Show => { println!("{}", cnt.show()) }
    };  
}