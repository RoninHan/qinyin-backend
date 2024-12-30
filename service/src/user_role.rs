use ::entity::{user_role, user_role::Entity as UserRole};
use sea_orm::*;

pub struct UserRoleServices;

impl UserRoleServices {
    pub async fn create_user_role(
        db: &DbConn,
        form_data: user_role::Model,
    ) -> Result<user_role::ActiveModel, DbErr> {
        user_role::ActiveModel {
            user_id: Set(form_data.user_id),
            role_id: Set(form_data.role_id),
            created_at: Set(chrono::Utc::now().naive_utc()),
            updated_at: Set(chrono::Utc::now().naive_utc()),
            ..Default::default()
        }
        .save(db)
        .await
    }

    pub async fn delete_user_role_by_id(db: &DbConn, id: i64) -> Result<(), DbErr> {
        UserRole::delete_many()
            .filter(user_role::Column::Id.eq(id))
            .exec(db)
            .await?;
        Ok(())
    }
}
