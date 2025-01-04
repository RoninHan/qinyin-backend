use ::entity::{lyric, lyric::Entity as Lyrics};
use chrono::Utc;
use prelude::DateTimeWithTimeZone;
use sea_orm::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct LyricsModel {
    pub song_id: i32,
    pub lyric: String,
}

pub struct LyricsService;

impl LyricsService {
    pub async fn create_lyrics(
        db: &DbConn,
        form_data: LyricsModel,
    ) -> Result<lyric::ActiveModel, DbErr> {
        lyric::ActiveModel {
            song_id: Set(form_data.song_id),
            lyric: Set(form_data.lyric.to_owned()),
            created_at: Set(DateTimeWithTimeZone::from(Utc::now())),
            updated_at: Set(DateTimeWithTimeZone::from(Utc::now())),
            ..Default::default()
        }
        .save(db)
        .await
    }

    pub async fn update_lyrics_by_id(
        db: &DbConn,
        id: i32,
        form_data: LyricsModel,
    ) -> Result<lyric::Model, DbErr> {
        let lyric: lyric::ActiveModel = Lyrics::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find lyric.".to_owned()))
            .map(Into::into)?;

        lyric::ActiveModel {
            id: lyric.id,
            song_id: Set(form_data.song_id),
            lyric: Set(form_data.lyric.to_owned()),
            updated_at: Set(DateTimeWithTimeZone::from(Utc::now())),
            ..Default::default()
        }
        .update(db)
        .await
    }

    pub async fn delete_lyrics(db: &DbConn, id: i32) -> Result<DeleteResult, DbErr> {
        let lyric: lyric::ActiveModel = Lyrics::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find lyric.".to_owned()))
            .map(Into::into)?;

        lyric.delete(db).await
    }

    pub async fn find_lyrics_by_id(db: &DbConn, id: i32) -> Result<Option<lyric::Model>, DbErr> {
        Lyrics::find_by_id(id).one(db).await
    }

    pub async fn find_lyrics(db: &DbConn) -> Result<Vec<lyric::Model>, DbErr> {
        Lyrics::find().all(db).await
    }
}
