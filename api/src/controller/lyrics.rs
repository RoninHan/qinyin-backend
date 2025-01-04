use crate::tools::{AppState, ResponseData, ResponseStatus};

use axum::{
    extract::{Form, Path, State},
    http::StatusCode,
    response::Json,
};
use serde_json::json;
use serde_json::to_value;
use service::{LyricsModel, LyricsService};

pub struct LyricsController;

impl LyricsController {
    pub async fn list_lyrics(
        state: State<AppState>,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        let lyricsdata = LyricsService::find_lyrics(&state.conn)
            .await
            .expect("Cannot find lyrics in page");

        let data = ResponseData {
            status: ResponseStatus::Success,
            data: lyricsdata,
        };
        let json_data = to_value(data).unwrap();
        println!("Json data: {:?}", json_data);
        Ok(Json(json!(json_data)))
    }

    pub async fn create_lyrics(
        state: State<AppState>,
        Json(form): Json<LyricsModel>,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        LyricsService::create_lyrics(&state.conn, form)
            .await
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create lyrics"))?;
        Ok(Json(json!({
            "status": "success",
            "message": "Lyrics created successfully"
        })))
    }

    pub async fn update_lyrics(
        state: State<AppState>,
        Path(id): Path<i32>,
        Json(form): Json<LyricsModel>,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        LyricsService::update_lyrics_by_id(&state.conn, id, form)
            .await
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to update lyrics"))?;
        Ok(Json(json!({
            "status": "success",
            "message": "Lyrics updated successfully"
        })))
    }

    pub async fn delete_lyrics(
        state: State<AppState>,
        Path(id): Path<i32>,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        LyricsService::delete_lyrics(&state.conn, id)
            .await
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to delete lyrics"))?;
        Ok(Json(json!({
            "status": "success",
            "message": "Lyrics deleted successfully"
        })))
    }
}
