use crate::constants::{mcp as mcp_constants, errors, rpc_errors};
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

pub trait ToolHandler {
    fn call(&self, arguments: Option<HashMap<String, serde_json::Value>>) -> Result<CallToolResult>;
}

pub trait ResourceHandler {
    fn read(&self, uri: &str) -> Result<ReadResourceResult>;
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

            match self.handle_request(&line).await {
                Ok(response) => {
                    let response_json = serde_json::to_string(&response)?;
                    writer.write_all(response_json.as_bytes()).await?;
                    writer.write_all(b"\n").await?;
                    writer.flush().await?;
                }
                Err(e) => {
                    eprintln!("Error handling request: {}", e);
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
                Ok(McpResponse::Initialize { id, result })
            }
            McpRequest::ListTools { id } => {
                let result = ListToolsResult {
                    tools: self.tools.clone(),
                };
                Ok(McpResponse::ListTools { id, result })
            }
            McpRequest::CallTool { id, params } => {
                if let Some(handler) = self.tool_handlers.get(&params.name) {
                    match handler.call(params.arguments) {
                        Ok(result) => Ok(McpResponse::CallTool { id, result }),
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
                Ok(McpResponse::ListResources { id, result })
            }
            McpRequest::ReadResource { id, params } => {
                if let Some(handler) = self.resource_handlers.get(&params.uri) {
                    match handler.read(&params.uri) {
                        Ok(result) => Ok(McpResponse::ReadResource { id, result }),
                        Err(e) => Ok(utils::create_internal_error(id, &format!("{}: {}", errors::RESOURCE_READ_FAILED, e))),
                    }
                } else {
                    Ok(utils::create_method_not_found_error(id, &params.uri))
                }
            }
        }
    }
}