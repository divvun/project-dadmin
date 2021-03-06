use super::common;
use crate::filter::Filter;
use crate::github;
use anyhow::Result;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
/// Set topics for all repositories that match a regex
pub struct TopicSetArgs {
    #[structopt(long, short)]
    /// Target organisation name
    ///
    /// You can set a default organisation in the init or set organisation command.
    pub organisation: Option<String>,
    #[structopt(long, short)]
    /// Optional regex to filter repositories
    pub regex: Option<Filter>,
    #[structopt(long, short)]
    /// All topics will be set
    pub topics: Vec<String>,
}

impl TopicSetArgs {
    pub fn run(&self) -> Result<()> {
        let user_token = common::user_token()?;
        let organisation = common::organisation(self.organisation.as_deref())?;

        let filtered_repos =
            common::query_and_filter_repositories(&organisation, self.regex.as_ref(), &user_token)?;

        if filtered_repos.is_empty() {
            println!(
                "There is no repositories in organisation {} that matches pattern {:?}",
                &organisation, self.regex
            );
            return Ok(());
        }

        for repo in filtered_repos {
            let result = github::set_topics(&repo, &self.topics, &user_token);
            match result {
                Ok(topics) => {
                    println!("Set topics for repo {} successfully", repo.name);
                    println!("List of topics for {} is: {:?}", repo.name, topics);
                }
                Err(e) => println!(
                    "Failed to set topics for repo {} because {:?}",
                    repo.name, e
                ),
            }
        }
        Ok(())
    }
}
