use crate::tools::{AppState, FlashData, ResponseData, ResponseStatus};
use axum::{
    extract::{Form, Path, State},
    http::StatusCode,
    response::Json,
};
use entity::friends;
use service::{FriendsModel, FriendsService};
use tower_cookies::Cookies;

use serde_json::json;
use serde_json::to_value;

pub struct FriendsController;

impl FriendsController {
    pub async fn list_friends(
        state: State<AppState>,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        let friends = FriendsService::find_friends(&state.conn)
            .await
            .expect("Cannot find posts in page");

        let data = ResponseData {
            status: ResponseStatus::Success,
            data: friends,
        };
        let json_data = to_value(data).unwrap();
        println!("Json data: {:?}", json_data);
        Ok(Json(json!(json_data)))
    }

    pub async fn create_friends(
        state: State<AppState>,
        Json(payload): Json<FriendsModel>,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        FriendsService::create_friends(&state.conn, payload)
            .await
            .map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to create friends",
                )
            })?;
        Ok(Json(json!({
            "status": "success",
            "message": "Friends created successfully"
        })))
    }

    pub async fn update_friends(
        state: State<AppState>,
        Path(id): Path<i32>,
        Json(form): Json<FriendsModel>,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        FriendsService::update_friends_by_id(&state.conn, id, form)
            .await
            .map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to update friends",
                )
            })?;
        Ok(Json(json!({
            "status": "success",
            "message": "Friends updated successfully"
        })))
    }

    pub async fn delete_friends(
        state: State<AppState>,
        Path(id): Path<i32>,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        FriendsService::delete_friends(&state.conn, id)
            .await
            .map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to delete friends",
                )
            })?;
        Ok(Json(json!({
            "status": "success",
            "message": "Friends deleted successfully"
        })))
    }
}
