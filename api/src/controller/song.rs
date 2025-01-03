use crate::flash::{get_flash_cookie, post_response, PostResponse};
use crate::tools::{AppState, FlashData, Params};
use axum::{
    extract::{Form, Path, Query, State},
    http::StatusCode,
    response::Html,
};
use entity::song;
use service::{ SongService};
use tower_cookies::Cookies;

pub struct SongController;

impl SongController {
    pub async fn list_songs(
        state: State<AppState>,
        Query(params): Query<Params>,
        cookies: Cookies,
    ) -> Result<Html<String>, (StatusCode, &'static str)> {
        let page = params.page.unwrap_or(1);
        let posts_per_page = params.posts_per_page.unwrap_or(5);

        let (songs, num_pages) = SongService::find_song(&state.conn, page, posts_per_page)
            .await
            .expect("Cannot find posts in page");

        let mut ctx = tera::Context::new();
        ctx.insert("songs", &songs);
        ctx.insert("page", &page);
        ctx.insert("posts_per_page", &posts_per_page);
        ctx.insert("num_pages", &num_pages);

        if let Some(value) = get_flash_cookie::<FlashData>(&cookies) {
            ctx.insert("flash", &value);
        }

        let body = state
            .templates
            .render("index.html.tera", &ctx)
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Template error"))?;

        Ok(Html(body))
    }

    pub async fn create_song(
        state: State<AppState>,
        mut cookies: Cookies,
        form: Form<song::Model>,
    ) -> Result<PostResponse, (StatusCode, &'static str)> {
        let form = form.0;

        SongService::create_song(&state.conn, form)
            .await
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create song"))?;

        Ok(post_response(
            &mut cookies,
            FlashData {
                kind: "success".to_owned(),
                message: "Song created successfully".to_owned(),
            },
        ))
    }

    pub async fn update_song(
        state: State<AppState>,
        Path(id): Path<i64>,
        mut cookies: Cookies,
        form: Form<song::Model>,
    ) -> Result<PostResponse, (StatusCode, &'static str)> {
        let form = form.0;

        SongService::update_song_by_id(&state.conn, id, form)
            .await
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to update song"))?;

        Ok(post_response(
            &mut cookies,
            FlashData {
                kind: "success".to_owned(),
                message: "Song updated successfully".to_owned(),
            },
        ))
    }

    pub async fn delete_song(
        state: State<AppState>,
        Path(id): Path<i64>,
        mut cookies: Cookies,
    ) -> Result<PostResponse, (StatusCode, &'static str)> {
        SongService::delete_song(&state.conn, id)
            .await
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to delete song"))?;

        Ok(post_response(
            &mut cookies,
            FlashData {
                kind: "success".to_string(),
                message: "Song deleted successfully".to_string(),
            },
        ))
    }
}
