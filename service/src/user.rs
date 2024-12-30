use ::entity::{user, user::Entity as User};
use sea_orm::*;

pub struct UserServices;

impl UserServices {
    pub async fn create_user(
        db: &DbConn,
        form_data: user::Model,
    ) -> Result<user::ActiveModel, DbErr> {
        user::ActiveModel {
            name: Set(form_data.name.to_owned()),
            email: Set(form_data.email.to_owned()),
            password: Set(form_data.password.to_owned()),
            id_card: Set(form_data.id_card.to_owned()),
            sex: Set(form_data.sex.to_owned()),
            phone: Set(form_data.phone.to_owned()),
            status: Set(String::from("active")),
            description: Set(form_data.description.to_owned()),
            job_number: Set(form_data.job_number.to_owned()),
            birthday: Set(form_data.birthday),
            department_id: Set(form_data.department_id),
            created_at: Set(chrono::Utc::now().naive_utc()),
            updated_at: Set(chrono::Utc::now().naive_utc()),
            ..Default::default()
        }
        .save(db)
        .await
    }

    pub async fn update_user_by_id(
        db: &DbConn,
        id: i64,
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
            password: Set(form_data.password.to_owned()),
            id_card: Set(form_data.id_card.to_owned()),
            sex: Set(form_data.sex.to_owned()),
            phone: Set(form_data.phone.to_owned()),
            description: Set(form_data.description.to_owned()),
            job_number: Set(form_data.job_number.to_owned()),
            birthday: Set(form_data.birthday),
            department_id: Set(form_data.department_id),
            updated_at: Set(chrono::Utc::now().naive_utc()),
            ..Default::default()
        }
        .update(db)
        .await
    }

    pub async fn delete_user(db: &DbConn, id: i64) -> Result<DeleteResult, DbErr> {
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
}
