use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(pk_auto(User::Id))
                    .col(ColumnDef::new(User::Name).string().not_null())
                    .col(ColumnDef::new(User::AppId).string().not_null())
                    .col(ColumnDef::new(User::Sex).integer().not_null())
                    .col(ColumnDef::new(User::Birthday).timestamp_with_time_zone())
                    .col(ColumnDef::new(User::Phone).string())
                    .col(ColumnDef::new(User::Email).string())
                    .col(
                        ColumnDef::new(User::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(User::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Song::Table)
                    .if_not_exists()
                    .col(pk_auto(Song::Id))
                    .col(ColumnDef::new(Song::Name).string().not_null())
                    .col(ColumnDef::new(Song::Author).string().not_null())
                    .col(ColumnDef::new(Song::SongTypeId).integer())
                    .col(ColumnDef::new(Song::Singer).string().not_null())
                    .col(
                        ColumnDef::new(User::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(User::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(SongType::Table)
                    .if_not_exists()
                    .col(pk_auto(SongType::Id))
                    .col(ColumnDef::new(SongType::Name).string().not_null())
                    .col(
                        ColumnDef::new(User::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(User::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Score::Table)
                    .if_not_exists()
                    .col(pk_auto(Score::Id))
                    .col(ColumnDef::new(Score::UserId).integer().not_null())
                    .col(ColumnDef::new(Score::SongId).integer().not_null())
                    .col(ColumnDef::new(Score::Score).integer().not_null())
                    .col(
                        ColumnDef::new(User::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(User::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Lyric::Table)
                    .if_not_exists()
                    .col(pk_auto(Lyric::Id))
                    .col(ColumnDef::new(Lyric::SongId).integer().not_null())
                    .col(ColumnDef::new(Lyric::Lyric).string().not_null())
                    .col(
                        ColumnDef::new(User::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(User::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Friends::Table)
                    .if_not_exists()
                    .col(pk_auto(Friends::Id))
                    .col(ColumnDef::new(Friends::UserId).integer().not_null())
                    .col(ColumnDef::new(Friends::FriendUserId).integer().not_null())
                    .col(
                        ColumnDef::new(User::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(User::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Creation::Table)
                    .if_not_exists()
                    .col(pk_auto(Creation::Id))
                    .col(ColumnDef::new(Creation::UserId).integer().not_null())
                    .col(ColumnDef::new(Creation::SongSrc).string().not_null())
                    .col(ColumnDef::new(Creation::Name).string().not_null())
                    .col(
                        ColumnDef::new(User::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(User::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Collect::Table)
                    .if_not_exists()
                    .col(pk_auto(Collect::Id))
                    .col(ColumnDef::new(Collect::UserId).integer().not_null())
                    .col(ColumnDef::new(Collect::SongId).integer().not_null())
                    .col(
                        ColumnDef::new(User::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(User::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Song::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(SongType::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Score::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Lyric::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Friends::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Creation::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Collect::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    Name,
    AppId,
    Sex,
    Birthday,
    Phone,
    Email,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Song {
    Table,
    Id,
    Name,
    Author,
    SongTypeId,
    Singer,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum SongType {
    Table,
    Id,
    Name,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Score {
    Table,
    Id,
    UserId,
    SongId,
    Score,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Lyric {
    Table,
    Id,
    SongId,
    Lyric,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Friends {
    Table,
    Id,
    UserId,
    FriendUserId,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Creation {
    Table,
    Id,
    UserId,
    SongSrc,
    Name,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Collect {
    Table,
    Id,
    UserId,
    SongId,
    CreatedAt,
    UpdatedAt,
}
