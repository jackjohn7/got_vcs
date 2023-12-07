mod args;
mod ops;

use clap::Parser;
use args::GotCommand;

fn main() {
    let args = args::GotArgs::parse();
    match args.got_command {
        GotCommand::Init => ops::init_ops::handle_init_command()
    }
}
