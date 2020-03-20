use super::path::RootDirectory;
use crate::commands::{CloneArgs, CreateBranchArgs, DefaultBranchArgs, ListRepoArgs};
use crate::filter::Filter;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "dadmin", about = "git multirepo maintenance tool")]
pub struct Args {
    #[structopt(subcommand)]
    pub command: Commands,
}

#[derive(Debug, StructOpt)]
pub enum Commands {
    #[structopt(name = "init")]
    Init(InitArgs),
    #[structopt(name = "update-config")]
    Update(ConfigArgs),
    #[structopt(name = "lr", aliases = &["list-repos"])]
    ListRepos(ListRepoArgs),
    #[structopt(name = "cl", aliases = &["clone"])]
    CloneRepos(CloneArgs),
    #[structopt(name = "cb", aliases = &["create-branch"])]
    CreateBranch(CreateBranchArgs),
    #[structopt(name = "db", aliases = &["default-branch"])]
    DefaultBranch(DefaultBranchArgs),
}

#[derive(Debug, StructOpt)]
pub struct InitArgs {
    #[structopt(long, short, default_value)]
    pub root: RootDirectory,

    #[structopt(short, long)]
    pub token: String,
}

#[derive(Debug, StructOpt)]
pub struct ConfigArgs {
    #[structopt(long, short, default_value = "./dadmin")]
    pub root: String,
}

#[derive(Debug, StructOpt)]
pub struct GlobalArgs {
    #[structopt(long, short, default_value = "divvun")]
    pub organisation: String,
    #[structopt(long, short)]
    pub repositories: Option<Filter>,
}
