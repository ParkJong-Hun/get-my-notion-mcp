use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RequestId {
    String(String),
    Number(i64),
    Null,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tool {
    pub name: String,
    pub description: String,
    pub input_schema: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resource {
    pub uri: String,
    pub name: String,
    pub description: Option<String>,
    pub mime_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "method")]
pub enum McpRequest {
    #[serde(rename = "initialize")]
    Initialize {
        id: RequestId,
        params: InitializeParams,
    },
    #[serde(rename = "tools/list")]
    ListTools { id: RequestId },
    #[serde(rename = "tools/call")]
    CallTool {
        id: RequestId,
        params: CallToolParams,
    },
    #[serde(rename = "resources/list")]
    ListResources { id: RequestId },
    #[serde(rename = "resources/read")]
    ReadResource {
        id: RequestId,
        params: ReadResourceParams,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitializeParams {
    pub protocol_version: String,
    pub capabilities: ClientCapabilities,
    pub client_info: ClientInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientCapabilities {
    pub roots: Option<RootsCapability>,
    pub sampling: Option<SamplingCapability>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RootsCapability {
    pub list_changed: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SamplingCapability {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientInfo {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallToolParams {
    pub name: String,
    pub arguments: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadResourceParams {
    pub uri: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "method")]
pub enum McpResponse {
    #[serde(rename = "initialize")]
    Initialize {
        id: RequestId,
        result: InitializeResult,
    },
    #[serde(rename = "tools/list")]
    ListTools {
        id: RequestId,
        result: ListToolsResult,
    },
    #[serde(rename = "tools/call")]
    CallTool {
        id: RequestId,
        result: CallToolResult,
    },
    #[serde(rename = "resources/list")]
    ListResources {
        id: RequestId,
        result: ListResourcesResult,
    },
    #[serde(rename = "resources/read")]
    ReadResource {
        id: RequestId,
        result: ReadResourceResult,
    },
    Error {
        id: RequestId,
        error: McpError,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitializeResult {
    pub protocol_version: String,
    pub capabilities: ServerCapabilities,
    pub server_info: ServerInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerCapabilities {
    pub tools: Option<ToolsCapability>,
    pub resources: Option<ResourcesCapability>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolsCapability {
    pub list_changed: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourcesCapability {
    pub subscribe: Option<bool>,
    pub list_changed: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerInfo {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListToolsResult {
    pub tools: Vec<Tool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallToolResult {
    pub content: Vec<ToolContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ToolContent {
    #[serde(rename = "text")]
    Text { text: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListResourcesResult {
    pub resources: Vec<Resource>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadResourceResult {
    pub contents: Vec<ResourceContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ResourceContent {
    #[serde(rename = "text")]
    Text { uri: String, text: String },
    #[serde(rename = "blob")]
    Blob {
        uri: String,
        blob: String, // base64 encoded
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpError {
    pub code: i32,
    pub message: String,
    pub data: Option<serde_json::Value>,
}