use serde::{Deserialize, Serialize};
use service::sea_orm::DatabaseConnection;
use tera::Tera;
#[derive(Clone)]
pub struct AppState {
    pub templates: Tera,
    pub conn: DatabaseConnection,
}

#[derive(Deserialize)]
pub struct Params {
    pub page: Option<u64>,
    pub posts_per_page: Option<u64>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct FlashData {
    pub kind: String,
    pub message: String,
}

#[derive(Deserialize, Serialize)]
pub enum ResponseStatus {
    Success,
    Error,
}

#[derive(Deserialize, Serialize)]
pub struct ResponseData<T> {
    pub status: ResponseStatus,
    pub data: T,
}
