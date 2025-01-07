use ::entity::{setting, setting::Entity as Setting};
use sea_orm::*;

pub struct SettingService;

impl SettingService {
    pub async fn get_setting(db: &DbConn, id: i32) -> Result<setting::Model, DbErr> {
        let setting: setting::Model = Setting::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find setting.".to_owned()))?;
        Ok(setting)
    }

    pub async fn create_setting(
        db: &DbConn,
        device_id: Option<String>,
    ) -> Result<setting::ActiveModel, DbErr> {
        setting::ActiveModel {
            device_id: Set(device_id),
            ..Default::default()
        }
        .save(db)
        .await
    }

    pub async fn update_setting_by_id(
        db: &DbConn,
        id: i32,
        device_id: Option<String>,
    ) -> Result<setting::Model, DbErr> {
        let setting: setting::ActiveModel = Setting::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find setting.".to_owned()))
            .map(Into::into)?;

        setting::ActiveModel {
            id: setting.id,
            device_id: Set(device_id),
            ..Default::default()
        }
        .update(db)
        .await
    }
}
