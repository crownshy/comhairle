use std::error::Error;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use axum::extract::{Request, State};
use axum::http::{Method, StatusCode};
use axum::{Json, Router};
use serde_json::{Value, json};
use tokio::net::TcpListener;
use tokio::sync::{Notify, RwLock};
use tokio::task::JoinHandle;

use super::{
    ComhairleBotService, ComhairleRagBotService, GetQueryParams, UpdateDocumentRequest,
    UploadFileRequest,
};

struct FakeBot {
    state: Arc<FakeBotState>,
    server: JoinHandle<std::io::Result<()>>,
}

struct FakeBotState {
    list_calls: AtomicUsize,
    response: RwLock<(StatusCode, Value)>,
    pause: RwLock<Option<Arc<ListPause>>>,
}

#[derive(Default)]
struct ListPause {
    entered: Notify,
    resume: Notify,
}

impl Drop for FakeBot {
    fn drop(&mut self) {
        self.server.abort();
    }
}

fn document(status: &str, progress: f64) -> Value {
    json!({
        "id": "document-1", "name": "document.pdf", "size": 42,
        "run": status, "progress": progress
    })
}

fn document_list(documents: Vec<Value>) -> Value {
    json!({"code": 0, "data": {"docs": documents}})
}

async fn respond(
    State(state): State<Arc<FakeBotState>>,
    request: Request,
) -> (StatusCode, Json<Value>) {
    if request.method() == Method::GET {
        state.list_calls.fetch_add(1, Ordering::SeqCst);
        let (status, response) = state.response.read().await.clone();
        let pause = state.pause.write().await.take();
        if let Some(pause) = pause {
            pause.entered.notify_one();
            pause.resume.notified().await;
        }
        return (status, Json(response));
    }
    if request.method() == Method::POST && request.uri().path().ends_with("/documents") {
        return (
            StatusCode::OK,
            Json(json!({"code": 0, "data": [document("RUNNING", 0.0)]})),
        );
    }
    (StatusCode::OK, Json(json!({"code": 0, "data": null})))
}

impl FakeBot {
    async fn start() -> Result<(Self, ComhairleRagBotService), Box<dyn Error>> {
        let state = Arc::new(FakeBotState {
            list_calls: AtomicUsize::new(0),
            response: RwLock::new((StatusCode::OK, document_list(vec![document("DONE", 1.0)]))),
            pause: RwLock::new(None),
        });
        let listener = TcpListener::bind("127.0.0.1:0").await?;
        let address = listener.local_addr()?;
        let app = Router::new().fallback(respond).with_state(state.clone());
        let server = tokio::spawn(async move { axum::serve(listener, app).await });
        let service = ComhairleRagBotService::new(&format!("http://{address}"), "test-key");
        Ok((Self { state, server }, service))
    }

    fn list_calls(&self) -> usize {
        self.state.list_calls.load(Ordering::SeqCst)
    }
}

#[tokio::test]
async fn parsed_lists_are_cached_and_concurrent_misses_share_one_fetch()
-> Result<(), Box<dyn Error>> {
    let (bot, service) = FakeBot::start().await?;
    let results = futures::future::join_all(
        (0..20).map(|_| service.list_documents("knowledge-base", Some(GetQueryParams::default()))),
    )
    .await;
    for result in results {
        let (status, documents) = result?;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(documents.len(), 1);
        assert_eq!(documents[0].parse_progress, 1.0);
    }
    assert_eq!(bot.list_calls(), 1);
    Ok(())
}

#[tokio::test]
async fn incomplete_and_mixed_lists_are_not_cached() -> Result<(), Box<dyn Error>> {
    let (bot, service) = FakeBot::start().await?;
    let cases = [
        vec![document("DONE", 0.999)],
        vec![document("RUNNING", 1.0)],
        vec![document("FAILED", 1.0)],
        vec![document("DONE", 0.0)],
        vec![document("DONE", 1.0), document("RUNNING", 0.5)],
    ];
    for (index, documents) in cases.into_iter().enumerate() {
        let knowledge_base = format!("knowledge-base-{index}");
        *bot.state.response.write().await = (StatusCode::OK, document_list(documents.clone()));
        let (_, actual) = service.list_documents(&knowledge_base, None).await?;
        assert_eq!(actual.len(), documents.len());
        *bot.state.response.write().await =
            (StatusCode::OK, document_list(vec![document("DONE", 1.0)]));
        let (_, actual) = service.list_documents(&knowledge_base, None).await?;
        assert_eq!(actual[0].parse_status, "DONE");
        assert_eq!(actual[0].parse_progress, 1.0);
        assert_eq!(bot.list_calls(), (index + 1) * 2);
    }
    Ok(())
}

