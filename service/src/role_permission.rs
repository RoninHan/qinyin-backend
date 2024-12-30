use ::entity::{role_permission, role_permission::Entity as RolePermission};
use sea_orm::*;

pub struct RolePermissionServices;

impl RolePermissionServices {
    pub async fn create_role_permission(
        db: &DbConn,
        form_data: role_permission::Model,
    ) -> Result<role_permission::ActiveModel, DbErr> {
        role_permission::ActiveModel {
            role_id: Set(form_data.role_id),
            permission_id: Set(form_data.permission_id),
            created_at: Set(chrono::Utc::now().naive_utc()),
            updated_at: Set(chrono::Utc::now().naive_utc()),
            ..Default::default()
        }
        .save(db)
        .await
    }

    pub async fn delete_role_permission_by_id(db: &DbConn, id: i64) -> Result<(), DbErr> {
        RolePermission::delete_many()
            .filter(role_permission::Column::Id.eq(id))
            .exec(db)
            .await?;
        Ok(())
    }
}
