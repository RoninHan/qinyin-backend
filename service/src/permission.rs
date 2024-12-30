use ::entity::{permission, permission::Entity as Permission};
use sea_orm::*;

pub struct PermissionServices;

impl PermissionServices {
    pub async fn create_permission(
        db: &DbConn,
        form_data: permission::Model,
    ) -> Result<permission::ActiveModel, DbErr> {
        permission::ActiveModel {
            title: Set(form_data.title.to_owned()),
            action: Set(form_data.action.to_owned()),
            status: Set(String::from("active")),
            created_at: Set(chrono::Utc::now().naive_utc()),
            updated_at: Set(chrono::Utc::now().naive_utc()),
            ..Default::default()
        }
        .save(db)
        .await
    }

    pub async fn update_permission_by_id(
        db: &DbConn,
        id: i64,
        form_data: permission::Model,
    ) -> Result<permission::Model, DbErr> {
        let permission: permission::ActiveModel = Permission::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find permission.".to_owned()))
            .map(Into::into)?;

        permission::ActiveModel {
            id: permission.id,
            title: Set(form_data.title.to_owned()),
            action: Set(form_data.action.to_owned()),
            updated_at: Set(chrono::Utc::now().naive_utc()),
            ..Default::default()
        }
        .update(db)
        .await
    }

    pub async fn delete_permission_by_id(db: &DbConn, id: i64) -> Result<(), DbErr> {
        Permission::delete_many()
            .filter(permission::Column::Id.eq(id))
            .exec(db)
            .await?;
        Ok(())
    }
}
