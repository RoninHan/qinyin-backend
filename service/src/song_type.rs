use ::entity::{song_type, song_type::Entity as SongType};
use chrono::Utc;
use prelude::DateTimeWithTimeZone;
use sea_orm::*;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct SongTypeModel {
    pub name: String,
}

pub struct SongTypeService;

impl SongTypeService {
    pub async fn create_song_type(
        db: &DbConn,
        form_data: SongTypeModel,
    ) -> Result<song_type::ActiveModel, DbErr> {
        song_type::ActiveModel {
            name: Set(form_data.name.to_owned()),
            created_at: Set(DateTimeWithTimeZone::from(Utc::now())),
            updated_at: Set(DateTimeWithTimeZone::from(Utc::now())),
            ..Default::default()
        }
        .save(db)
        .await
    }

    pub async fn update_song_type_by_id(
        db: &DbConn,
        id: i32,
        form_data: SongTypeModel,
    ) -> Result<song_type::Model, DbErr> {
        let song_type: song_type::ActiveModel = SongType::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find song type.".to_owned()))
            .map(Into::into)?;

        song_type::ActiveModel {
            id: song_type.id,
            name: Set(form_data.name.to_owned()),
            updated_at: Set(DateTimeWithTimeZone::from(Utc::now())),
            ..Default::default()
        }
        .update(db)
        .await
    }

    pub async fn delete_song_type(db: &DbConn, id: i32) -> Result<DeleteResult, DbErr> {
        let song_type: song_type::ActiveModel = SongType::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find song type.".to_owned()))
            .map(Into::into)?;

        song_type.delete(db).await
    }

    pub async fn find_song_type(
        db: &DbConn,
        page: u64,
        per_page: u64,
    ) -> Result<(Vec<song_type::Model>, u64), DbErr> {
        let paginator = SongType::find()
            .order_by_asc(song_type::Column::Id)
            .paginate(db, per_page);
        let num_pages = paginator.num_pages().await?;

        paginator.fetch_page(page - 1).await.map(|p| (p, num_pages))
    }
}
