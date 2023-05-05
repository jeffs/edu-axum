use axum::{http::HeaderMap, extract::Path, TypedHeader, headers::UserAgent};

pub async fn get(Path(name): Path<String>, headers: HeaderMap) -> String {
    headers
        .get(name)
        .and_then(|value| value.to_str().ok())
        .unwrap_or_default()
        .to_owned()
}

pub async fn get_user_agent(TypedHeader(user_agent): TypedHeader<UserAgent>) -> String {
    user_agent.to_string()
}
