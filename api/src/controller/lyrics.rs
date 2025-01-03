use crate::flash::{get_flash_cookie, post_response, PostResponse};
use crate::tools::{AppState, FlashData, Params};
use axum::{
    extract::{Form, Path, Query, State},
    http::StatusCode,
    response::Html,
};
use entity::lyrics;
use service::LyricsService;
use tower_cookies::Cookies;

pub struct LyricsController;

impl LyricsController {
    pub async fn list_lyrics(
        state: State<AppState>,
        Query(params): Query<Params>,
        cookies: Cookies,
    ) -> Result<Html<String>, (StatusCode, &'static str)> {
        let (lyrics) = LyricsService::find_lyrics(&state.conn)
            .await
            .expect("Cannot find lyrics in page");

        let mut ctx = tera::Context::new();
        ctx.insert("lyrics", &lyrics);

        if let Some(value) = get_flash_cookie::<FlashData>(&cookies) {
            ctx.insert("flash", &value);
        }

        let body = state
            .templates
            .render("index.html.tera", &ctx)
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Template error"))?;

        Ok(Html(body))
    }

    pub async fn create_lyrics(
        state: State<AppState>,
        mut cookies: Cookies,
        form: Form<lyrics::Model>,
    ) -> Result<PostResponse, (StatusCode, &'static str)> {
        let form = form.0;

        LyricsService::create_lyrics(&state.conn, form)
            .await
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create lyrics"))?;
        Ok(post_response(
            &mut cookies,
            FlashData {
                kind: "success".to_string(),
                message: "Lyrics created successfully".to_string(),
            },
        ))
    }

    pub async fn update_lyrics(
        state: State<AppState>,
        mut cookies: Cookies,
        Path(id): Path<i64>,
        form: Form<lyrics::Model>,
    ) -> Result<PostResponse, (StatusCode, &'static str)> {
        let form = form.0;

        LyricsService::update_lyrics_by_id(&state.conn, id, form)
            .await
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to update lyrics"))?;
        Ok(post_response(
            &mut cookies,
            FlashData {
                kind: "success".to_string(),
                message: "Lyrics updated successfully".to_string(),
            },
        ))
    }

    pub async fn delete_lyrics(
        state: State<AppState>,
        mut cookies: Cookies,
        Path(id): Path<i64>,
    ) -> Result<PostResponse, (StatusCode, &'static str)> {
        LyricsService::delete_lyrics(&state.conn, id)
            .await
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to delete lyrics"))?;
        Ok(post_response(
            &mut cookies,
            FlashData {
                kind: "success".to_string(),
                message: "Lyrics deleted successfully".to_string(),
            },
        ))
    }
}
