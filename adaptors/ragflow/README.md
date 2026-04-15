# RAGFlow Client

Rust client for the RAGFlow HTTP API. Provides methods for sending requests to corresponding API endpoints.

## Features / Endpoints

This client was built to support certain features required by the Crownshy team working on their Comhairle platform. There are no immediate plans to add modules for unsupported endpoints but contributions are welcome.

- ✅ Datasets
- ✅ Documents within datasets
- ✅ Chats
- ✅ Chat sessions
- ✅ Agents
- ✅ Agent sessions
- ❌️ Knowledge graphs
- ❌️ Chunks
- ❌️ Memory
- ❌️ Files
- ❌️ Search apps

## Installation

```toml
[dependencies]
ragflow-client = "0.0.1"
```

## Quick Start

```rust
use ragflow_client::{
    RagflowClient, dataset, GetQueryParams, Result
};

#[tokio::main]
async fn main() -> Result<()> {
    // Create a client
    let client = RagflowClient::new("https://ragflow-server.example", "<ragflow-api-key>");

    let params = GetQueryParams {
        id: Some("123".to_string()),
        ..Default::default()
    };

    let (status, datasets) = dataset::list(&client, Some(params)).await?;

    Ok(())
}
```

## Error Handling

The client uses custom `Result<T>` type with `RagflowError`.

## Requirements

- Tokio async runtime
