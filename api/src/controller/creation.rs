use crate::tools::{AppState, ResponseData, ResponseStatus};
use axum::{
    extract::{Form, Path, State},
    http::StatusCode,
    response::Json,
};
use service::{CreationModel, CreationService};

use serde_json::json;
use serde_json::to_value;

pub struct CreationController;

impl CreationController {
    pub async fn list_creations(
        state: State<AppState>,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        let creations = CreationService::find_creation(&state.conn)
            .await
            .expect("Cannot find posts in page");

        let data = ResponseData {
            status: ResponseStatus::Success,
            data: creations,
        };
        let json_data = to_value(data).unwrap();
        println!("Json data: {:?}", json_data);
        Ok(Json(json!(json_data)))
    }

    pub async fn create_creation(
        state: State<AppState>,
        Json(form): Json<CreationModel>,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        CreationService::create_creation(&state.conn, form)
            .await
            .map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to create creation",
                )
            })?;
        Ok(Json(json!({
            "status": "success",
            "message": "Creation created successfully"
        })))
    }

    pub async fn update_creation(
        state: State<AppState>,
        Path(id): Path<i32>,
        Json(form): Json<CreationModel>,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        CreationService::update_creation_by_id(&state.conn, id, form)
            .await
            .map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to update creation",
                )
            })?;
        Ok(Json(json!({
            "status": "success",
            "message": "Creation updated successfully"
        })))
    }

    pub async fn delete_creation(
        state: State<AppState>,
        Path(id): Path<i32>,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        CreationService::delete_creation(&state.conn, id)
            .await
            .map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to delete creation",
                )
            })?;
        Ok(Json(json!({
            "status": "success",
            "message": "Creation deleted successfully"
        })))
    }

    pub async fn get_creation_by_user_id(
        state: State<AppState>,
        Path(id): Path<i32>,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        let creation = CreationService::find_creation_by_user_id(&state.conn, id)
            .await
            .expect("Cannot find creation by user id");

        let data = ResponseData {
            status: ResponseStatus::Success,
            data: creation,
        };
        let json_data = to_value(data).unwrap();
        println!("Json data: {:?}", json_data);
        Ok(Json(json!(json_data)))
    }
}
