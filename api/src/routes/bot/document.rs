pub fn router(state: Arc<ComhairleState>) -> ApiRouter {
    ApiRouter::new()
    .with_state(state)
}
