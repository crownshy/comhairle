pub mod agent;
pub mod chat;
pub mod client;
pub mod dataset;
pub mod document;
pub mod error;

use std::collections::HashMap;

pub use error::RagflowError;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ConvoQuestion {
    pub question: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inputs: Option<HashMap<String, Input>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Input {
    pub r#type: String,
    pub value: String,
}

#[derive(Serialize, Default, Debug)]
pub struct GetQueryParams {
    pub page: Option<i32>,
    pub page_size: Option<i32>,
    pub orderby: Option<String>,
    pub desc: Option<bool>,
    pub name: Option<String>,
    pub id: Option<String>,
    pub title: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SessionMessage {
    pub content: String,
    pub id: Option<String>,
    pub role: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<Vec<MessageReference>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageReference {
    pub id: String,
    pub content: String,
    pub dataset_id: String,
    pub document_id: String,
    pub document_name: String,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum ChunkMethod {
    #[default]
    Naive,
    Manual,
    QA,
    Table,
    Paper,
    Book,
    Laws,
    Presentation,
    Picture,
    One,
    Email,
}

#[derive(Serialize)]
pub struct DeleteResources<'a> {
    pub ids: Vec<&'a str>,
}
