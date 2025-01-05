use ::entity::{song, song::Entity as Song};
use chrono::Utc;
use prelude::DateTimeWithTimeZone;
use sea_orm::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct SongModel {
    pub name: String,
    pub author: String,
    pub song_type_id: i32,
    pub singer: String,
}

pub struct SongService;

impl SongService {
    pub async fn create_song(
        db: &DbConn,
        form_data: SongModel,
    ) -> Result<song::ActiveModel, DbErr> {
        song::ActiveModel {
            name: Set(form_data.name.to_owned()),
            author: Set(form_data.author.to_owned()),
            song_type_id: Set(Some(form_data.song_type_id)),
            singer: Set(form_data.singer.to_owned()),
            created_at: Set(DateTimeWithTimeZone::from(Utc::now())),
            updated_at: Set(DateTimeWithTimeZone::from(Utc::now())),
            ..Default::default()
        }
        .save(db)
        .await
    }

    pub async fn update_song_by_id(
        db: &DbConn,
        id: i32,
        form_data: SongModel,
    ) -> Result<song::Model, DbErr> {
        let song: song::ActiveModel = Song::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find song.".to_owned()))
            .map(Into::into)?;

        song::ActiveModel {
            id: song.id,
            name: Set(form_data.name.to_owned()),
            singer: Set(form_data.singer.to_owned()),
            author: Set(form_data.author.to_owned()),
            song_type_id: Set(Some(form_data.song_type_id)),
            updated_at: Set(DateTimeWithTimeZone::from(Utc::now())),
            ..Default::default()
        }
        .update(db)
        .await
    }

    pub async fn delete_song(db: &DbConn, id: i32) -> Result<DeleteResult, DbErr> {
        let song: song::ActiveModel = Song::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find song.".to_owned()))
            .map(Into::into)?;

        song.delete(db).await
    }

    pub async fn find_song_by_id(db: &DbConn, id: i32) -> Result<Option<song::Model>, DbErr> {
        Song::find_by_id(id).one(db).await
    }

    pub async fn find_song(
        db: &DbConn,
        page: u64,
        per_page: u64,
    ) -> Result<(Vec<song::Model>, u64), DbErr> {
        let paginator = Song::find()
            .order_by_asc(song::Column::Id)
            .paginate(db, per_page);
        let num_pages = paginator.num_pages().await?;

        paginator.fetch_page(page - 1).await.map(|p| (p, num_pages))
    }
}
