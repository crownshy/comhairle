pub mod agent;
pub mod chat;
pub mod client;
pub mod dataset;
pub mod document;
pub mod error;

pub use error::RagflowError;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ConvoQuestion {
    pub question: String,
    pub stream: Option<bool>,
    pub session_id: Option<String>,
    pub user_id: Option<String>,
}

#[derive(Serialize, Default)]
pub struct GetQueryParams {
    pub page: Option<i32>,
    pub page_size: Option<i32>,
    pub orderby: Option<String>,
    pub desc: Option<bool>,
    pub name: Option<String>,
    pub id: Option<String>,
    pub title: Option<String>,
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
