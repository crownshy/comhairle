use aws_config::BehaviorVersion;
use comhairle::redis_connection::RedisImpl;
use comhairle::{
    ComhairleState,
    bot_service::{ComhairleBotService, ComhairleRagBotService},
    bulk_storage_service::{BulkStorageService, s3_storage::S3StorageService},
    categorization_service::{CategorizationService, tttc_categorizer::TttcCategorizer},
    config::{TranscriptionServiceConfig, TranslatorConfig},
    db::setup_db,
    mailer::Mailer,
    setup_server,
    transcription_service::amazon_transcriber::AmazonTranscriber,
    translation_service::GoogleTranslateService,
    websockets::ComhairleWebSocketService,
    websockets::handlers::video_call::VideoCallMessageHandler,
    wiki_poll_service::polis_service::PolisClient,
    worker_service::{init_monitor, init_worker_service},
};
use redis::aio::ConnectionManagerConfig;
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

    // Setup Websocket service
    let websockets =
        Arc::new(ComhairleWebSocketService::new(config.websocket_service.as_ref()).await?);

    // Setup Translation Service
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
    let bulk_storage_service = if let Some(bulk_storage_config) = &config.bulk_storage_service {
        let s3_config = aws_config::load_defaults(BehaviorVersion::latest()).await;
        Some(Arc::new(S3StorageService::new(
            &s3_config,
            bulk_storage_config.store_name.to_owned(),
        )) as Arc<dyn BulkStorageService>)
    } else {
        None
    };

    // Setup Transcription Service
    let transcription_service = match &config.transcription_service {
        Some(TranscriptionServiceConfig::AmazonTranscribe(_)) => {
            Some(Arc::new(AmazonTranscriber::new().await)
                as Arc<dyn comhairle::transcription_service::Transcriber>)
        }
        None => None,
    };

    // Setup bot service

    let bot_service = config.bot_service.as_ref().map(|config| {
        Arc::new(ComhairleRagBotService::new(&config.host, &config.api_key))
            as Arc<dyn ComhairleBotService>
    });

    let wiki_poll_service = Arc::new(PolisClient::new(&config.polis_url));

    let (worker_service, storage) = match init_worker_service(&config.worker_service).await {
        Some((worker_service, storage)) => (Some(worker_service), Some(storage)),
        _ => (None, None),
    };

    let categorization_service = config.categorization_service.as_ref().map(|config| {
        Arc::new(TttcCategorizer::new(&config.server_url, &config.api_key))
            as Arc<dyn CategorizationService>
    });

    // Setup Redis connection for permissions cache
    let redis_conn = if let Some(ref redis_url) = config.redis_cache_url {
        match redis::Client::open(redis_url.as_str()) {
            Ok(client) => {
                let timeout = std::time::Duration::from_secs(5);
                let config =
                    ConnectionManagerConfig::default().set_connection_timeout(Some(timeout));
                match client.get_connection_manager_lazy(config) {
                    Ok(manager) => {
                        tracing::info!("Redis permissions cache connected");
                        Some(Arc::new(RedisImpl::new(manager))
                            as Arc<dyn comhairle::redis_connection::RedisConnection>)
                    }
                    Err(e) => {
                        tracing::warn!("Redis permissions cache unavailable: {e}");
                        None
                    }
                }
            }
            Err(e) => {
                tracing::warn!("Invalid Redis URL for permissions cache: {e}");
                None
            }
        }
    } else {
        None
    };

    let video_call_handler = Arc::new(VideoCallMessageHandler::new());

    let state = Arc::new(ComhairleState {
        db,
        mailer,
        config,
        websockets,
        video_call_handler,
        translation_service,
        transcription_service,
        bot_service,
        wiki_poll_service,
        worker_service,
        bulk_storage_service,
        categorization_service,
        redis_conn,
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
