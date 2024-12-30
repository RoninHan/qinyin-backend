use ::entity::{role, role::Entity as Role};
use sea_orm::*;

pub struct RoleServices;

impl RoleServices {
    pub async fn create_role(
        db: &DbConn,
        form_data: role::Model,
    ) -> Result<role::ActiveModel, DbErr> {
        role::ActiveModel {
            name: Set(form_data.name.to_owned()),
            status: Set(String::from("active")),
            description: Set(form_data.description.to_owned()),
            created_at: Set(chrono::Utc::now().naive_utc()),
            updated_at: Set(chrono::Utc::now().naive_utc()),
            ..Default::default()
        }
        .save(db)
        .await
    }

    pub async fn update_role_by_id(
        db: &DbConn,
        id: i64,
        form_data: role::Model,
    ) -> Result<role::Model, DbErr> {
        let role: role::ActiveModel = Role::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find role.".to_owned()))
            .map(Into::into)?;

        role::ActiveModel {
            id: role.id,
            name: Set(form_data.name.to_owned()),
            description: Set(form_data.description.to_owned()),
            updated_at: Set(chrono::Utc::now().naive_utc()),
            ..Default::default()
        }
        .update(db)
        .await
    }

    pub async fn delete_role_by_id(db: &DbConn, id: i64) -> Result<(), DbErr> {
        Role::delete_many()
            .filter(role::Column::Id.eq(id))
            .exec(db)
            .await?;
        Ok(())
    }
}
