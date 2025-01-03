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
                    .col(pk_auto(Post::Id))
                    .col(User::Name, Varchar(255).not_null())
                    .col(User::AppId, Varchar(255).not_null())
                    .col(User::Sex, Varchar(255).not_null())
                    .col(User::Birthday, Date)
                    .col(User::Phone, Varchar(255).not_null())
                    .col(User::Email, Varchar(255).not_null())
                    .col(User::CreatedAt, Datetime)
                    .col(User::UpdatedAt, Datetime)
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Song::Table)
                    .if_not_exists()
                    .col(pk_auto(Song::Id))
                    .col(Song::Name, Varchar(255).not_null())
                    .col(Song::Author, Varchar(255).not_null())
                    .col(Song::SongTypeId, Integer)
                    .col(Song::Singer, Varchar(255).not_null())
                    .col(Song::CreatedAt, Datetime)
                    .col(Song::UpdatedAt, Datetime)
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(SongType::Table)
                    .if_not_exists()
                    .col(pk_auto(SongType::Id))
                    .col(SongType::Name, Varchar(255).not_null())
                    .col(SongType::CreatedAt, Datetime)
                    .col(SongType::UpdatedAt, Datetime)
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Score::Table)
                    .if_not_exists()
                    .col(pk_auto(Score::Id))
                    .col(Score::UserId, Integer)
                    .col(Score::SongId, Integer)
                    .col(Score::Score, Integer)
                    .col(Score::CreatedAt, Datetime)
                    .col(Score::UpdatedAt, Datetime)
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Lyric::Table)
                    .if_not_exists()
                    .col(pk_auto(Lyric::Id))
                    .col(Lyric::SongId, Integer)
                    .col(Lyric::Lyric, Text)
                    .col(Lyric::CreatedAt, Datetime)
                    .col(Lyric::UpdatedAt, Datetime)
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Friends::Table)
                    .if_not_exists()
                    .col(pk_auto(Friends::Id))
                    .col(Friends::UserId, Integer)
                    .col(Friends::FriendUserId, Integer)
                    .col(Friends::CreatedAt, Datetime)
                    .col(Friends::UpdatedAt, Datetime)
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Creation::Table)
                    .if_not_exists()
                    .col(pk_auto(Creation::Id))
                    .col(Creation::UserId, Integer)
                    .col(Creation::SongId, Integer)
                    .col(Creation::CreatedAt, Datetime)
                    .col(Creation::UpdatedAt, Datetime)
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Collect::Table)
                    .if_not_exists()
                    .col(pk_auto(Collect::Id))
                    .col(Collect::UserId, Integer)
                    .col(Collect::SongId, Integer)
                    .col(Collect::CreatedAt, Datetime)
                    .col(Collect::UpdatedAt, Datetime)
                    .to_owned(),
            )
            .await?;
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
    SongId,
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
