use crate::flash::{get_flash_cookie, post_response, PostResponse};
use crate::tools::{AppState, FlashData};
use axum::{
    extract::{Form, Path, State},
    http::StatusCode,
    response::Html,
};
use entity::creation;
use service::CreationService;
use tower_cookies::Cookies;

pub struct CreationController;

impl CreationController {
    pub async fn list_creations(
        state: State<AppState>,
        cookies: Cookies,
    ) -> Result<Html<String>, (StatusCode, &'static str)> {
        let creations = CreationService::find_creation(&state.conn)
            .await
            .expect("Cannot find posts in page");

        let mut ctx = tera::Context::new();
        ctx.insert("creations", &creations);

        if let Some(value) = get_flash_cookie::<FlashData>(&cookies) {
            ctx.insert("flash", &value);
        }

        let body = state
            .templates
            .render("index.html.tera", &ctx)
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Template error"))?;

        Ok(Html(body))
    }

    pub async fn create_creation(
        state: State<AppState>,
        mut cookies: Cookies,
        form: Form<creation::Model>,
    ) -> Result<PostResponse, (StatusCode, &'static str)> {
        let form = form.0;

        CreationService::create_creation(&state.conn, form)
            .await
            .map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to create creation",
                )
            })?;
        Ok(post_response(
            &mut cookies,
            FlashData {
                kind: "success".to_string(),
                message: "Creation created successfully".to_string(),
            },
        ))
    }

    pub async fn update_creation(
        state: State<AppState>,
        mut cookies: Cookies,
        form: Form<creation::Model>,
    ) -> Result<PostResponse, (StatusCode, &'static str)> {
        let form = form.0;

        CreationService::update_creation_by_id(&state.conn, form.id,form)
            .await
            .map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to update creation",
                )
            })?;
        Ok(post_response(
            &mut cookies,
            FlashData {
                kind: "success".to_string(),
                message: "Creation updated successfully".to_string(),
            },
        ))
    }

    pub async fn delete_creation(
        state: State<AppState>,
        mut cookies: Cookies,
        Path(id): Path<i64>,
    ) -> Result<PostResponse, (StatusCode, &'static str)> {

        CreationService::delete_creation(&state.conn, id)
            .await
            .map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to delete creation",
                )
            })?;
            let data = FlashData {
                kind: "success".to_owned(),
                message: "Creation deleted successfully".to_owned(),
            };
        Ok(post_response(&mut cookies,data))
    }
}
