use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "clipd", about = "A slightly smart clipboard.")]
pub struct Opt {
    #[structopt(subcommand)]
    pub cmd: Command,
}

#[derive(Debug, StructOpt)]
#[structopt(flatten)]
pub enum Command {
    #[structopt(visible_alias = "c", about = "Copy string into the clipboard")]
    Copy {
        
        #[structopt(short, long, default_value = "1", 
            help = "Optionally associate a custom key with the value copied to the clipboard. \
            The numbers {1, ..., n} are always used as keys for the last n items clipped, \
            with key = 1 being the most recent item.")]
        key: String,

        #[structopt(parse(from_str), 
            help = "Specifies the value to be copied to the clipboard")]
        value: String
    },

    #[structopt(visible_alias = "p", about = "Paste string from clipboard to stdout")]
    Paste {

        #[structopt(parse(from_str), default_value = "1", 
            help = "Use the associated key to retrieve items from the clipboard. \
            The numbers {1, ..., n} can be used for the last n items added, \
            with key = 1 being the most recent item.")]
        key: String,
    },
}