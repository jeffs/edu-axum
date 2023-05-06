use std::sync::{
    atomic::{AtomicUsize, Ordering::Relaxed},
    Arc,
};

use axum::Extension;

pub struct State {
    pub next: AtomicUsize,
}

impl State {
    pub fn arc() -> Arc<State> {
        Arc::new(State {
            next: AtomicUsize::new(1),
        })
    }
}

pub async fn get_shared(Extension(state): Extension<Arc<State>>) -> String {
    state.next.fetch_add(1, Relaxed).to_string()
}