#[tokio::test]
async fn in_flight_list_cannot_restore_results_after_invalidation() -> Result<(), Box<dyn Error>> {
    let (bot, service) = FakeBot::start().await?;
    let service = Arc::new(service);
    let pause = Arc::new(ListPause::default());
    *bot.state.pause.write().await = Some(pause.clone());
    let listing = tokio::spawn({
        let service = service.clone();
        async move { service.list_documents("changed", None).await }
    });
    pause.entered.notified().await;

    let mutation_started = Arc::new(Notify::new());
    let deleting = tokio::spawn({
        let service = service.clone();
        let mutation_started = mutation_started.clone();
        async move {
            mutation_started.notify_one();
            service
                .delete_document("document-1".into(), "changed".into())
                .await
        }
    });
    mutation_started.notified().await;
    pause.resume.notify_one();
    listing.await??;
    deleting.await??;

    *bot.state.response.write().await = (StatusCode::OK, document_list(vec![]));
    let (_, documents) = service.list_documents("changed", None).await?;
    assert!(documents.is_empty());
    assert_eq!(bot.list_calls(), 2);
    Ok(())
}

#[tokio::test]
async fn failures_are_not_cached() -> Result<(), Box<dyn Error>> {
    let (bot, service) = FakeBot::start().await?;
    for (index, (status, response)) in [
        (StatusCode::BAD_GATEWAY, json!({"message": "unavailable"})),
        (
            StatusCode::OK,
            json!({"code": 102, "message": "bot failure"}),
        ),
    ]
    .into_iter()
    .enumerate()
    {
        let knowledge_base = format!("knowledge-base-{index}");
        *bot.state.response.write().await = (status, response);
        assert!(service.list_documents(&knowledge_base, None).await.is_err());
        *bot.state.response.write().await =
            (StatusCode::OK, document_list(vec![document("DONE", 1.0)]));
        service.list_documents(&knowledge_base, None).await?;
        assert_eq!(bot.list_calls(), (index + 1) * 2);
    }
    Ok(())
}

#[tokio::test]
async fn cache_keys_include_knowledge_base_and_all_query_fields() -> Result<(), Box<dyn Error>> {
    let (bot, service) = FakeBot::start().await?;
    let queries = vec![
        None,
        Some(GetQueryParams::default()),
        Some(GetQueryParams {
            page: Some(2),
            ..Default::default()
        }),
        Some(GetQueryParams {
            page_size: Some(10),
            ..Default::default()
        }),
        Some(GetQueryParams {
            order_by: Some("name".into()),
            ..Default::default()
        }),
        Some(GetQueryParams {
            name: Some("document.pdf".into()),
            ..Default::default()
        }),
        Some(GetQueryParams {
            title: Some("title".into()),
            ..Default::default()
        }),
    ];
    for knowledge_base in ["first", "second"] {
        for query in &queries {
            service
                .list_documents(knowledge_base, query.clone())
                .await?;
            service
                .list_documents(knowledge_base, query.clone())
                .await?;
        }
    }
    assert_eq!(bot.list_calls(), queries.len() * 2);
    Ok(())
}

#[tokio::test]
async fn every_mutation_invalidates_all_queries_for_only_its_knowledge_base()
-> Result<(), Box<dyn Error>> {
    let (bot, service) = FakeBot::start().await?;
    for mutation in 0..6 {
        service.list_documents("changed", None).await?;
        service
            .list_documents("changed", Some(GetQueryParams::default()))
            .await?;
        service.list_documents("unchanged", None).await?;
        match mutation {
            0 => {
                service
                    .upload_document(
                        "changed",
                        UploadFileRequest {
                            filename: "document.pdf".into(),
                            bytes: vec![1, 2, 3],
                        },
                    )
                    .await?;
            }
            1 => {
                service
                    .delete_document("document-1".into(), "changed".into())
                    .await?;
            }
            2 => {
                service
                    .parse_document("document-1".into(), "changed".into())
                    .await?;
            }
            3 => {
                service
                    .stop_parsing_document("document-1".into(), "changed".into())
                    .await?;
            }
            4 => {
                service
                    .update_document(
                        "document-1",
                        "changed",
                        UpdateDocumentRequest {
                            name: Some("renamed.pdf".into()),
                        },
                    )
                    .await?;
            }
            _ => {
                service.delete_knowledge_base("changed").await?;
            }
        }
        let calls = bot.list_calls();
        service.list_documents("changed", None).await?;
        service
            .list_documents("changed", Some(GetQueryParams::default()))
            .await?;
        service.list_documents("unchanged", None).await?;
        assert_eq!(bot.list_calls(), calls + 2);
    }
    Ok(())
}
