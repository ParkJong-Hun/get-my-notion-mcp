use crate::constants::{github, mcp};
use crate::mcp::*;

// Common URL builders for GitHub API
pub fn build_github_contents_url(owner: &str, repo: &str, path: &str) -> String {
    format!("{}/repos/{}/{}/contents/{}", github::API_BASE_URL, owner, repo, path)
}

pub fn build_github_commits_url(owner: &str, repo: &str, branch: &str) -> String {
    format!("{}/repos/{}/{}/commits/{}", github::API_BASE_URL, owner, repo, branch)
}

// Repository info formatter
pub fn format_repository_info(owner: &str, repo: &str, latest_commit: &str) -> String {
    format!(
        "Repository: {}/{}\nLatest commit: {}\nAccess via: https://github.com/{}/{}",
        owner, repo, latest_commit, owner, repo
    )
}

// File info formatter for list_files output
pub fn format_file_info(files: &[crate::github::GitHubFile]) -> String {
    let mut content = String::new();
    content.push_str("Files in repository:\n\n");
    
    for file in files {
        content.push_str(&format!(
            "- **{}** ({})\n  Path: {}\n  Type: {}\n",
            file.name, 
            truncate_sha(&file.sha),
            file.path,
            file.file_type
        ));
        if let Some(size) = file.size {
            content.push_str(&format!("  Size: {} bytes\n", size));
        }
        content.push('\n');
    }
    
    content
}

// File content formatter
pub fn format_file_content(path: &str, content: &str) -> String {
    format!("Content of file: {}\n\n```\n{}\n```", path, content)
}

// SHA truncation utility
pub fn truncate_sha(sha: &str) -> String {
    if sha.len() >= 7 {
        sha[..7].to_string()
    } else {
        sha.to_string()
    }
}

// Tool creation helpers
pub fn create_list_files_tool() -> Tool {
    Tool {
        name: mcp::TOOL_LIST_FILES.to_string(),
        description: "List files in the my-notion GitHub repository".to_string(),
        input_schema: crate::constants::schemas::list_files_schema(),
    }
}

pub fn create_get_file_content_tool() -> Tool {
    Tool {
        name: mcp::TOOL_GET_FILE_CONTENT.to_string(),
        description: "Get content of a specific file from the my-notion repository".to_string(),
        input_schema: crate::constants::schemas::get_file_content_schema(),
    }
}

pub fn create_get_latest_commit_tool() -> Tool {
    Tool {
        name: mcp::TOOL_GET_LATEST_COMMIT.to_string(),
        description: "Get the latest commit SHA from the my-notion repository".to_string(),
        input_schema: crate::constants::schemas::get_latest_commit_schema(),
    }
}

// Resource creation helper
pub fn create_repo_info_resource() -> Resource {
    Resource {
        uri: mcp::RESOURCE_REPO_INFO.to_string(),
        name: "My Notion Repository Info".to_string(),
        description: Some("Information about the my-notion GitHub repository".to_string()),
        mime_type: Some(crate::constants::mime_types::TEXT_PLAIN.to_string()),
    }
}

// Server capabilities creation
pub fn create_server_capabilities() -> ServerCapabilities {
    ServerCapabilities {
        tools: Some(ToolsCapability {
            list_changed: Some(false),
        }),
        resources: Some(ResourcesCapability {
            subscribe: Some(false),
            list_changed: Some(false),
        }),
    }
}

// Server info creation
pub fn create_server_info() -> ServerInfo {
    ServerInfo {
        name: mcp::SERVER_NAME.to_string(),
        version: mcp::SERVER_VERSION.to_string(),
    }
}

// Common error creation helpers
pub fn create_method_not_found_error(id: String, method_name: &str) -> McpResponse {
    McpResponse::Error {
        id,
        error: McpError {
            code: crate::constants::rpc_errors::METHOD_NOT_FOUND,
            message: format!("{}: '{}'", crate::constants::errors::TOOL_NOT_FOUND, method_name),
            data: None,
        },
    }
}

pub fn create_internal_error(id: String, message: &str) -> McpResponse {
    McpResponse::Error {
        id,
        error: McpError {
            code: crate::constants::rpc_errors::INTERNAL_ERROR,
            message: message.to_string(),
            data: None,
        },
    }
}

// HTTP request header helper
pub fn get_user_agent_header() -> (&'static str, &'static str) {
    ("User-Agent", github::USER_AGENT)
}