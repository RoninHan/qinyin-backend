use crate::tools::{AppState, FlashData, Params};
use service::{ UserServices,
};
use axum::{
    response::Html,
    extract::{Form, Path, Query, State},
    http::StatusCode
};
use tower_cookies::{ Cookies};
use crate::flash::{get_flash_cookie, post_response, PostResponse};
use entity::user;

pub struct UserController;

impl UserController {
    pub async fn list_users(
        state: State<AppState>,
        Query(params): Query<Params>,
        cookies: Cookies,
    ) -> Result<Html<String>, (StatusCode, &'static str)> {
        let page = params.page.unwrap_or(1);
        let posts_per_page = params.posts_per_page.unwrap_or(5);

        let (posts, num_pages) = UserServices::find_user(&state.conn, page, posts_per_page)
            .await
            .expect("Cannot find posts in page");

        let mut ctx = tera::Context::new();
        ctx.insert("posts", &posts);
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

    pub async fn create_user(
        state: State<AppState>,
        mut cookies: Cookies,
        form: Form<user::Model>,
    ) -> Result<PostResponse, (StatusCode, &'static str)> {
        let form = form.0;

        UserServices::create_user(&state.conn, form)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create user"))?;

        Ok(post_response(
            &mut cookies,
            FlashData {
                kind: "success".to_string(),
                message: "Post created successfully".to_string(),
            },
        ))
    }

    pub async fn update_user(
        state: State<AppState>,
        Path(id): Path<i64>,
        mut cookies: Cookies,
        form: Form<user::Model>,
    ) -> Result<PostResponse, (StatusCode, &'static str)> {
        let form = form.0;

        UserServices::update_user_by_id(&state.conn, id, form)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to update user"))?;

        Ok(post_response(
            &mut cookies,
            FlashData {
                kind: "success".to_string(),
                message: "Post updated successfully".to_string(),
            },
        ))
    }

    pub async fn delete_user(
        state: State<AppState>,
        Path(id): Path<i64>,
        mut cookies: Cookies,
    ) -> Result<PostResponse, (StatusCode, &'static str)> {
        UserServices::delete_user(&state.conn, id)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to delete user"))?;

        Ok(post_response(
            &mut cookies,
            FlashData {
                kind: "success".to_string(),
                message: "Post deleted successfully".to_string(),
            },
        ))
    }
}
