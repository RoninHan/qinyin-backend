use crate::tools::{AppState, Params, ResponseData, ResponseStatus};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use service::{LyricsModel, LyricsService, SongModel, SongService};

use serde_json::json;
use serde_json::to_value;

#[derive(Deserialize, Debug, Serialize)]
struct SongData{
    id: i32,
    name: String,
    author: String,
    song_type_id: i32,
    singer: String,
    lyric: String,
}
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
    
        let mut songsList: Vec<SongData> = Vec::new();
        for song in songs.into_iter() {
            let lyric = LyricsService::find_lyrics_by_song_id(&state.conn, song.id).await.unwrap();
            let lyric = lyric.map(|lyric| lyric.lyric).unwrap_or("".to_string());
            songsList.push(SongData {
                id: song.id,
                name: song.name,
                author: song.author,
                song_type_id: song.song_type_id.unwrap(),
                singer: song.singer,
                lyric,
            });
        }

        let data = ResponseData {
            status: ResponseStatus::Success,
            data: songsList,
        };
        let json_data = to_value(data).unwrap();
        println!("Json data: {:?}", json_data);
        Ok(Json(json!(json_data)))
    }

    pub async fn create_song(
        state: State<AppState>,
        Json(form): Json<SongModel>,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        let lyric = form.lyric.clone();
        let res = SongService::create_song(&state.conn, form)
            .await
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create song"))?;

        let create_lyrics_form = LyricsModel {
            song_id: res.id.unwrap(),
            lyric,
        };
        let _ = LyricsService::create_lyrics(&state.conn, create_lyrics_form).await;


        Ok(Json(json!({
            "status": "success",
            "message": "Song created successfully"
        })))
    }

    pub async fn update_song(
        state: State<AppState>,
        Path(id): Path<i32>,
        Json(form): Json<SongModel>,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        
        let lyric = form.lyric.clone();
        let res_song = SongService::update_song_by_id(&state.conn, id, form)
            .await
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to update song"))?;

        let res_find_lyric = LyricsService::find_lyrics_by_song_id(&state.conn, res_song.id).await.map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to find lyric"))?;
        // 如果根據song_id找不到歌詞，則新增歌詞
        if res_find_lyric.is_none(){
            let create_lyric=lyric.clone();
            let create_lyrics_form = LyricsModel {
                song_id: res_song.id,
                lyric:create_lyric,
            };

            let _ = LyricsService::create_lyrics(&state.conn, create_lyrics_form).await;
        }else{
            let update_lyrics_form = LyricsModel {
                song_id: res_song.id,
                lyric,
            };
    
            let _ = LyricsService::update_lyrics_by_id(&state.conn, res_find_lyric.unwrap().id, update_lyrics_form).await;
        }
        
        Ok(Json(json!({
            "status": "success",
            "message": "Song updated successfully",
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

    pub async fn find_song_by_id(
        state: State<AppState>,
        Path(id): Path<i32>,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        let song = SongService::find_song_by_id(&state.conn, id)
            .await
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to find song"))?;

        let lyric = LyricsService::find_lyrics_by_song_id(&state.conn, id).await.unwrap();
        let lyric = lyric.map(|lyric| lyric.lyric).unwrap_or("".to_string());

        let song = song.map(|song| SongData {
            id: song.id,
            name: song.name,
            author: song.author,
            song_type_id: song.song_type_id.unwrap(),
            singer: song.singer,
            lyric,
        });
        
        Ok(Json(json!({
            "status": "success",
            "data": song
        })))
    }
}
