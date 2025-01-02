use crate::flash::{get_flash_cookie, post_response, PostResponse};
use crate::tools::{AppState, FlashData, Params};
use anyhow::Ok;
use axum::{
    extract::{Form, Path, Query, State},
    http::StatusCode,
    response::Html,
};
use entity::score;
use service::ScoreService;
use tower_cookies::Cookies;

pub struct ScoreController;

impl ScoreController {
    pub async fn list_scores(
        state: State<AppState>,
        Query(params): Query<Params>,
        cookies: Cookies,
    ) -> Result<Html<String>, (StatusCode, &'static str)> {
        let (scores) = ScoreService::find_score(&state.conn)
            .await
            .expect("Cannot find posts in page");

        let mut ctx = tera::Context::new();
        ctx.insert("scores", &scores);

        if let Some(value) = get_flash_cookie::<FlashData>(&cookies) {
            ctx.insert("flash", &value);
        }

        let body = state
            .templates
            .render("index.html.tera", &ctx)
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Template error"))?;

        Ok(Html(body))
    }

    pub async fn create_score(
        state: State<AppState>,
        mut cookies: Cookies,
        form: Form<score::Model>,
    ) -> Result<PostResponse, (StatusCode, &'static str)> {
        let form = form.0;

        ScoreService::create_score(&state.conn, form)
            .await
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create score"))?;
        Ok(post_response(
            &mut cookies,
            FlashData {
                kind: "success".to_owned(),
                message: "Score created successfully".to_owned(),
            },
        ))
    }

    pub async fn update_score(
        state: State<AppState>,
        mut cookies: Cookies,
        Path(id): Path<i64>,
        form: Form<score::Model>,
    ) -> Result<PostResponse, (StatusCode, &'static str)> {
        let form = form.0;

        ScoreService::update_score_by_id(&state.conn, id, form)
            .await
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to update score"))?;
        Ok(post_response(
            &mut cookies,
            FlashData {
                kind: "success".to_owned(),
                message: "Score updated successfully".to_owned(),
            },
        ))
    }

    pub async fn delete_score(
        state: State<AppState>,
        mut cookies: Cookies,
        Path(id): Path<i64>,
    ) -> Result<PostResponse, (StatusCode, &'static str)> {
        ScoreService::delete_score(&state.conn, id)
            .await
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to delete score"))?;
        Ok(post_response(
            &mut cookies,
            FlashData {
                kind: "success".to_owned(),
                message: "Score deleted successfully".to_owned(),
            },
        ))
    }
}
