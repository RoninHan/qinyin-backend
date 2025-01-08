mod controller;
mod flash;
mod tools;

use axum::{
    http::StatusCode,
    routing::{delete, get, get_service, post},
    Router,
};
use controller::setting::SettingController;
use migration::{Migrator, MigratorTrait};
use service::sea_orm::Database;

use std::{env, sync::LazyLock};
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
        .route("/user", get(UserController::list_users))
        .route("/user/:id", get(UserController::get_user_by_id))
        .route("/user/new", post(UserController::create_user))
        .route("/user/update/:id", post(UserController::update_user))
        .route("/user/delete/:id", delete(UserController::delete_user))
        .route("/song", get(SongController::list_songs))
        .route("/song/new", post(SongController::create_song))
        .route("/song/update/:id", post(SongController::update_song))
        .route("/song/delete/:id", delete(SongController::delete_song))
        .route("/song_type", get(SongTypeController::list_song_types))
        .route("/song_type/new", post(SongTypeController::create_song_type))
        .route(
            "/song_type/update/:id",
            post(SongTypeController::update_song_type),
        )
        .route(
            "/song_type/delete/:id",
            delete(SongTypeController::delete_song_type),
        )
        .route("/score", get(ScoreController::list_scores))
        .route("/score/new", post(ScoreController::create_score))
        .route("/score/update/:id", post(ScoreController::update_score))
        .route("/score/delete/:id", delete(ScoreController::delete_score))
        .route(
            "/score/get_globale_ranking/:id",
            get(ScoreController::get_globale_ranking),
        )
        .route(
            "/score/get_friends_ranking",
            get(ScoreController::get_friends_ranking),
        )
        .route("/lyrics", get(LyricsController::list_lyrics))
        .route("/lyrics/new", post(LyricsController::create_lyrics))
        .route("/lyrics/update/:id", post(LyricsController::update_lyrics))
        .route(
            "/lyrics/delete/:id",
            delete(LyricsController::delete_lyrics),
        )
        .route("/friends", get(FriendsController::list_friends))
        .route("/friends/new", post(FriendsController::create_friends))
        .route(
            "/friends/update/:id",
            post(FriendsController::update_friends),
        )
        .route(
            "/friends/delete/:id",
            delete(FriendsController::delete_friends),
        )
        .route("/creation", get(CreationController::list_creations))
        .route("/creation/new", post(CreationController::create_creation))
        .route(
            "/creation/update/:id",
            post(CreationController::update_creation),
        )
        .route(
            "/creation/delete/:id",
            delete(CreationController::delete_creation),
        )
        .route(
            "/creation/:id",
            get(CreationController::get_creation_by_user_id),
        )
        .route("/collect", get(CollectController::list_collects))
        .route("/collect/new", post(CollectController::create_collect))
        .route("/setting", get(SettingController::list_settings))
        .route("/setting/update", post(SettingController::update_setting))
        .route("/setting/new", post(SettingController::update_setting))
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
