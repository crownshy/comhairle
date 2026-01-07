use apalis::prelude::{MemoryStorage, Monitor, WorkerBuilder, WorkerFactoryFn};
use comhairle::{
    bot_service::ComhairleRagBotService,
    config::TranslatorConfig,
    db::setup_db,
    mailer::Mailer,
    setup_server,
    translation_service::GoogleTranslateService,
    websockets::ComhairleWebSocketService,
    workers::{knowledge_bases::handle_knowledge_base_processing, JobQueues},
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
    let translation_service = match &config.translator {
        Some(TranslatorConfig::Google(google_config)) => Some(
            Arc::new(GoogleTranslateService::new(
                google_config.api_key.to_owned(),
            )) as Arc<dyn comhairle::translation_service::TranslationService>,
        ),
        None => None,
    };

    let websockets = Arc::new(ComhairleWebSocketService::new());
    let bot_service = Arc::new(ComhairleRagBotService::new(
        &config.bot_service_host,
        &config.bot_service_api_key,
    ));

    let knowledge_base_storage = MemoryStorage::new();
    let jobs = Arc::new(JobQueues {
        knowledge_bases: Arc::new(Mutex::new(knowledge_base_storage.clone())),
    });

    let state = Arc::new(ComhairleState {
        db,
        mailer,
        config,
        websockets,
        translation_service,
        bot_service,
        jobs,
    });

    let app = setup_server(state.clone()).await?;

    let server_future = async move {
        // run our app with hyper
        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
        tracing::info!("listening on {}", listener.local_addr().unwrap());
        axum::serve(listener, app).await.unwrap();
    };

    let knowledge_base_worker = WorkerBuilder::new("process_knowledge_base_job")
        .data(state.clone())
        .backend(knowledge_base_storage.clone())
        .build_fn(handle_knowledge_base_processing);

    let worker_future = { Monitor::new().register(knowledge_base_worker).run() };

    let _ = tokio::join!(server_future, worker_future);

    Ok(())
}
