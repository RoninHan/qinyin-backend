use ::entity::{creation, creation::Entity as Creation};
use sea_orm::*;

pub struct CreationService{
    pub async fn create_creation(db:&DbConn,form_data: creation::Model)->Result<creation::ActiveModel,DbErr>{
        creation::ActiveModel{
            user_id: Set(form_data.user_id),
            song_id: Set(form_data.song_id),
            created_at: Set(chrono::Utc::now().naive_utc()),
            updated_at: Set(chrono::Utc::now().naive_utc()),
            ..Default::default()
        }
        .save(db)
        .await
    }

    pub async fn update_creation_by_id(db: &DbConn, id: i64, form_data: creation::Model) -> Result<creation::Model, DbErr> {
        let creation: creation::ActiveModel = Creation::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find creation.".to_owned()))
            .map(Into::into)?;

        creation::ActiveModel {
            id: creation.id,
            user_id: Set(form_data.user_id),
            song_id: Set(form_data.song_id),
            updated_at: Set(chrono::Utc::now().naive_utc()),
            ..Default::default()
        }
        .update(db)
        .await
    }

    pub async fn delete_creation(db: &DbConn, id: i64) -> Result<DeleteResult, DbErr> {
        let creation: creation::ActiveModel = Creation::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find creation.".to_owned()))
            .map(Into::into)?;

        creation.delete(db).await
    }

    pub async fn find_creation_by_id(db: &DbConn, id: i64) -> Result<Option<creation::Model>, DbErr> {
        Creation::find_by_id(id).one(db).await
    }

    pub async fn find_creation(db: &DbConn) -> Result<Vec<creation::Model>, DbErr> {
        Creation::find().all(db).await
    }

    
}