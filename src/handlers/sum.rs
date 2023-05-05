use axum::{
    extract::{Path, Query},
    Json,
};
use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct SumRequest(i32, i32);

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub struct SumQueryRequest {
    first_number: i32,
    second_number: i32,
}

pub async fn get(Path(numbers): Path<SumRequest>) -> String {
    (numbers.0 + numbers.1).to_string()
}

pub async fn get_query(Query(request): Query<SumQueryRequest>) -> String {
    (request.first_number + request.second_number).to_string()
}

pub async fn post(Json(numbers): Json<Vec<i32>>) -> Json<i32> {
    numbers.into_iter().sum::<i32>().into()
}
