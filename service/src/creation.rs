use ::entity::{creation, creation::Entity as Creation};
use chrono::Utc;
use prelude::DateTimeWithTimeZone;
use sea_orm::*;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct CreationModel {
    pub user_id: i32,
    pub song_src: String,
    pub name: String,
}

pub struct CreationService;

impl CreationService {
    pub async fn create_creation(
        db: &DbConn,
        form_data: CreationModel,
    ) -> Result<creation::ActiveModel, DbErr> {
        creation::ActiveModel {
            user_id: Set(form_data.user_id),
            song_src: Set(form_data.song_src.to_owned()),
            name: Set(form_data.name.to_owned()),
            created_at: Set(DateTimeWithTimeZone::from(Utc::now())),
            updated_at: Set(DateTimeWithTimeZone::from(Utc::now())),
            ..Default::default()
        }
        .save(db)
        .await
    }

    pub async fn update_creation_by_id(
        db: &DbConn,
        id: i32,
        form_data: CreationModel,
    ) -> Result<creation::Model, DbErr> {
        let creation: creation::ActiveModel = Creation::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find creation.".to_owned()))
            .map(Into::into)?;

        creation::ActiveModel {
            id: creation.id,
            user_id: Set(form_data.user_id),
            song_src: Set(form_data.song_src.to_owned()),
            name: Set(form_data.name.to_owned()),
            updated_at: Set(DateTimeWithTimeZone::from(Utc::now())),
            ..Default::default()
        }
        .update(db)
        .await
    }

    pub async fn delete_creation(db: &DbConn, id: i32) -> Result<DeleteResult, DbErr> {
        let creation: creation::ActiveModel = Creation::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find creation.".to_owned()))
            .map(Into::into)?;

        creation.delete(db).await
    }

    pub async fn find_creation_by_user_id(
        db: &DbConn,
        user_id: i32,
    ) -> Result<Option<creation::Model>, DbErr> {
        Creation::find()
            .filter(creation::Column::UserId.eq(user_id))
            .one(db)
            .await
    }

    pub async fn find_creation(db: &DbConn) -> Result<Vec<creation::Model>, DbErr> {
        Creation::find().all(db).await
    }
}
