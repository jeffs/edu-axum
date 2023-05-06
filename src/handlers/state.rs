use std::sync::Arc;

use axum::Extension;
use tokio::sync::Mutex;

#[derive(Clone, Default)]
pub struct State {
    pub count: usize,
}

impl State {
    pub fn arc() -> Arc<Mutex<State>> {
        Arc::new(Mutex::from(State::default()))
    }
}

pub async fn get_shared(Extension(state): Extension<Arc<Mutex<State>>>) -> String {
    let mut state = state.lock().await;
    state.count += 1;
    state.count.to_string()
}
