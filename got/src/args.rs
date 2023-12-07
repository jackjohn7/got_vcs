use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct GotArgs {
    #[clap(subcommand)]
    pub got_command: GotCommand
}

#[derive(Subcommand, Debug)]
pub enum GotCommand {
    /// Initialize a Got repository
    Init
}
