use std::io::Read;
use structopt::StructOpt;
use crate::{clipboard, container};
use clipboard::{Clipboard, ClipboardType::X11};
use container::{Container, ContainerType::ClipdFs};

#[derive(Debug, StructOpt)]
#[structopt(name = "clipd", about = "A slightly smart clipboard.")]
pub struct Opt {
    #[structopt(
        default_value = "default",
        help = "Specify a clipboard container")]
    pub container: String,

    #[structopt(parse(from_str),
        help = "Specify a key.  Either a custom string or number. \
        The numbers {0, ..., n-1} are always used as keys for the last n items clipped, \
        with key = 0 being the most recent item.")]
    pub key: Option<String>,

    #[structopt(subcommand)]
    pub cmd: Option<Command>,
}

#[derive(Debug, StructOpt)]
#[structopt(flatten)]
pub enum Command {
    #[structopt(visible_alias = "c", about = "Copy string into the clipboard")]
    Copy {
        
        // #[structopt(short, long,
        //     help = "Optionally associate a custom key with the value copied to the clipboard. \
        //     The numbers {0, ..., n-1} are always used as keys for the last n items clipped, \
        //     with key = 0 being the most recent item.")]
        // key: Option<String>,

        #[structopt(parse(from_str), 
            help = "Specifies the value to be copied to the clipboard")]
        value: Option<String>
    },

    #[structopt(visible_alias = "p", about = "Paste string from clipboard to stdout")]
    Paste,

        // #[structopt(default_value = "0", 
        //     help = "Use the associated key to retrieve items from the clipboard. \
        //     The numbers {0, ..., n-1} can be used for the last n items added, \
        //     with key = 0 being the most recent item.")]
        // key: String,
    

    #[structopt(visible_alias = "s", about = "Show a table view of a container (take a peek)")]
    Show {

        #[structopt(short, default_value = "10", 
            help = "Only show the n most recent values")]
        n: usize,
    },

    #[structopt(visible_alias = "remove", about = "Clear a container of all its contents [WARNING]")]
    Clear,
}

pub fn run() {
    // TODO: this should return a Result
    let opt = Opt::from_args();
    let mut cnt = container::create(ClipdFs, opt.container);

    let cmd = opt.cmd.unwrap_or(Command::Paste);
    match cmd {
        Command::Copy { value } => {
            cnt.add(opt.key, value.unwrap_or_else(|| {
                // Copy from system clipboard if there isn't already bytes waiting from stdin (piped input)
                if atty::is(atty::Stream::Stdin) {
                    return clipboard::create(X11).paste()
                }
                
                let mut buf = String::new();
                std::io::stdin().read_to_string(&mut buf).expect("couldn't read from stdin");
                buf
            }));
        },

        Command::Show { n } => {
            println!("{}", cnt.show(n))
        },

        Command::Paste => {
            println!("{}", cnt.get(opt.key).unwrap());
        },

        Command::Clear => {
            cnt.clear()
        } // TODO: add a warning
    };  
}
