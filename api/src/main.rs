use apalis::prelude::{MemoryStorage, Monitor, WorkerBuilder, WorkerFactoryFn};
use apalis_redis::RedisStorage;
use aws_config::BehaviorVersion;
use comhairle::{
    bot_service::{ComhairleBotService, ComhairleRagBotService},
    bulk_storage::s3_storage::S3StorageService,
    config::TranslatorConfig,
    db::setup_db,
    mailer::Mailer,
    setup_server,
    translation_service::GoogleTranslateService,
    websockets::ComhairleWebSocketService,
    wiki_poll_service::polis_service::PolisClient,
    workers::{process_documents::process_document_handler, JobQueues},
    ComhairleState,
};
use std::{error::Error, sync::Arc};
use tokio::sync::Mutex;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load .env files
    //
    dotenvy::dotenv().ok();

    // initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                "debug,sqlx=debug,tower_http=info,axum::rejection=trace".into()
            }),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .with_file(true)
                .with_line_number(true)
                .with_thread_ids(true)
                .with_thread_names(true)
                .with_target(true)
                .with_span_events(tracing_subscriber::fmt::format::FmtSpan::CLOSE)
                .pretty(),
        )
        .init();

    // Load Config
    let config = comhairle::config::load()?;

    // Setup DB
    let db = setup_db(&config.database_url).await?;

    // Setup Mailer
    let mailer = Arc::new(Mailer::new(
        &config.mailer.host,
        &config.mailer.user,
        &config.mailer.password,
    ));

    // Setup Translation Service
    //
    let translation_service =
        config
            .translator
            .as_ref()
            .map(|TranslatorConfig::Google(google_config)| {
                Arc::new(GoogleTranslateService::new(
                    google_config.api_key.to_owned(),
                )) as Arc<dyn comhairle::translation_service::TranslationService>
            });

    // Setup Bulk Storage Service
    //
    let s3_config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let bulk_storage_service = S3StorageService::new(&s3_config, "comhairle".to_owned());

    // Setup Websocket service
    let websockets = Arc::new(ComhairleWebSocketService::new());

    // Setup bot service

    let bot_service = match (
        &config.bot_service_host,
        &config.bot_service_api_key,
        &config.default_knowledge_base_id,
        &config.elicitation_bot_agent_id,
    ) {
        (Some(host), Some(api_key), Some(_), Some(_)) => {
            Some(Arc::new(ComhairleRagBotService::new(host, api_key))
                as Arc<dyn ComhairleBotService>)
        }
        _ => None,
    };

    let wiki_poll_service = Arc::new(PolisClient::new(&config.polis_url));

    let process_documents_storage = MemoryStorage::new();
    let redis_connection = apalis_redis::connect(config.workers.redis_url.clone())
        .await
        .expect("Could not connect to redis");
    let process_transcriptions_storage = RedisStorage::new(redis_connection);
    let jobs = Arc::new(JobQueues {
        process_documents: Arc::new(Mutex::new(process_documents_storage.clone())),
        process_transcriptions: Arc::new(Mutex::new(process_transcriptions_storage.clone())),
    });

    let state = Arc::new(ComhairleState {
        db,
        mailer,
        config,
        websockets,
        translation_service,
        bot_service,
        wiki_poll_service,
        jobs,
        bulk_storage_service: Arc::new(bulk_storage_service),
    });

    let app = setup_server(state.clone()).await?;

    let server_future = async move {
        // run our app with hyper
        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
        tracing::info!("listening on {}", listener.local_addr().unwrap());
        axum::serve(listener, app).await.unwrap();
    };

    let process_document_worker = WorkerBuilder::new("process_document_job")
        .data(state.clone())
        .backend(process_documents_storage.clone())
        .build_fn(process_document_handler);

    let worker_future = { Monitor::new().register(process_document_worker).run() };

    let _ = tokio::join!(server_future, worker_future);

    Ok(())
}
