use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct CreateDataset {
    pub name: String,
    pub description: Option<String>,
    pub permission: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct CreateDatasetResponse {
    pub code: i32,
    pub data: Dataset,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Dataset {
    pub avatar: Option<String>,
    pub chunk_count: i32,
    pub chunk_method: ChunkMethod,
    pub create_date: String,
    pub create_time: i64,
    pub created_by: String,
    pub description: Option<String>,
    pub document_count: i32,
    pub embedding_model: String,
    pub id: String,
    pub language: String,
    pub name: String,
    pub pagerank: i32,
    pub permission: String,
    pub similarity_threshold: f64,
    pub status: String,
    pub tenant_id: String,
    pub token_num: i32,
    pub update_date: String,
    pub update_time: i64,
    pub vector_similarity_weight: f64,
}

#[derive(Serialize, Deserialize)]
pub struct GetDocumentsResponse {
    pub code: i32,
    pub data: DocumentList,
}

#[derive(Serialize, Deserialize)]
pub struct DocumentList {
    pub docs: Vec<Document>,
    pub total: Option<i32>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Document {
    pub chunk_count: i32,
    pub create_date: String,
    pub create_time: i64,
    pub created_by: String,
    pub knowledgebase_id: String,
    pub location: String,
    pub name: String,
    pub chunk_method: ChunkMethod,
    pub process_begin_at: Option<String>,
    pub process_duration: f64,
    pub progress: f64,
    pub progress_message: Option<String>,
    pub run: String,
    pub size: i64,
    pub source_type: String,
    pub status: i32,
    pub suffix: Option<String>,
    pub thumbnail: Option<String>,
    pub token_count: i32,
    pub r#type: Option<String>,
    pub update_date: String,
    pub update_time: i64,
}

#[derive(Serialize, Default)]
pub struct GetQueryParams {
    pub page: Option<i32>,
    pub page_size: Option<i32>,
    pub orderby: Option<String>,
    pub desc: Option<bool>,
    pub name: Option<String>,
    pub id: Option<String>,
}

#[derive(Serialize)]
pub struct UploadFile {
    pub filename: String,
    pub bytes: Vec<u8>,
}

#[derive(Serialize)]
pub struct UpdateDocument {
    pub name: Option<String>,
    pub chunk_method: Option<ChunkMethod>,
    pub parser_config: Option<ParserConfig>,
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
pub enum ParserConfig {
    Naive(NaiveParserConfig),
    RaptorOnly(RaptorOnlyParserConfig),
    Empty(EmptyParserConfig),
}

#[derive(Serialize)]
pub struct NaiveParserConfig {
    chunk_token_num: Option<u32>,
    layout_recognize: Option<bool>,
    html4excel: Option<bool>,
    delimiter: Option<String>,
    task_page_size: Option<u32>,
    raptor: Option<RaptorSettings>,
}

#[derive(Serialize)]
pub struct RaptorOnlyParserConfig {
    raptor: RaptorSettings,
}

#[derive(Serialize)]
pub struct EmptyParserConfig;

#[derive(Serialize)]
pub struct RaptorSettings {
    use_raptor: bool,
}

#[derive(Serialize)]
pub struct DeleteResources<'a> {
    pub ids: Vec<&'a str>,
}

#[derive(Serialize)]
pub struct ParseDocuments<'a> {
    pub document_ids: Vec<&'a str>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct CreateChat {
    pub name: String,
    pub avatar: Option<String>,
    pub dataset_ids: Vec<String>,
    pub llm: Llm,
    pub prompt: Prompt,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Llm {
    pub model_name: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Prompt {
    pub opener: Option<String>,
    pub empty_response: Option<String>,
    pub prompt: Option<String>,
    pub keywords_similarity_weight: Option<f64>,
    pub rerank_model: Option<String>,
    pub similarity_threshold: Option<f64>,
    pub top_n: i32,
    pub variables: Option<Vec<Variable>>,
}

#[derive(Deserialize)]
pub struct CreateChatResponse {
    code: i32,
    pub data: Chat,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Chat {
    pub name: String,
    pub avatar: Option<String>,
    pub create_date: String,
    pub create_time: i64,
    pub dataset_ids: Vec<String>,
    pub description: Option<String>,
    pub do_refer: Option<String>,
    pub id: String,
    pub language: Option<String>,
    pub llm: Llm,
    pub prompt: Prompt,
}

#[derive(Serialize, Default)]
pub struct UpdateChat {
    pub name: String,
    pub avatar: String,
    pub dataset_ids: Vec<String>,
    pub llm: Llm,
    pub prompt: Prompt,
}

#[derive(Deserialize)]
pub struct GetChatResponse {
    code: i32,
    pub data: Vec<Chat>,
}

#[derive(Serialize, Deserialize)]
pub struct Variable {
    key: String,
    optional: bool,
}

#[derive(Serialize, Default)]
pub struct CreateUpdateChatSession {
    pub name: Option<String>,
    pub user_id: Option<String>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct ChatSession {
    pub chat_id: String,
    pub create_date: String,
    pub create_time: i64,
    pub id: String,
    pub name: Option<String>,
    pub update_date: String,
    pub update_time: i64,
    pub messages: Vec<ChatSessionMessage>,
}

#[derive(Serialize, Deserialize)]
pub struct ChatSessionMessage {
    content: String,
    role: String,
}

#[derive(Deserialize)]
pub struct CreateChatSessionResponse {
    code: i32,
    pub data: ChatSession,
}

#[derive(Deserialize)]
pub struct GetChatSessionResponse {
    code: i32,
    pub data: Vec<ChatSession>,
}
