use crate::tools::{AppState, Params, ResponseData, ResponseStatus};
use axum::{
    extract::{Form, Path, Query, State},
    http::StatusCode,
    response::Json,
};
use serde_json::json;
use serde_json::to_value;
use service::{SongTypeModel, SongTypeService};
use tower_cookies::Cookies;

pub struct SongTypeController;

impl SongTypeController {
    pub async fn list_song_types(
        state: State<AppState>,
        Query(params): Query<Params>,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        let page = params.page.unwrap_or(1);
        let posts_per_page = params.posts_per_page.unwrap_or(5);

        let (song_types, num_pages) =
            SongTypeService::find_song_type(&state.conn, page, posts_per_page)
                .await
                .expect("Cannot find posts in page");

        let data = ResponseData {
            status: ResponseStatus::Success,
            data: song_types,
        };
        let json_data = to_value(data).unwrap();
        println!("Json data: {:?}", json_data);
        Ok(Json(json!(json_data)))
    }

    pub async fn create_song_type(
        state: State<AppState>,
        Json(form): Json<SongTypeModel>,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        SongTypeService::create_song_type(&state.conn, form)
            .await
            .map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to create song type",
                )
            })?;

        Ok(Json(json!({
            "status": "success",
            "message": "Song type created successfully"
        })))
    }

    pub async fn delete_song_type(
        state: State<AppState>,
        Path(id): Path<i32>,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        SongTypeService::delete_song_type(&state.conn, id)
            .await
            .map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to delete song type",
                )
            })?;

        Ok(Json(json!({
            "status": "success",
            "message": "Song type deleted successfully"
        })))
    }

    pub async fn update_song_type(
        state: State<AppState>,
        Path(id): Path<i32>,
        Json(form): Json<SongTypeModel>,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        SongTypeService::update_song_type_by_id(&state.conn, id, form)
            .await
            .map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to update song type",
                )
            })?;

        Ok(Json(json!({
            "status": "success",
            "message": "Song type updated successfully"
        })))
    }
}
