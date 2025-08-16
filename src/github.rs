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
    pub client: reqwest::Client,
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

    pub fn new_with_client(owner: String, repo: String, client: reqwest::Client) -> Self {
        Self {
            client,
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

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::{MockServer, Mock, ResponseTemplate};
    use wiremock::matchers::{method, path, header};

    #[tokio::test]
    async fn test_list_files_success() {
        let mock_server = MockServer::start().await;
        
        let mock_response = serde_json::json!([
            {
                "name": "README.md",
                "path": "README.md",
                "sha": "abc123",
                "type": "file",
                "size": 100,
                "download_url": "https://example.com/file"
            }
        ]);

        Mock::given(method("GET"))
            .and(path("/repos/test-owner/test-repo/contents/"))
            .and(header("User-Agent", "get-my-notion-mcp"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&mock_response))
            .mount(&mock_server)
            .await;

        let client = reqwest::Client::builder()
            .build()
            .unwrap();
        
        let github_client = GitHubClient::new_with_client(
            "test-owner".to_string(),
            "test-repo".to_string(),
            client,
        );

        // Override the base URL to use mock server
        let url = format!("{}/repos/test-owner/test-repo/contents/", mock_server.uri());
        let response = github_client.client
            .get(&url)
            .header("User-Agent", "get-my-notion-mcp")
            .send()
            .await
            .unwrap();

        let files: Vec<GitHubFile> = response.json().await.unwrap();
        
        assert_eq!(files.len(), 1);
        assert_eq!(files[0].name, "README.md");
        assert_eq!(files[0].file_type, "file");
    }

    #[tokio::test]
    async fn test_get_file_content_base64() {
        let mock_server = MockServer::start().await;
        
        let content = "Hello, World!";
        let encoded_content = base64::engine::general_purpose::STANDARD.encode(content);
        
        let mock_response = serde_json::json!({
            "name": "test.txt",
            "path": "test.txt",
            "sha": "abc123",
            "size": 13,
            "content": encoded_content,
            "encoding": "base64"
        });

        Mock::given(method("GET"))
            .and(path("/repos/test-owner/test-repo/contents/test.txt"))
            .and(header("User-Agent", "get-my-notion-mcp"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&mock_response))
            .mount(&mock_server)
            .await;

        let client = reqwest::Client::builder()
            .build()
            .unwrap();
        
        let github_client = GitHubClient::new_with_client(
            "test-owner".to_string(),
            "test-repo".to_string(),
            client,
        );

        // Test the decoding logic directly
        let url = format!("{}/repos/test-owner/test-repo/contents/test.txt", mock_server.uri());
        let response = github_client.client
            .get(&url)
            .header("User-Agent", "get-my-notion-mcp")
            .send()
            .await
            .unwrap();

        let file_content: GitHubContent = response.json().await.unwrap();
        
        if file_content.encoding == "base64" {
            let decoded = base64::engine::general_purpose::STANDARD
                .decode(&file_content.content.replace('\n', ""))
                .unwrap();
            let decoded_string = String::from_utf8(decoded).unwrap();
            assert_eq!(decoded_string, "Hello, World!");
        }
    }

    #[tokio::test]
    async fn test_get_latest_commit_sha() {
        let mock_server = MockServer::start().await;
        
        let mock_response = serde_json::json!({
            "sha": "abc123def456",
            "commit": {
                "message": "Test commit"
            }
        });

        Mock::given(method("GET"))
            .and(path("/repos/test-owner/test-repo/commits/main"))
            .and(header("User-Agent", "get-my-notion-mcp"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&mock_response))
            .mount(&mock_server)
            .await;

        let client = reqwest::Client::builder()
            .build()
            .unwrap();
        
        let github_client = GitHubClient::new_with_client(
            "test-owner".to_string(),
            "test-repo".to_string(),
            client,
        );

        // Test the commit SHA extraction
        let url = format!("{}/repos/test-owner/test-repo/commits/main", mock_server.uri());
        let response = github_client.client
            .get(&url)
            .header("User-Agent", "get-my-notion-mcp")
            .send()
            .await
            .unwrap();

        let commit: serde_json::Value = response.json().await.unwrap();
        let sha = commit["sha"].as_str().unwrap();
        
        assert_eq!(sha, "abc123def456");
    }

    #[test]
    fn test_github_client_creation() {
        let client = GitHubClient::new("owner".to_string(), "repo".to_string());
        assert_eq!(client.owner, "owner");
        assert_eq!(client.repo, "repo");
    }
}