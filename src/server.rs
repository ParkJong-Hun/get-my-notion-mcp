use crate::constants::{mcp as mcp_constants, errors};
use crate::mcp::*;
use crate::utils;
use anyhow::Result;
use std::collections::HashMap;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter};

pub struct McpServer {
    tools: Vec<Tool>,
    resources: Vec<Resource>,
    tool_handlers: HashMap<String, Box<dyn ToolHandler + Send + Sync>>,
    resource_handlers: HashMap<String, Box<dyn ResourceHandler + Send + Sync>>,
}

#[async_trait::async_trait]
pub trait ToolHandler: Send + Sync {
    async fn call(&self, arguments: Option<HashMap<String, serde_json::Value>>) -> Result<CallToolResult>;
}

#[async_trait::async_trait]
pub trait ResourceHandler: Send + Sync {
    async fn read(&self, uri: &str) -> Result<ReadResourceResult>;
}

impl McpServer {
    pub fn new() -> Self {
        Self {
            tools: Vec::new(),
            resources: Vec::new(),
            tool_handlers: HashMap::new(),
            resource_handlers: HashMap::new(),
        }
    }

    pub fn add_tool<H>(&mut self, tool: Tool, handler: H)
    where
        H: ToolHandler + Send + Sync + 'static,
    {
        self.tool_handlers.insert(tool.name.clone(), Box::new(handler));
        self.tools.push(tool);
    }

    pub fn add_resource<H>(&mut self, resource: Resource, handler: H)
    where
        H: ResourceHandler + Send + Sync + 'static,
    {
        self.resource_handlers.insert(resource.uri.clone(), Box::new(handler));
        self.resources.push(resource);
    }

    pub async fn run(&self) -> Result<()> {
        let stdin = tokio::io::stdin();
        let reader = BufReader::new(stdin);
        let stdout = tokio::io::stdout();
        let mut writer = BufWriter::new(stdout);

        let mut lines = reader.lines();

        while let Some(line) = lines.next_line().await? {
            if line.trim().is_empty() {
                continue;
            }

            // First try to handle as notification (no response needed)
            if let Ok(notification) = serde_json::from_str::<crate::mcp::McpNotification>(&line) {
                match notification {
                    McpNotification::Initialized => {
                        // Server is now ready to handle requests
                        eprintln!("Server initialized successfully");
                    }
                }
                continue;
            }
            
            // If not a notification, handle as a request
            match self.handle_request(&line).await {
                Ok(response) => {
                    let response_json = serde_json::to_string(&response)?;
                    writer.write_all(response_json.as_bytes()).await?;
                    writer.write_all(b"\n").await?;
                    writer.flush().await?;
                }
                Err(e) => {
                    // Try to parse the request to get the ID for error response
                    let error_response = if let Ok(value) = serde_json::from_str::<serde_json::Value>(&line) {
                        if let Some(id) = value.get("id") {
                            let request_id = if id.is_string() {
                                RequestId::String(id.as_str().unwrap_or("unknown").to_string())
                            } else if id.is_number() {
                                RequestId::Number(id.as_i64().unwrap_or(0))
                            } else {
                                RequestId::Null
                            };
                            utils::create_parse_error(request_id, &format!("Error handling request: {}", e))
                        } else {
                            utils::create_parse_error(RequestId::Null, &format!("Error handling request: {}", e))
                        }
                    } else {
                        utils::create_parse_error(RequestId::Null, &format!("Error handling request: {}", e))
                    };
                    
                    if let Ok(response_json) = serde_json::to_string(&error_response) {
                        let _ = writer.write_all(response_json.as_bytes()).await;
                        let _ = writer.write_all(b"\n").await;
                        let _ = writer.flush().await;
                    }
                }
            }
        }

        Ok(())
    }

    async fn handle_request(&self, request_str: &str) -> Result<McpResponse> {
        let request: McpRequest = serde_json::from_str(request_str)?;

        match request {
            McpRequest::Initialize { id, params: _ } => {
                let result = InitializeResult {
                    protocol_version: mcp_constants::PROTOCOL_VERSION.to_string(),
                    capabilities: utils::create_server_capabilities(),
                    server_info: utils::create_server_info(),
                };
                Ok(McpResponse::Initialize { 
                    jsonrpc: "2.0".to_string(),
                    id, 
                    result 
                })
            }
            McpRequest::ListTools { id } => {
                let result = ListToolsResult {
                    tools: self.tools.clone(),
                };
                Ok(McpResponse::ListTools { 
                    jsonrpc: "2.0".to_string(),
                    id, 
                    result 
                })
            }
            McpRequest::CallTool { id, params } => {
                if let Some(handler) = self.tool_handlers.get(&params.name) {
                    match handler.call(params.arguments).await {
                        Ok(result) => Ok(McpResponse::CallTool { 
                            jsonrpc: "2.0".to_string(),
                            id, 
                            result 
                        }),
                        Err(e) => Ok(utils::create_internal_error(id, &format!("{}: {}", errors::TOOL_EXECUTION_FAILED, e))),
                    }
                } else {
                    Ok(utils::create_method_not_found_error(id, &params.name))
                }
            }
            McpRequest::ListResources { id } => {
                let result = ListResourcesResult {
                    resources: self.resources.clone(),
                };
                Ok(McpResponse::ListResources { 
                    jsonrpc: "2.0".to_string(),
                    id, 
                    result 
                })
            }
            McpRequest::ReadResource { id, params } => {
                if let Some(handler) = self.resource_handlers.get(&params.uri) {
                    match handler.read(&params.uri).await {
                        Ok(result) => Ok(McpResponse::ReadResource { 
                            jsonrpc: "2.0".to_string(),
                            id, 
                            result 
                        }),
                        Err(e) => Ok(utils::create_internal_error(id, &format!("{}: {}", errors::RESOURCE_READ_FAILED, e))),
                    }
                } else {
                    Ok(utils::create_method_not_found_error(id, &params.uri))
                }
            }
        }
    }
}