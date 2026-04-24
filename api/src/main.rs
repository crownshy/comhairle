use aws_config::BehaviorVersion;
use comhairle::{
    bot_service::{ComhairleBotService, ComhairleRagBotService},
    bulk_storage::{s3_storage::S3StorageService, BulkStorageService},
    categorization_service::{tttc_categorizer::TttcCategorizer, CategorizationService},
    config::{TranscriptionServiceConfig, TranslatorConfig},
    db::setup_db,
    mailer::Mailer,
    setup_server,
    transcription_service::amazon_transcriber::AmazonTranscriber,
    translation_service::GoogleTranslateService,
    websockets::ComhairleWebSocketService,
    wiki_poll_service::polis_service::PolisClient,
    worker_service::{init_monitor, init_worker_service},
    ComhairleState,
};
use std::{error::Error, sync::Arc};
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
    let bulk_storage_service = Arc::new(S3StorageService::new(
        &s3_config,
        "comhairle-media".to_owned(),
    )) as Arc<dyn BulkStorageService>;

    // Setup Websocket service
    let websockets = Arc::new(ComhairleWebSocketService::new());

    // Setup Transcription Service
    let transcription_service = match &config.transcription_service {
        Some(TranscriptionServiceConfig::AmazonTranscribe(_)) => {
            Some(Arc::new(AmazonTranscriber::new().await)
                as Arc<dyn comhairle::transcription_service::Transcriber>)
        }
        None => None,
    };

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

    let (worker_service, storage) = match init_worker_service(&config.worker_service).await {
        Some((worker_service, storage)) => (Some(worker_service), Some(storage)),
        _ => (None, None),
    };

    let categorization_service = config.categorization_service.as_ref().map(|config| {
        Arc::new(TttcCategorizer::new(&config.server_url, &config.api_key))
            as Arc<dyn CategorizationService>
    });

    let state = Arc::new(ComhairleState {
        db,
        mailer,
        config,
        websockets,
        translation_service,
        transcription_service,
        bot_service,
        wiki_poll_service,
        worker_service,
        bulk_storage_service,
        categorization_service,
    });

    // Register WebSocket message handlers
    comhairle::websockets::setup::register_handlers(&state);

    let app = setup_server(state.clone()).await?;

    let server_future = async move {
        // run our app with hyper
        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
        tracing::info!("listening on {}", listener.local_addr().unwrap());
        axum::serve(
            listener,
            app.into_make_service_with_connect_info::<std::net::SocketAddr>(),
        )
        .await
        .unwrap();
    };

    let worker_future = init_monitor(storage, &state);

    if let Some(worker_future) = worker_future {
        let _ = tokio::join!(server_future, worker_future);
    } else {
        server_future.await;
    }

    Ok(())
}
