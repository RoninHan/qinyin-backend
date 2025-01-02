use crate::flash::{get_flash_cookie, post_response, PostResponse};
use crate::tools::{AppState, FlashData, Params};
use anyhow::Ok;
use axum::{
    extract::{Form, Path, Query, State},
    http::StatusCode,
    response::Html,
};
use entity::collect;
use service::CollectService;
use tower_cookies::Cookies;

pub struct CollectController;

impl CollectController {
    pub async fn list_collects(
        state: State<AppState>,
        Query(params): Query<Params>,
        cookies: Cookies,
    ) -> Result<Html<String>, (StatusCode, &'static str)> {
        let page = params.page.unwrap_or(1);
        let posts_per_page = params.posts_per_page.unwrap_or(5);

        let (collects) = CollectService::find_collect(&state.conn)
            .await
            .expect("Cannot find posts in page");

        let mut ctx = tera::Context::new();
        ctx.insert("collects", &collects);

        if let Some(value) = get_flash_cookie::<FlashData>(&cookies) {
            ctx.insert("flash", &value);
        }

        let body = state
            .templates
            .render("index.html.tera", &ctx)
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Template error"))?;

        Ok(Html(body))
    }

    pub async fn create_collect(
        state: State<AppState>,
        mut cookies: Cookies,
        form: Form<collect::Model>,
    ) -> Result<PostResponse, (StatusCode, &'static str)> {
        let form = form.0;

        CollectService::create_collect(&state.conn, form)
            .await
            .map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to create collect",
                )
            })?;
        Ok(post_response(
            &mut cookies,
            FlashData {
                kind: "success".to_string(),
                message: "Collect created successfully".to_string(),
            },
        ))
    }
}
