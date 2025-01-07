use crate::tools::{AppState, Params, ResponseData, ResponseStatus};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use entity::song;
use service::{SongModel, SongService};

use serde_json::json;
use serde_json::to_value;

pub struct SongController;

impl SongController {
    pub async fn list_songs(
        state: State<AppState>,
        Query(params): Query<Params>,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        let page = params.page.unwrap_or(1);
        let posts_per_page = params.posts_per_page.unwrap_or(5);

        let (songs, num_pages) = SongService::find_song(&state.conn, page, posts_per_page)
            .await
            .expect("Cannot find posts in page");

        let data = ResponseData {
            status: ResponseStatus::Success,
            data: songs,
        };
        let json_data = to_value(data).unwrap();
        println!("Json data: {:?}", json_data);
        Ok(Json(json!(json_data)))
    }

    pub async fn create_song(
        state: State<AppState>,
        Json(form): Json<SongModel>,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        let res = SongService::create_song(&state.conn, form)
            .await
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create song"))?;

        let data = ResponseData {
            status: ResponseStatus::Success,
            data: song::Model {
                id: res.id.unwrap(),
                name: res.name.unwrap(),
                author: res.author.unwrap(),
                song_type_id: res.song_type_id.unwrap(),
                singer: res.singer.unwrap(),
                created_at: res.created_at.unwrap(),
                updated_at: res.updated_at.unwrap(),
            },
        };

        let json_data = to_value(data).unwrap();
        Ok(Json(json!(json_data)))
    }

    pub async fn update_song(
        state: State<AppState>,
        Path(id): Path<i32>,
        Json(form): Json<SongModel>,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        SongService::update_song_by_id(&state.conn, id, form)
            .await
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to update song"))?;

        Ok(Json(json!({
            "status": "success",
            "message": "Song updated successfully"
        })))
    }

    pub async fn delete_song(
        state: State<AppState>,
        Path(id): Path<i32>,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        SongService::delete_song(&state.conn, id)
            .await
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to delete song"))?;

        Ok(Json(json!({
            "status": "success",
            "message": "Song deleted successfully"
        })))
    }
}
