use ::entity::{score, score::Entity as Score};
use chrono::Utc;
use prelude::DateTimeWithTimeZone;
use sea_orm::*;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct ScoreModel {
    pub user_id: i32,
    pub song_id: i32,
    pub score: i32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct FriendRankingModel {
    pub user_id: i32,
    pub song_id: i32,
}

pub struct ScoreService;

impl ScoreService {
    pub async fn create_score(
        db: &DbConn,
        form_data: ScoreModel,
    ) -> Result<score::ActiveModel, DbErr> {
        score::ActiveModel {
            user_id: Set(form_data.user_id),
            song_id: Set(form_data.song_id),
            score: Set(form_data.score),
            created_at: Set(DateTimeWithTimeZone::from(Utc::now())),
            updated_at: Set(DateTimeWithTimeZone::from(Utc::now())),
            ..Default::default()
        }
        .save(db)
        .await
    }

    pub async fn update_score_by_id(
        db: &DbConn,
        id: i32,
        form_data: ScoreModel,
    ) -> Result<score::Model, DbErr> {
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
            updated_at: Set(DateTimeWithTimeZone::from(Utc::now())),
            ..Default::default()
        }
        .update(db)
        .await
    }

    pub async fn delete_score(db: &DbConn, id: i32) -> Result<DeleteResult, DbErr> {
        let score: score::ActiveModel = Score::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find score.".to_owned()))
            .map(Into::into)?;

        score.delete(db).await
    }

    pub async fn find_score_by_id(db: &DbConn, id: i32) -> Result<Option<score::Model>, DbErr> {
        Score::find_by_id(id).one(db).await
    }

    pub async fn find_score(db: &DbConn) -> Result<Vec<score::Model>, DbErr> {
        Score::find().all(db).await
    }

    pub async fn get_score_by_song_id(db: &DbConn, id: i32) -> Result<Vec<score::Model>, DbErr> {
        Score::find()
            .filter(score::Column::SongId.eq(id))
            .order_by_desc(score::Column::Score)
            .all(db)
            .await
    }

    pub async fn get_score_by_user_id(
        db: &DbConn,
        song_id: i32,
        user_id: i32,
    ) -> Result<Vec<score::Model>, DbErr> {
        Score::find()
            .filter(score::Column::SongId.eq(song_id))
            .filter(score::Column::UserId.eq(user_id))
            .order_by_desc(score::Column::Score)
            .all(db)
            .await
    }
}
