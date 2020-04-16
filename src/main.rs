mod cli;
mod commands;
mod config;
mod convert;
mod filter;
mod git;
mod github;
mod path;
mod toml;
mod user;

use anyhow::Result;
use cli::{Args, Commands};
use structopt::StructOpt;

fn main() -> Result<()> {
    color_backtrace::install();

    pretty_env_logger::formatted_timed_builder()
        .filter(None, log::LevelFilter::Info)
        .filter(Some("dadmin"), log::LevelFilter::Debug)
        .init();

    let args = Args::from_args();
    log::debug!("Arguments: {:?}", args);

    match args.command {
        Commands::Add(args) => args.run(),
        Commands::Apply(args) => args.run(),
        Commands::Branch(args) => args.run(),
        Commands::Checkout(args) => args.run(),
        Commands::Clone(args) => args.run(),
        Commands::Clean(args) => args.run(),
        Commands::Commit(args) => args.run(),
        Commands::Create(args) => args.run(),
        Commands::Init(args) => args.save_config(),
        Commands::Merge(args) => args.run(),
        Commands::Push(args) => args.run(),
        Commands::Remove(args) => args.run(),
        Commands::Set(args) => args.run(),
        Commands::Show(args) => args.run(),
        Commands::Status(args) => args.run(),
    }
}
