use ::entity::{lyrics,lyrics::Entity as Lyrics};
use sea_orm::*;

pub struct LyricsService;

impl LyricsService {
    pub async fn create_lyrics(db: &DbConn,form_data: lyrics::Model) -> Result<lyrics::ActiveModel, DbErr> {
        lyrics::ActiveModel {
            song_id: Set(form_data.song_id),
            content: Set(form_data.content.to_owned()),
            created_at: Set(chrono::Utc::now().naive_utc()),
            updated_at: Set(chrono::Utc::now().naive_utc()),
            ..Default::default()
        }
        .save(db)
        .await
    }

    pub async fn update_lyrics_by_id(db: &DbConn, id: i64, form_data: lyrics::Model) -> Result<lyrics::Model, DbErr> {
        let lyrics: lyrics::ActiveModel = Lyrics::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find lyrics.".to_owned()))
            .map(Into::into)?;

        lyrics::ActiveModel {
            id: lyrics.id,
            song_id: Set(form_data.song_id),
            content: Set(form_data.content.to_owned()),
            updated_at: Set(chrono::Utc::now().naive_utc()),
            ..Default::default()
        }
        .update(db)
        .await
    }

    pub async fn delete_lyrics(db: &DbConn, id: i64) -> Result<DeleteResult, DbErr> {
        let lyrics: lyrics::ActiveModel = Lyrics::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find lyrics.".to_owned()))
            .map(Into::into)?;

        lyrics.delete(db).await
    }

    pub async fn find_lyrics_by_id(db: &DbConn, id: i64) -> Result<Option<lyrics::Model>, DbErr> {
        Lyrics::find_by_id(id).one(db).await
    }

    pub async fn find_lyrics(db: &DbConn) -> Result<Vec<lyrics::Model>, DbErr> {
        Lyrics::find().all(db).await
    }
}