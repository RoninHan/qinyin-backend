use ::entity::{friends,firends::Entity as Friends};
use sea_orm::*;

pub struct FriendsService;

impl FriendsService {
    pub async fn create_friends(db: &DbConn,form_data: friends::Model)-> Result<friends::ActiveModel, DbErr> {
        friends::ActiveModel {
            user_id: Set(form_data.user_id),
            friend_id: Set(form_data.friend_id),
            created_at: Set(chrono::Utc::now().naive_utc()),
            updated_at: Set(chrono::Utc::now().naive_utc()),
            ..Default::default()
        }
        .save(db)
        .await
    }

    pub async fn update_friends_by_id(db: &DbConn, id: i64, form_data: friends::Model) -> Result<friends::Model, DbErr> {
        let friends: friends::ActiveModel = Friends::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find friends.".to_owned()))
            .map(Into::into)?;

        friends::ActiveModel {
            id: friends.id,
            user_id: Set(form_data.user_id),
            friend_id: Set(form_data.friend_id),
            updated_at: Set(chrono::Utc::now().naive_utc()),
            ..Default::default()
        }
        .update(db)
        .await
    }

    pub async fn delete_friends(db: &DbConn, id: i64) -> Result<DeleteResult, DbErr> {
        let friends: friends::ActiveModel = Friends::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find friends.".to_owned()))
            .map(Into::into)?;

        friends.delete(db).await
    }

    pub async fn find_friends_by_id(db: &DbConn, id: i64) -> Result<Option<friends::Model>, DbErr> {
        Friends::find_by_id(id).one(db).await
    }

    pub async fn find_friends(db: &DbConn) -> Result<Vec<friends::Model>, DbErr> {
        Friends::find().all(db).await
    }
}