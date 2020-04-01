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
        Commands::Init(args) => args.save_config(),
        Commands::CloneRepos(clone_args) => clone_args.clone(),
        Commands::Add(args) => args.run(),
        Commands::RemoveUsers(args) => args.remove_users(),
        Commands::Set(args) => args.run(),
        Commands::Show(args) => args.show(),
        Commands::Create(args) => args.do_create(),
        Commands::Branch(args) => args.run(),
    }
}
