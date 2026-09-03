use std::{collections::HashMap, sync::Arc};

use comhairle::config::{ComhairleConfig, MailerConfig};
use comhairle::error::ComhairleError;
use comhairle::models::{permissions::ResourcePermission, users::User};
use comhairle::websockets::handlers::video_call::VideoCallMessageHandler;
use comhairle::websockets::{
    ConnectionId, WebSocketConnection, WebSocketMessageHandler, messages::WebSocketMessage,
};
use comhairle::wiki_poll_service::error::WikiPollServiceError;
use comhairle::wiki_poll_service::polis_service::WikiPollReport;
use comhairle::wiki_poll_service::{
    ModerationStatus, PostedStatement, WikiPoll, WikiPollComment, WikiPollConfigUpdate,
    WikiPollLogin, WikiPollXid,
};
use comhairle::{ComhairleState, mailer, websockets, wiki_poll_service};
use lettre::message::SinglePart;
use minijinja::Value;
use uuid::Uuid;

#[derive(Clone)]
struct DummyMailer;
#[async_trait::async_trait]
impl mailer::ComhairleMailer for DummyMailer {
    fn send_email(
        &self,
        _to: &str,
        _subject: &str,
        _template: &str,
        _context: Value,
        _attachment: Option<SinglePart>,
    ) -> Result<(), ComhairleError> {
        todo!()
    }

    async fn send_conversation_invite_email(
        &self,
        _state: &Arc<ComhairleState>,
        _email: &str,
        _conversation_id: Uuid,
        _user_id: Uuid,
        _invite_id: Uuid,
        _locale: &str,
    ) -> Result<(), ComhairleError> {
        todo!()
    }

    fn send_welcome_email(&self, _user: &User, _verify_link: String) -> Result<(), ComhairleError> {
        todo!()
    }

    fn send_password_reset_email(
        &self,
        _to: &Option<String>,
        _username: &Option<String>,
        _reset_link: String,
    ) -> Result<(), ComhairleError> {
        todo!()
    }

    fn send_user_account_created_email(
        &self,
        _to: &Option<String>,
        _username: &Option<String>,
        _set_password_link: String,
    ) -> Result<(), ComhairleError> {
        todo!()
    }

    fn send_verification_email(
        &self,
        _username: &Option<String>,
        _email: &Option<String>,
        _verify_link: String,
    ) -> Result<(), ComhairleError> {
        todo!()
    }

    fn send_otp_email(
        &self,
        _username: &Option<String>,
        _email: &Option<String>,
        _passcode: String,
        _passcode_link: Option<String>,
    ) -> Result<(), ComhairleError> {
        todo!()
    }

    async fn send_event_registration_email(
        &self,
        _state: &Arc<ComhairleState>,
        _email: &str,
        _event_id: Uuid,
        _user_id: Uuid,
        _invite_id: Uuid,
        _locale: &str,
    ) -> Result<(), ComhairleError> {
        todo!()
    }

    async fn send_event_confirmation_email(
        &self,
        _state: &Arc<ComhairleState>,
        _email: &str,
        _event_id: Uuid,
        _user_id: Uuid,
        _locale: &str,
    ) -> Result<(), ComhairleError> {
        todo!()
    }

    async fn send_event_reminder(
        &self,
        _state: &Arc<ComhairleState>,
        _email: &str,
        _event_id: Uuid,
        _recipient_id: Uuid,
        _sender_id: Uuid,
        _locale: &str,
    ) -> Result<(), ComhairleError> {
        todo!()
    }

    fn send_conversation_broadcast_email(
        &self,
        _email: &str,
        _subject: &str,
        _html_body: &str,
    ) -> Result<(), ComhairleError> {
        todo!()
    }

    fn send_permission_notification_email(
        &self,
        _email: &str,
        _permission: &ResourcePermission,
        _action: &str,
    ) -> Result<(), ComhairleError> {
        todo!()
    }

    fn preview_email(
        &self,
        _template: &str,
        _slots_map: HashMap<String, String>,
        _variables_map: Option<HashMap<String, String>>,
    ) -> Result<String, ComhairleError> {
        todo!()
    }
}

#[derive(Clone)]
struct DummyWebSocketService;
#[async_trait::async_trait]
impl websockets::WebSocketService for DummyWebSocketService {
    fn add_connection(&self, _connection: WebSocketConnection) {
        todo!()
    }

    fn remove_connection(&self, _connection_id: &ConnectionId) -> Option<WebSocketConnection> {
        todo!()
    }

    async fn broadcast_to_all(&self, _message: &WebSocketMessage) -> Result<usize, ComhairleError> {
        todo!()
    }

    async fn broadcast_to_authenticated_users(
        &self,
        _message: &WebSocketMessage,
    ) -> Result<usize, ComhairleError> {
        todo!()
    }

    async fn send_to_user(
        &self,
        _user_id: &Uuid,
        _message: &WebSocketMessage,
    ) -> Result<usize, ComhairleError> {
        todo!()
    }

