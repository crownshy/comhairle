use serde::Serialize;

#[derive(Serialize, Default)]
pub struct GetDocumentsQueryParams {
    pub page: Option<i32>,
    pub page_size: Option<i32>,
    pub orderby: Option<String>,
    pub desc: Option<bool>,
    pub id: Option<String>,
    pub create_time_from: Option<i32>,
    // keywords: Option<String>, // TODO: find way to implement these
    // suffix: Option<Vec<String>>,
    // run: Option<Vec<String>>,
}

#[derive(Serialize)]
pub struct UploadFile {
    pub filename: String,
    pub bytes: Vec<u8>,
}

#[derive(Serialize)]
pub struct UpdateDocument {
    pub name: Option<String>,
    pub enabled: Option<i32>, // TODO: limit to 1 or 0
    pub chunk_method: Option<ChunkMethod>,
    pub meta_fields: Option<()>, // TODO: find fields in docs
    pub parser_config: Option<ParserConfig>,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ChunkMethod {
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
pub struct DeleteDocument<'a> {
    pub ids: Vec<&'a str>,
}
