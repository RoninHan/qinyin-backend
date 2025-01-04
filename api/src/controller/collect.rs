use crate::tools::{AppState, ResponseData, ResponseStatus};
use axum::{
    extract::{Form, State},
    http::StatusCode,
    response::Json,
};
use service::{CollectModel, CollectService};
use tower_cookies::Cookies;

use serde_json::json;
use serde_json::to_value;

pub struct CollectController;

impl CollectController {
    pub async fn list_collects(
        state: State<AppState>,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        let collects = CollectService::find_collect(&state.conn)
            .await
            .expect("Cannot find posts in page");

        let data = ResponseData {
            status: ResponseStatus::Success,
            data: collects,
        };
        let json_data = to_value(data).unwrap();
        println!("Json data: {:?}", json_data);
        Ok(Json(json!(json_data)))
    }

    pub async fn create_collect(
        state: State<AppState>,
        Json(form): Json<CollectModel>,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        CollectService::create_collect(&state.conn, form)
            .await
            .map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to create collect",
                )
            })?;
        Ok(Json(json!({
            "status": "success",
            "message": "Collect created successfully"
        })))
    }
}