    async fn send_to_connections(
        &self,
        _connection_ids: &[ConnectionId],
        _message: &WebSocketMessage,
    ) -> Result<usize, ComhairleError> {
        todo!()
    }

    async fn broadcast_to_room(
        &self,
        _domain: &str,
        _room_id: &Uuid,
        _message: &WebSocketMessage,
    ) -> Result<usize, ComhairleError> {
        todo!()
    }

    fn get_connection_count(&self) -> usize {
        todo!()
    }

    fn get_user_connection_count(&self, _user_id: &Uuid) -> usize {
        todo!()
    }

    fn get_connected_user_ids(&self) -> Vec<Uuid> {
        todo!()
    }

    fn register_handler(&self, _handler: Arc<dyn WebSocketMessageHandler>) {
        todo!()
    }

    fn unregister_handler(&self, _domain: &str) -> Option<Arc<dyn WebSocketMessageHandler>> {
        todo!()
    }

    fn get_handler(&self, _domain: &str) -> Option<Arc<dyn WebSocketMessageHandler>> {
        todo!()
    }
}

#[derive(Clone)]
struct DummyWikiPollService;
#[async_trait::async_trait]
impl wiki_poll_service::WikiPollService for DummyWikiPollService {
    async fn create_random_admin_user(&self) -> Result<(String, String), WikiPollServiceError> {
        todo!()
    }

    async fn login(&self, _login: &WikiPollLogin) -> Result<String, WikiPollServiceError> {
        todo!()
    }

    async fn create_poll(&self, _auth_cookies: &str) -> Result<String, WikiPollServiceError> {
        todo!()
    }

    async fn post_seed_comment(
        &self,
        _comment: &str,
        _poll_id: &str,
        _auth_cookies: &str,
    ) -> Result<String, WikiPollServiceError> {
        todo!()
    }

    async fn post_statement(
        &self,
        _comment: &str,
        _poll_id: &str,
        _is_seed: bool,
        _auth_cookies: &str,
    ) -> Result<PostedStatement, WikiPollServiceError> {
        todo!()
    }

    async fn get_comments(
        &self,
        _poll_id: &str,
    ) -> Result<Vec<WikiPollComment>, WikiPollServiceError> {
        todo!()
    }

    async fn get_xids(
        &self,
        _poll_id: &str,
        _auth_cookies: &str,
    ) -> Result<Vec<WikiPollXid>, WikiPollServiceError> {
        todo!()
    }

    async fn get_report_data(
        &self,
        _poll_id: &str,
    ) -> Result<WikiPollReport, WikiPollServiceError> {
        todo!()
    }

    async fn moderate_comment(
        &self,
        _poll_id: &str,
        _tid: i32,
        _decision: ModerationStatus,
        _auth_cookies: &str,
    ) -> Result<(), WikiPollServiceError> {
        todo!()
    }

    async fn delete_poll(
        &self,
        _poll_id: &str,
        _auth_cookies: &str,
    ) -> Result<WikiPoll, WikiPollServiceError> {
        todo!()
    }

    async fn update_poll_config(
        &self,
        _poll_id: &str,
        _auth_cookies: &str,
        _config: &WikiPollConfigUpdate,
    ) -> Result<WikiPoll, WikiPollServiceError> {
        todo!()
    }

    async fn get_participant_vote_count(
        &self,
        _poll_id: &str,
        _pid: u32,
        _auth_cookies: &str,
    ) -> Result<u32, WikiPollServiceError> {
        todo!()
    }
}

pub(crate) unsafe fn create_dummy_state() -> ComhairleState {
    unsafe {
        ComhairleState {
            db: sqlx::PgPool::connect_lazy("postgres://localhost/dummy").unwrap(),
            config: ComhairleConfig {
                enable_rate_limiting: true,
                redis_cache_ttl_secs: 0,
                mailer: MailerConfig {
                    from_email: "".to_string(),
                    host: "".to_string(),
                    password: "".to_string(),
                    user: "".to_string(),
                },
                database_url: "".to_string(),
                default_conversation_image_url: "".to_string(),
                domain: "".to_string(),
                heyform_url: "".to_string(),
                jwt_secret: "".to_string(),
                polis_url: "".to_string(),
                resource_bucket: "".to_string(),
                admin_users: None,
                bot_service: None,
                bulk_storage_service: None,
                categorization_service: None,
                redis_cache_url: None,
                transcription_service: None,
                translator: None,
                video_call_service: None,
                websocket_service: None,
                whitelisted_domains: None,
                worker_service: None,
            },
            mailer: Arc::new(DummyMailer),
            websockets: Arc::new(DummyWebSocketService),
            video_call_handler: Arc::from_raw(
                std::ptr::NonNull::dangling().as_ptr() as *const VideoCallMessageHandler
            ),
            translation_service: None,
            bot_service: None,
            wiki_poll_service: Arc::new(DummyWikiPollService),
            bulk_storage_service: None,
            transcription_service: None,
            worker_service: None,
            categorization_service: None,
            redis_conn: None,
        }
    }
}
