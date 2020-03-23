use super::models;
use super::models::RemoteRepo;
use anyhow::Result;
use reqwest::{blocking as req, StatusCode};
use serde::{Deserialize, Serialize};

fn patch<T: Serialize + ?Sized>(
    url: &str,
    body: &T,
    token: &str,
) -> Result<req::Response, reqwest::Error> {
    log::debug!("Patch: {}", url);
    let client = req::Client::new();
    client
        .patch(url)
        .bearer_auth(token)
        .header("User-Agent", "dadmin")
        .header("Accept", "application/vnd.github.v3+json")
        .json(body)
        .send()
}

fn put<T: Serialize + ?Sized>(
    url: &str,
    body: &T,
    token: &str,
) -> Result<req::Response, reqwest::Error> {
    log::debug!("PUT: {}", url);
    let client = req::Client::new();
    client
        .put(url)
        .bearer_auth(token)
        .header("User-Agent", "dadmin")
        .header("Accept", "application/vnd.github.luke-cage-preview+json")
        .json(body)
        .send()
}

fn post<T: Serialize + ?Sized>(
    url: &str,
    body: &T,
    token: &str,
) -> Result<req::Response, reqwest::Error> {
    log::debug!("POST: {}", url);
    let client = req::Client::new();
    client
        .post(url)
        .bearer_auth(token)
        .header("User-Agent", "dadmin")
        .header("Accept", "application/vnd.github.v3+json")
        .json(body)
        .send()
}

#[derive(Serialize, Debug)]
struct DefaultBranch {
    default_branch: String,
}

pub fn set_default_branch(repo: &RemoteRepo, branch: &str, token: &str) -> Result<()> {
    let url = format!("https://api.github.com/repos/{}/{}", repo.owner, repo.name);
    let body = DefaultBranch {
        default_branch: branch.to_string(),
    };
    let response = patch(&url, &body, token)?;
    let status = response.status();
    if status == StatusCode::UNAUTHORIZED {
        return Err(models::Unauthorized.into());
    }

    if !status.is_success() {
        return Err(models::Unsuccessful(status).into());
    }

    Ok(())
}

#[derive(Serialize, Debug)]
struct ProtectedBranch {
    required_status_checks: Option<RequiredStatusCheck>,
    enforce_admins: bool,
    required_pull_request_reviews: Option<RequiredPullRequestReviews>,
    restrictions: Option<Restrictions>,
    required_linear_history: bool,
    allow_force_pushes: bool,
    allow_deletions: bool,
}

#[derive(Serialize, Debug)]
struct RequiredStatusCheck {
    strict: bool,
    context: Vec<String>,
}

#[derive(Serialize, Debug)]
struct RequiredPullRequestReviews {
    dismiss_stale_reviews: bool,
    require_code_owner_reviews: bool,
    required_approving_review_count: i32,
}

#[derive(Serialize, Debug)]
struct Restrictions {
    users: Vec<String>,
    teams: Vec<String>,
    apps: Vec<String>,
}

pub fn set_protected_branch(repo: &RemoteRepo, branch: &str, token: &str) -> Result<()> {
    let url = format!(
        "https://api.github.com/repos/{}/{}/branches/{}/protection",
        repo.owner, repo.name, branch
    );
    let body = ProtectedBranch {
        required_status_checks: None,
        enforce_admins: true,
        required_pull_request_reviews: None,
        restrictions: None,
        required_linear_history: true,
        allow_force_pushes: false,
        allow_deletions: false,
    };

    log::debug!("Body {:?}", body);

    let response = put(&url, &body, token)?;
    log::debug!("Response: {:?}", response);

    let status = response.status();

    if status == StatusCode::UNAUTHORIZED {
        return Err(models::Unauthorized.into());
    }

    if !status.is_success() {
        return Err(models::Unsuccessful(status).into());
    }

    Ok(())
}

pub fn create_team(
    org: &str,
    team: &str,
    description: &str,
    maintainers: Vec<String>,
    is_secret: bool,
    token: &str,
) -> Result<CreateTeamResponse> {
    let url = format!("https://api.github.com/orgs/{}/teams", org);
    let privacy = if is_secret {
        "secret".to_string()
    } else {
        "closed".to_string()
    };
    let body = CreateTeamBody {
        name: team.to_string(),
        description: description.to_string(),
        maintainers,
        privacy,
    };
    log::debug!("Body {:?}", body);

    let response = post(&url, &body, token)?;

    let status = response.status();

    if status == StatusCode::UNAUTHORIZED {
        return Err(models::Unauthorized.into());
    }

    if !status.is_success() {
        return Err(models::Unsuccessful(status).into());
    }

    let response_body: CreateTeamResponse = response.json()?;
    Ok(response_body)
}

#[derive(Serialize, Debug)]
struct CreateTeamBody {
    name: String,
    description: String,
    maintainers: Vec<String>,
    privacy: String,
}

#[derive(Deserialize, Debug)]
pub struct CreateTeamResponse {
    id: i32,
    url: String,
}
