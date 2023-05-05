use axum::{extract::Path, Json};
use axum::{headers::UserAgent, TypedHeader};
use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct SumRequest(i32, i32);

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub struct SumQueryRequest {
    first_number: i32,
    second_number: i32,
}

pub mod header {
    use axum::http::HeaderMap;

    use super::*;

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
}

pub mod square {
    use super::*;

    pub async fn get(Path(number): Path<i32>) -> String {
        (number * number).to_string()
    }
}

pub mod sum {
    use axum::extract::Query;

    use super::*;

    pub async fn get(Path(numbers): Path<SumRequest>) -> String {
        (numbers.0 + numbers.1).to_string()
    }

    pub async fn get_query(Query(request): Query<SumQueryRequest>) -> String {
        (request.first_number + request.second_number).to_string()
    }

    pub async fn post(Json(numbers): Json<Vec<i32>>) -> Json<i32> {
        numbers.into_iter().sum::<i32>().into()
    }
}
