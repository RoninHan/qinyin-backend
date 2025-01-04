use ::entity::{friends, friends::Entity as Friends};
use chrono::Utc;
use prelude::DateTimeWithTimeZone;
use sea_orm::*;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct FriendsModel {
    pub user_id: i32,
    pub friend_user_id: i32,
}

pub struct FriendsService;

impl FriendsService {
    pub async fn create_friends(
        db: &DbConn,
        form_data: FriendsModel,
    ) -> Result<friends::ActiveModel, DbErr> {
        friends::ActiveModel {
            user_id: Set(form_data.user_id),
            friend_user_id: Set(form_data.friend_user_id),
            created_at: Set(DateTimeWithTimeZone::from(Utc::now())),
            updated_at: Set(DateTimeWithTimeZone::from(Utc::now())),
            ..Default::default()
        }
        .save(db)
        .await
    }

    pub async fn update_friends_by_id(
        db: &DbConn,
        id: i32,
        form_data: FriendsModel,
    ) -> Result<friends::Model, DbErr> {
        let friends: friends::ActiveModel = Friends::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find friends.".to_owned()))
            .map(Into::into)?;

        friends::ActiveModel {
            id: friends.id,
            user_id: Set(form_data.user_id),
            friend_user_id: Set(form_data.friend_user_id),
            updated_at: Set(DateTimeWithTimeZone::from(Utc::now())),
            ..Default::default()
        }
        .update(db)
        .await
    }

    pub async fn delete_friends(db: &DbConn, id: i32) -> Result<DeleteResult, DbErr> {
        let friends: friends::ActiveModel = Friends::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find friends.".to_owned()))
            .map(Into::into)?;

        friends.delete(db).await
    }

    pub async fn find_friends_by_id(db: &DbConn, id: i32) -> Result<Option<friends::Model>, DbErr> {
        Friends::find_by_id(id).one(db).await
    }

    pub async fn find_friends(db: &DbConn) -> Result<Vec<friends::Model>, DbErr> {
        Friends::find().all(db).await
    }
}
