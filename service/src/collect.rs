use ::entity::{collect, collect::Entity as Collect};
use chrono::Utc;
use prelude::DateTimeWithTimeZone;
use sea_orm::*;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct CollectModel {
    pub user_id: i32,
    pub song_id: i32,
}

pub struct CollectService;

impl CollectService {
    pub async fn create_collect(
        db: &DbConn,
        form_data: CollectModel,
    ) -> Result<collect::ActiveModel, DbErr> {
        collect::ActiveModel {
            user_id: Set(form_data.user_id),
            song_id: Set(form_data.song_id),
            created_at: Set(DateTimeWithTimeZone::from(Utc::now())),
            updated_at: Set(DateTimeWithTimeZone::from(Utc::now())),
            ..Default::default()
        }
        .save(db)
        .await
    }

    pub async fn update_collect_by_id(
        db: &DbConn,
        id: i32,
        form_data: CollectModel,
    ) -> Result<collect::Model, DbErr> {
        let collect: collect::ActiveModel = Collect::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find collect.".to_owned()))
            .map(Into::into)?;

        collect::ActiveModel {
            id: collect.id,
            user_id: Set(form_data.user_id),
            song_id: Set(form_data.song_id),
            updated_at: Set(DateTimeWithTimeZone::from(Utc::now())),
            ..Default::default()
        }
        .update(db)
        .await
    }

    pub async fn delete_collect(db: &DbConn, id: i32) -> Result<DeleteResult, DbErr> {
        let collect: collect::ActiveModel = Collect::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find collect.".to_owned()))
            .map(Into::into)?;

        collect.delete(db).await
    }

    pub async fn find_collect_by_id(db: &DbConn, id: i32) -> Result<Option<collect::Model>, DbErr> {
        Collect::find_by_id(id).one(db).await
    }

    pub async fn find_collect(db: &DbConn) -> Result<Vec<collect::Model>, DbErr> {
        Collect::find().all(db).await
    }
}
