use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use base64::Engine;

#[derive(Debug, Serialize, Deserialize)]
pub struct GitHubFile {
    pub name: String,
    pub path: String,
    pub sha: String,
    #[serde(rename = "type")]
    pub file_type: String,
    pub size: Option<u64>,
    pub download_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GitHubContent {
    pub name: String,
    pub path: String,
    pub sha: String,
    pub size: u64,
    pub content: String,
    pub encoding: String,
}

pub struct GitHubClient {
    client: reqwest::Client,
    owner: String,
    repo: String,
}

impl GitHubClient {
    pub fn new(owner: String, repo: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            owner,
            repo,
        }
    }

    pub async fn list_files(&self, path: Option<&str>) -> Result<Vec<GitHubFile>> {
        let path = path.unwrap_or("");
        let url = format!(
            "https://api.github.com/repos/{}/{}/contents/{}",
            self.owner, self.repo, path
        );

        let response = self
            .client
            .get(&url)
            .header("User-Agent", "get-my-notion-mcp")
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow!(
                "GitHub API request failed: {}",
                response.status()
            ));
        }

        let files: Vec<GitHubFile> = response.json().await?;
        Ok(files)
    }

    pub async fn get_file_content(&self, path: &str) -> Result<String> {
        let url = format!(
            "https://api.github.com/repos/{}/{}/contents/{}",
            self.owner, self.repo, path
        );

        let response = self
            .client
            .get(&url)
            .header("User-Agent", "get-my-notion-mcp")
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow!(
                "GitHub API request failed: {}",
                response.status()
            ));
        }

        let content: GitHubContent = response.json().await?;
        
        if content.encoding == "base64" {
            let decoded = base64::engine::general_purpose::STANDARD
                .decode(&content.content.replace('\n', ""))?;
            Ok(String::from_utf8(decoded)?)
        } else {
            Ok(content.content)
        }
    }

    pub async fn get_latest_commit_sha(&self) -> Result<String> {
        let url = format!(
            "https://api.github.com/repos/{}/{}/commits/main",
            self.owner, self.repo
        );

        let response = self
            .client
            .get(&url)
            .header("User-Agent", "get-my-notion-mcp")
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow!(
                "GitHub API request failed: {}",
                response.status()
            ));
        }

        let commit: serde_json::Value = response.json().await?;
        let sha = commit["sha"]
            .as_str()
            .ok_or_else(|| anyhow!("Could not extract commit SHA"))?;
        
        Ok(sha.to_string())
    }
}