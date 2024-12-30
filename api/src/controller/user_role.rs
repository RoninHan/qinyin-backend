use crate::tools::{AppState, FlashData, Params};
use service::{
    Query as QueryCore, UserRoleServices,
};
use axum::{
    response::Html,
    extract::{Form, Query, State},
    http::StatusCode
};
use tower_cookies::{ Cookies};
use crate::flash::{get_flash_cookie, post_response, PostResponse};
use entity::user_role;

pub struct UserRoleController;

impl UserRoleController {
    pub async fn list_user_roles(
        state: State<AppState>,
        Query(params): Query<Params>,
        cookies: Cookies,
    ) -> Result<Html<String>, (StatusCode, &'static str)> {
        let page = params.page.unwrap_or(1);
        let posts_per_page = params.posts_per_page.unwrap_or(5);

        let (posts, num_pages) = QueryCore::find_posts_in_page(&state.conn, page, posts_per_page)
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

    pub async fn create_user_role(
        state: State<AppState>,
        mut cookies: Cookies,
        form: Form<user_role::Model>,
    ) -> Result<PostResponse, (StatusCode, &'static str)> {
        let form = form.0;

        UserRoleServices::create_user_role(&state.conn, form)
            .await
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create user role"))?;

        Ok(post_response(
            &mut cookies,
            FlashData {
                kind: "success".to_string(),
                message: "User role created successfully".to_string(),
            },
        ))
    }

    pub async fn delete_user_role(
        state: State<AppState>,
        mut cookies: Cookies,
        form: Form<user_role::Model>,
    ) -> Result<PostResponse, (StatusCode, &'static str)> {
        let form = form.0;

        UserRoleServices::delete_user_role_by_id(&state.conn, form.id)
            .await
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to delete user role"))?;

        Ok(post_response(
            &mut cookies,
            FlashData {
                kind: "success".to_string(),
                message: "Post deleted successfully".to_string(),
            },
        ))
    }
}
