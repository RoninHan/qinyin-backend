mod controller;
mod flash;
mod middleware;
mod tools;

use axum::{
    http::StatusCode,
    middleware as axum_middleware,
    routing::{delete, get, get_service, post},
    Router,
};
use controller::setting::SettingController;
use middleware::auth::Auth;
use migration::{Migrator, MigratorTrait};
use service::sea_orm::Database;

use std::{env, sync::Arc};
use tera::Tera;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;

use crate::controller::collect::CollectController;
use crate::controller::creation::CreationController;
use crate::controller::friends::FriendsController;
use crate::controller::lyrics::LyricsController;
use crate::controller::score::ScoreController;
use crate::controller::song::SongController;
use crate::controller::song_type::SongTypeController;
use crate::controller::user::UserController;

use tools::AppState;

#[tokio::main]
async fn start() -> anyhow::Result<()> {
    env::set_var("RUST_LOG", "debug");
    tracing_subscriber::fmt::init();

    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{host}:{port}");

    let conn = Database::connect(db_url)
        .await
        .expect("Database connection failed");
    Migrator::up(&conn, None).await.unwrap();

    let templates = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*"))
        .expect("Tera initialization failed");

    let state = AppState { templates, conn };

    let app = Router::new()
        .route("/api/login", post(UserController::login))
        .route(
            "/api/create_admin_user",
            post(UserController::create_admin_user),
        )
        .route(
            "/api/user",
            get(UserController::list_users).layer(axum_middleware::from_fn_with_state(
                state.clone(),
                Auth::authorization_middleware,
            )),
        )
        .route(
            "/api/user/:id",
            get(UserController::get_user_by_id).layer(axum_middleware::from_fn_with_state(
                state.clone(),
                Auth::authorization_middleware,
            )),
        )
        .route(
            "/api/user/new",
            post(UserController::create_user).layer(axum_middleware::from_fn_with_state(
                state.clone(),
                Auth::authorization_middleware,
            )),
        )
        .route(
            "/api/user/update/:id",
            post(UserController::update_user).layer(axum_middleware::from_fn_with_state(
                state.clone(),
                Auth::authorization_middleware,
            )),
        )
        .route(
            "/api/user/delete/:id",
            delete(UserController::delete_user).layer(axum_middleware::from_fn_with_state(
                state.clone(),
                Auth::authorization_middleware,
            )),
        )
        .route("/api/song", get(SongController::list_songs))
        .route(
            "/api/song/new",
            post(SongController::create_song).layer(axum_middleware::from_fn_with_state(
                state.clone(),
                Auth::authorization_middleware,
            )),
        )
        .route(
            "/api/song/update/:id",
            post(SongController::update_song).layer(axum_middleware::from_fn_with_state(
                state.clone(),
                Auth::authorization_middleware,
            )),
        )
        .route(
            "/api/song/delete/:id",
            delete(SongController::delete_song).layer(axum_middleware::from_fn_with_state(
                state.clone(),
                Auth::authorization_middleware,
            )),
        )
        .route("/api/song_type", get(SongTypeController::list_song_types))
        .route(
            "/api/song_type/new",
            post(SongTypeController::create_song_type).layer(axum_middleware::from_fn_with_state(
                state.clone(),
                Auth::authorization_middleware,
            )),
        )
        .route(
            "/api/song_type/update/:id",
            post(SongTypeController::update_song_type).layer(axum_middleware::from_fn_with_state(
                state.clone(),
                Auth::authorization_middleware,
            )),
        )
        .route(
            "/api/song_type/delete/:id",
            delete(SongTypeController::delete_song_type).layer(
                axum_middleware::from_fn_with_state(state.clone(), Auth::authorization_middleware),
            ),
        )
        .route(
            "/api/score",
            get(ScoreController::list_scores).layer(axum_middleware::from_fn_with_state(
                state.clone(),
                Auth::authorization_middleware,
            )),
        )
        .route("/api/score/new", post(ScoreController::create_score))
        .route(
            "/api/score/update/:id",
            post(ScoreController::update_score).layer(axum_middleware::from_fn_with_state(
                state.clone(),
                Auth::authorization_middleware,
            )),
        )
        .route(
            "/api/score/delete/:id",
            delete(ScoreController::delete_score).layer(axum_middleware::from_fn_with_state(
                state.clone(),
                Auth::authorization_middleware,
            )),
        )
        .route(
            "/api/score/get_globale_ranking/:id",
            get(ScoreController::get_globale_ranking),
        )
        .route(
            "/api/score/get_friends_ranking",
            get(ScoreController::get_friends_ranking),
        )
        .route("/api/lyrics", get(LyricsController::list_lyrics))
        .route(
            "/api/lyrics/new",
            post(LyricsController::create_lyrics).layer(axum_middleware::from_fn_with_state(
                state.clone(),
                Auth::authorization_middleware,
            )),
        )
        .route(
            "/api/lyrics/update/:id",
            post(LyricsController::update_lyrics).layer(axum_middleware::from_fn_with_state(
                state.clone(),
                Auth::authorization_middleware,
            )),
        )
        .route(
            "/api/lyrics/delete/:id",
            delete(LyricsController::delete_lyrics).layer(axum_middleware::from_fn_with_state(
                state.clone(),
                Auth::authorization_middleware,
            )),
        )
        .route(
            "/api/friends",
            get(FriendsController::list_friends).layer(axum_middleware::from_fn_with_state(
                state.clone(),
                Auth::authorization_middleware,
            )),
        )
        .route("/api/friends/new", post(FriendsController::create_friends))
        .route(
            "/api/friends/update/:id",
            post(FriendsController::update_friends).layer(axum_middleware::from_fn_with_state(
                state.clone(),
                Auth::authorization_middleware,
            )),
        )
        .route(
            "/api/friends/delete/:id",
            delete(FriendsController::delete_friends).layer(axum_middleware::from_fn_with_state(
                state.clone(),
                Auth::authorization_middleware,
            )),
        )
        .route("/api/creation", get(CreationController::list_creations))
        .route(
            "/api/creation/new",
            post(CreationController::create_creation),
        )
        .route(
            "/api/creation/update/:id",
            post(CreationController::update_creation),
        )
        .route(
            "/api/creation/delete/:id",
            delete(CreationController::delete_creation).layer(axum_middleware::from_fn_with_state(
                state.clone(),
                Auth::authorization_middleware,
            )),
        )
        .route(
            "/api/creation/:id",
            get(CreationController::get_creation_by_user_id),
        )
        .route("/api/collect", get(CollectController::list_collects))
        .route(
            "/api/collect/new",
            post(CollectController::create_collect).layer(axum_middleware::from_fn_with_state(
                state.clone(),
                Auth::authorization_middleware,
            )),
        )
        .route(
            "/api/setting",
            get(SettingController::list_settings).layer(axum_middleware::from_fn_with_state(
                state.clone(),
                Auth::authorization_middleware,
            )),
        )
        .route(
            "/api/setting/update/:id",
            post(SettingController::update_setting).layer(axum_middleware::from_fn_with_state(
                state.clone(),
                Auth::authorization_middleware,
            )),
        )
        .route(
            "/api/setting/new",
            post(SettingController::update_setting).layer(axum_middleware::from_fn_with_state(
                state.clone(),
                Auth::authorization_middleware,
            )),
        )
        .nest_service(
            "/static",
            get_service(ServeDir::new(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/static"
            )))
            .handle_error(|error| async move {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Unhandled internal error: {error}"),
                )
            }),
        )
        .layer(CookieManagerLayer::new())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(&server_url).await.unwrap();
    axum::serve(listener, app).await?;

    Ok(())
}

pub fn main() {
    let result = start();

    if let Some(err) = result.err() {
        println!("Error: {err}");
    }
}
