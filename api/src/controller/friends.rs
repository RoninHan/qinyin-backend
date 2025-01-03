use crate::flash::{get_flash_cookie, post_response, PostResponse};
use crate::tools::{AppState, FlashData};
use axum::{
    extract::{Form, Path, State},
    http::StatusCode,
    response::Html,
};
use entity::friends;
use service::FriendsService;
use tower_cookies::Cookies;

pub struct FriendsController;

impl FriendsController {
    pub async fn list_friends(
        state: State<AppState>,
        cookies: Cookies,
    ) -> Result<Html<String>, (StatusCode, &'static str)> {
        let friends = FriendsService::find_friends(&state.conn)
            .await
            .expect("Cannot find posts in page");

        let mut ctx = tera::Context::new();
        ctx.insert("friends", &friends);

        if let Some(value) = get_flash_cookie::<FlashData>(&cookies) {
            ctx.insert("flash", &value);
        }

        let body = state
            .templates
            .render("index.html.tera", &ctx)
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Template error"))?;

        Ok(Html(body))
    }

    pub async fn create_friends(
        state: State<AppState>,
        mut cookies: Cookies,
        form: Form<friends::Model>,
    ) -> Result<PostResponse, (StatusCode, &'static str)> {
        let form = form.0;

        FriendsService::create_friends(&state.conn, form)
            .await
            .map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to create friends",
                )
            })?;
        Ok(post_response(
            &mut cookies,
            FlashData {
                kind: "success".to_string(),
                message: "Friends created successfully".to_string(),
            },
        ))
    }

    pub async fn update_friends(
        state: State<AppState>,
        mut cookies: Cookies,
        Path(id): Path<i64>,
        form: Form<friends::Model>,
    ) -> Result<PostResponse, (StatusCode, &'static str)> {
        let form = form.0;

        FriendsService::update_friends_by_id(&state.conn, id, form)
            .await
            .map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to update friends",
                )
            })?;
        Ok(post_response(
            &mut cookies,
            FlashData {
                kind: "success".to_string(),
                message: "Friends updated successfully".to_string(),
            },
        ))
    }

    pub async fn delete_friends(
        state: State<AppState>,
        mut cookies: Cookies,
        Path(id): Path<i64>,
    ) -> Result<PostResponse, (StatusCode, &'static str)> {
        FriendsService::delete_friends(&state.conn, id)
            .await
            .map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to delete friends",
                )
            })?;
        Ok(post_response(
            &mut cookies,
            FlashData {
                kind: "success".to_string(),
                message: "Friends deleted successfully".to_string(),
            },
        ))
    }
}
