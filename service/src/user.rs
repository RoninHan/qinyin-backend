use ::entity::{user, user::Entity as User};
use chrono::{DateTime, Utc};
use prelude::DateTimeWithTimeZone;
use sea_orm::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct UserModel {
    pub name: String,
    pub sex: String,
    pub email: String,
    pub app_id: String,
    pub phone: String,
    pub birthday: Option<DateTimeWithTimeZone>,
}

pub struct UserServices;

impl UserServices {
    pub async fn create_user(
        db: &DbConn,
        form_data: UserModel,
    ) -> Result<user::ActiveModel, DbErr> {
        let sex: i32 = form_data.sex.parse().expect("msg");
        user::ActiveModel {
            name: Set(form_data.name.to_owned()),
            sex: Set(sex),
            email: Set(Some(form_data.email)),
            app_id: Set(form_data.app_id.to_owned()),
            phone: Set(Some(form_data.phone.to_owned())),
            birthday: Set(form_data.birthday),
            created_at: Set(DateTimeWithTimeZone::from(Utc::now())),
            updated_at: Set(DateTimeWithTimeZone::from(Utc::now())),
            ..Default::default()
        }
        .save(db)
        .await
    }

    pub async fn update_user_by_id(
        db: &DbConn,
        id: i32,
        form_data: UserModel,
    ) -> Result<user::Model, DbErr> {
        let user: user::ActiveModel = User::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find user.".to_owned()))
            .map(Into::into)?;
        let sex: i32 = form_data.sex.parse().expect("msg");
        user::ActiveModel {
            id: user.id,
            name: Set(form_data.name.to_owned()),
            email: Set(Some(form_data.email)),
            app_id: Set(form_data.app_id.to_owned()),
            sex: Set(sex),
            phone: Set(Some(form_data.phone)),
            birthday: Set(form_data.birthday),
            updated_at: Set(DateTimeWithTimeZone::from(Utc::now())),
            ..Default::default()
        }
        .update(db)
        .await
    }

    pub async fn delete_user(db: &DbConn, id: i32) -> Result<DeleteResult, DbErr> {
        let user: user::ActiveModel = User::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find user.".to_owned()))
            .map(Into::into)?;

        user.delete(db).await
    }

    pub async fn delete_all_users(db: &DbConn) -> Result<DeleteResult, DbErr> {
        User::delete_many().exec(db).await
    }

    pub async fn find_user(
        db: &DbConn,
        page: u64,
        per_page: u64,
    ) -> Result<(Vec<user::Model>, u64), DbErr> {
        let paginator = User::find()
            .order_by_asc(user::Column::Id)
            .paginate(db, per_page);
        let num_pages = paginator.num_pages().await?;

        paginator.fetch_page(page - 1).await.map(|p| (p, num_pages))
    }

    pub async fn find_user_by_id(db: &DbConn, id: i32) -> Result<Option<user::Model>, DbErr> {
        User::find_by_id(id).one(db).await
    }
}
