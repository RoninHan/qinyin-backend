use ::entity::{score, score::Entity as Score};
use sea_orm::*;

pub struct ScoreService;

impl ScoreService{
    pub async fn create_score(db: &DbConn, form_data: score::Model) -> Result<score::ActiveModel, DbErr> {
        score::ActiveModel {
            user_id: Set(form_data.user_id),
            song_id: Set(form_data.song_id),
            score: Set(form_data.score),
            created_at: Set(chrono::Utc::now().naive_utc()),
            updated_at: Set(chrono::Utc::now().naive_utc()),
            ..Default::default()
        }
        .save(db)
        .await

    }

    pub async fn update_score_by_id(db: &DbConn, id: i64, form_data: score::Model) -> Result<score::Model, DbErr> {
        let score: score::ActiveModel = Score::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find score.".to_owned()))
            .map(Into::into)?;

        score::ActiveModel {
            id: score.id,
            user_id: Set(form_data.user_id),
            song_id: Set(form_data.song_id),
            score: Set(form_data.score),
            updated_at: Set(chrono::Utc::now().naive_utc()),
            ..Default::default()
        }
        .update(db)
        .await
    }

    pub async fn delete_score(db: &DbConn, id: i64) -> Result<DeleteResult, DbErr> {
        let score: score::ActiveModel = Score::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find score.".to_owned()))
            .map(Into::into)?;

        score.delete(db).await
    }

    pub async fn find_score_by_id(db: &DbConn, id: i64) -> Result<Option<score::Model>, DbErr> {
        Score::find_by_id(id).one(db).await
    }

    pub async fn find_score(db: &DbConn) -> Result<Vec<score::Model>, DbErr> {
        Score::find().all(db).await
    }
}