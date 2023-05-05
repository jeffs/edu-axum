use axum::extract::Path;

pub async fn get(Path(number): Path<i32>) -> String {
    (number * number).to_string()
}
