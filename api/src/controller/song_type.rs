use crate::flash::{get_flash_cookie, post_response, PostResponse};
use crate::tools::{AppState, FlashData, Params};
use axum::{
    extract::{Form, Path, Query, State},
    http::StatusCode,
    response::Html,
};
use entity::song_type;
use service::SongTypeService;
use tower_cookies::Cookies;

pub struct SongTypeController;

impl SongTypeController {
    pub async fn list_song_types(
        state: State<AppState>,
        Query(params): Query<Params>,
        cookies: Cookies,
    ) -> Result<Html<String>, (StatusCode, &'static str)> {
        let page = params.page.unwrap_or(1);
        let posts_per_page = params.posts_per_page.unwrap_or(5);

        let (song_types, num_pages) =
            SongTypeService::find_song_type(&state.conn, page, posts_per_page)
                .await
                .expect("Cannot find posts in page");

        let mut ctx = tera::Context::new();
        ctx.insert("song_types", &song_types);
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

    pub async fn create_song_type(
        state: State<AppState>,
        mut cookies: Cookies,
        form: Form<song_type::Model>,
    ) -> Result<PostResponse, (StatusCode, &'static str)> {
        let form = form.0;

        SongTypeService::create_song_type(&state.conn, form)
            .await
            .map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to create song type",
                )
            })?;

        Ok(post_response(
            &mut cookies,
            FlashData {
                kind: "success".to_string(),
                message: "Song type created successfully".to_string(),
            },
        ))
    }

    pub async fn delete_song_type(
        state: State<AppState>,
        Path(id): Path<i64>,
        mut cookies: Cookies,
    ) -> Result<PostResponse, (StatusCode, &'static str)> {
        SongTypeService::delete_song_type(&state.conn, id)
            .await
            .map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to delete song type",
                )
            })?;

        Ok(post_response(
            &mut cookies,
            FlashData {
                kind: "success".to_string(),
                message: "Song type deleted successfully".to_string(),
            },
        ))
    }
}
