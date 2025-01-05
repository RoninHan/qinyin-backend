use crate::tools::{AppState, ResponseData, ResponseStatus};
use axum::{
    extract::{Form, Path, State},
    http::StatusCode,
    response::Json,
};
use service::{FriendRankingModel, ScoreModel, ScoreService};

use serde_json::json;
use serde_json::to_value;

pub struct ScoreController;

impl ScoreController {
    pub async fn list_scores(
        state: State<AppState>,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        let scores = ScoreService::find_score(&state.conn)
            .await
            .expect("Cannot find scores in page");

        let data = ResponseData {
            status: ResponseStatus::Success,
            data: scores,
        };
        let json_data = to_value(data).unwrap();
        println!("Json data: {:?}", json_data);
        Ok(Json(json!(json_data)))
    }

    pub async fn create_score(
        state: State<AppState>,
        Json(form): Json<ScoreModel>,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        ScoreService::create_score(&state.conn, form)
            .await
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create score"))?;
        Ok(Json(json!({
            "status": "success",
            "message": "Score created successfully"
        })))
    }

    pub async fn update_score(
        state: State<AppState>,
        Path(id): Path<i32>,
        Json(form): Json<ScoreModel>,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        ScoreService::update_score_by_id(&state.conn, id, form)
            .await
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to update score"))?;
        Ok(Json(json!({
            "status": "success",
            "message": "Score updated successfully"
        })))
    }

    pub async fn delete_score(
        state: State<AppState>,
        Path(id): Path<i32>,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        ScoreService::delete_score(&state.conn, id)
            .await
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to delete score"))?;
        Ok(Json(json!({
            "status": "success",
            "message": "Score deleted successfully"
        })))
    }

    // 全球排行榜
    pub async fn get_globale_ranking(
        state: State<AppState>,
        Path(id): Path<i32>,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        let scores = ScoreService::get_score_by_song_id(&state.conn, id)
            .await
            .expect("Cannot find scores in page");

        let data = ResponseData {
            status: ResponseStatus::Success,
            data: scores,
        };
        let json_data = to_value(data).unwrap();
        println!("Json data: {:?}", json_data);
        Ok(Json(json!(json_data)))
    }

    //好友排行版
    pub async fn get_friends_ranking(
        state: State<AppState>,
        Json(form): Json<FriendRankingModel>,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        let scores = ScoreService::get_score_by_user_id(&state.conn, form.song_id, form.user_id)
            .await
            .expect("Cannot find scores in page");

        let data = ResponseData {
            status: ResponseStatus::Success,
            data: scores,
        };
        let json_data = to_value(data).unwrap();
        println!("Json data: {:?}", json_data);
        Ok(Json(json!(json_data)))
    }
}
