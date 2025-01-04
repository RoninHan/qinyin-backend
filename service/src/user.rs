use ::entity::{user, user::Entity as User};
use chrono::{DateTime, Utc};
use sea_orm::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct UserModel {
    pub name: String,
    pub sex: String,
    pub email: String,
    pub app_id: String,
    pub phone: String,
    pub birthday: DateTime<Utc>,
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
            email: Set(form_data.email.to_owned()),
            app_id: Set(form_data.app_id.to_owned()),
            phone: Set(form_data.phone.to_owned()),
            birthday: Set(form_data.birthday.naive_utc()),
            created_at: Set(chrono::Utc::now().naive_utc()),
            updated_at: Set(chrono::Utc::now().naive_utc()),
            ..Default::default()
        }
        .save(db)
        .await
    }

    pub async fn update_user_by_id(
        db: &DbConn,
        id: i32,
        form_data: user::Model,
    ) -> Result<user::Model, DbErr> {
        let user: user::ActiveModel = User::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find user.".to_owned()))
            .map(Into::into)?;

        user::ActiveModel {
            id: user.id,
            name: Set(form_data.name.to_owned()),
            email: Set(form_data.email.to_owned()),
            app_id: Set(form_data.app_id.to_owned()),
            sex: Set(form_data.sex.to_owned()),
            phone: Set(form_data.phone.to_owned()),
            birthday: Set(form_data.birthday),
            updated_at: Set(chrono::Utc::now().naive_utc()),
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
}
