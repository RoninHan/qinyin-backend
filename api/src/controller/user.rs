use crate::tools::{AppState, Params, ResponseData, ResponseStatus};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use service::{UserModel, UserServices};

use serde_json::json;
use serde_json::to_value;

pub struct UserController;

impl UserController {
    pub async fn list_users(
        state: State<AppState>,
        Query(params): Query<Params>,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        let page = params.page.unwrap_or(1);
        let posts_per_page = params.posts_per_page.unwrap_or(5);

        let (users, num_pages) = UserServices::find_user(&state.conn, page, posts_per_page)
            .await
            .expect("Cannot find posts in page");

        let data = ResponseData {
            status: ResponseStatus::Success,
            data: users,
        };

        let json_data = to_value(data).unwrap();
        println!("Json data: {:?}", json_data);
        Ok(Json(json!(json_data)))
    }

    pub async fn create_user(
        state: State<AppState>,
        Json(payload): Json<UserModel>,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        println!("Payload: {:?}", payload);
        UserServices::create_user(&state.conn, payload)
            .await
            .map_err(|e| {
                println!("Failed to create user: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create user")
            })?;

        Ok(Json(json!({
            "status": "success",
            "message": "User created successfully"
        })))
    }

    pub async fn update_user(
        state: State<AppState>,
        Path(id): Path<i32>,
        Json(payload): Json<UserModel>,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        println!("Payload: {:?}", payload);
        UserServices::update_user_by_id(&state.conn, id, payload)
            .await
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to update user"))?;

        Ok(Json(json!({
            "status": "success",
            "message": "User updated"})))
    }

    pub async fn delete_user(
        state: State<AppState>,
        Path(id): Path<i32>,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        UserServices::delete_user(&state.conn, id)
            .await
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to delete user"))?;

        Ok(Json(json!({
            "status": "success",
            "message": "User deleted"
        })))
    }

    pub async fn get_user_by_id(
        state: State<AppState>,
        Path(id): Path<i32>,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        let user = UserServices::find_user_by_id(&state.conn, id)
            .await
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to find user"))?;

        Ok(Json(json!({
            "status": "success",
            "data": user
        })))
    }
}
