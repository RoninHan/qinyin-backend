use crate::tools::{AppState, ResponseData, ResponseStatus};
use axum::{
    extract::{Form, Path, State},
    http::StatusCode,
    response::Json,
};
use service::SettingService;

use serde_json::json;
use serde_json::to_value;

pub struct SettingController;

impl SettingController {
    pub async fn list_settings(
        state: State<AppState>,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        let settings = SettingService::get_setting(&state.conn, 1)
            .await
            .expect("Cannot find settings in page");

        let data = ResponseData {
            status: ResponseStatus::Success,
            data: settings,
        };
        let json_data = to_value(data).unwrap();
        println!("Json data: {:?}", json_data);
        Ok(Json(json!(json_data)))
    }

    pub async fn create_setting(
        state: State<AppState>,
        Json(form): Json<String>,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        SettingService::create_setting(&state.conn, Some(form))
            .await
            .map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to create setting",
                )
            })?;
        Ok(Json(json!({
            "status": "success",
            "message": "Setting created successfully"
        })))
    }

    pub async fn update_setting(
        state: State<AppState>,
        Path(id): Path<i32>,
        Json(form): Json<String>,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        SettingService::update_setting_by_id(&state.conn, id, Some(form))
            .await
            .map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to update setting",
                )
            })?;
        Ok(Json(json!({
            "status": "success",
            "message": "Setting updated successfully"
        })))
    }
}
