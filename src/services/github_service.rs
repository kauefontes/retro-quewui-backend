use anyhow::Result;
use reqwest::{Client, header};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GitHubUser {
    pub login: String,
    pub id: i64,
    pub avatar_url: String,
    pub html_url: String,
    pub name: Option<String>,
    pub company: Option<String>,
    pub blog: Option<String>,
    pub location: Option<String>,
    pub email: Option<String>,
    pub bio: Option<String>,
    pub twitter_username: Option<String>,
    pub public_repos: i32,
    pub public_gists: i32,
    pub followers: i32,
    pub following: i32,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GitHubRepo {
    pub id: i64,
    pub name: String,
    pub full_name: String,
    pub html_url: String,
    pub description: Option<String>,
    pub fork: bool,
    pub created_at: String,
    pub updated_at: String,
    pub pushed_at: String,
    pub homepage: Option<String>,
    pub size: i32,
    pub stargazers_count: i32,
    pub watchers_count: i32,
    pub language: Option<String>,
    pub forks_count: i32,
    pub open_issues_count: i32,
    pub topics: Option<Vec<String>>,
    pub visibility: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GitHubOrg {
    pub login: String,
    pub id: i64,
    pub avatar_url: String,
    pub description: Option<String>,
    pub html_url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GitHubActivity {
    pub id: String,
    pub r#type: String,
    pub actor: GitHubActor,
    pub repo: GitHubActivityRepo,
    pub payload: serde_json::Value,
    pub public: bool,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GitHubActor {
    pub id: i64,
    pub login: String,
    pub display_login: String,
    pub avatar_url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GitHubActivityRepo {
    pub id: i64,
    pub name: String,
    pub url: String,
}

pub struct GitHubService {
    client: Client,
    username: String,
}

impl GitHubService {
    pub fn new() -> Result<Self> {
        let github_token = env::var("GITHUB_TOKEN").ok();
        let github_username = env::var("GITHUB_USERNAME")
            .expect("GITHUB_USERNAME must be set in environment variables");
        
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::USER_AGENT,
            header::HeaderValue::from_static("retro-quewui-backend"),
        );
        
        if let Some(token) = github_token {
            headers.insert(
                header::AUTHORIZATION,
                header::HeaderValue::from_str(&format!("token {}", token))?,
            );
        }
        
        let client = Client::builder()
            .default_headers(headers)
            .build()?;
            
        Ok(Self {
            client,
            username: github_username,
        })
    }
    
    pub async fn get_user_profile(&self) -> Result<GitHubUser> {
        let url = format!("https://api.github.com/users/{}", self.username);
        let response = self.client.get(&url).send().await?;
        
        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "GitHub API error: {} - {}",
                response.status(),
                response.text().await?
            ));
        }
        
        let user = response.json::<GitHubUser>().await?;
        Ok(user)
    }
    
    pub async fn get_user_repos(&self, per_page: u32, page: u32) -> Result<Vec<GitHubRepo>> {
        let url = format!(
            "https://api.github.com/users/{}/repos?per_page={}&page={}&sort=updated",
            self.username, per_page, page
        );
        
        let response = self.client.get(&url).send().await?;
        
        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "GitHub API error: {} - {}",
                response.status(),
                response.text().await?
            ));
        }
        
        let repos = response.json::<Vec<GitHubRepo>>().await?;
        Ok(repos)
    }
    
    pub async fn get_user_organizations(&self) -> Result<Vec<GitHubOrg>> {
        let url = format!("https://api.github.com/users/{}/orgs", self.username);
        
        let response = self.client.get(&url).send().await?;
        
        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "GitHub API error: {} - {}",
                response.status(),
                response.text().await?
            ));
        }
        
        let orgs = response.json::<Vec<GitHubOrg>>().await?;
        Ok(orgs)
    }
    
    pub async fn get_user_activity(&self, per_page: u32) -> Result<Vec<GitHubActivity>> {
        let url = format!(
            "https://api.github.com/users/{}/events/public?per_page={}",
            self.username, per_page
        );
        
        let response = self.client.get(&url).send().await?;
        
        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "GitHub API error: {} - {}",
                response.status(),
                response.text().await?
            ));
        }
        
        let activities = response.json::<Vec<GitHubActivity>>().await?;
        Ok(activities)
    }
}
