use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 1. Users table create
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    // .col(pk_auto(User::Id))
                    .col(integer(Users::Id).not_null().primary_key())
                    .col(
                        string(Users::Username)
                            .not_null()
                            .unique_key()
                            .extra("COLLATE NOCASE"),
                    )
                    .col(
                        date_time(Users::CreatedAt)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        // 2. Posts table create
        manager
            .create_table(
                Table::create()
                    .table(Posts::Table)
                    .if_not_exists()
                    // .col(pk_auto(Post::Id))
                    .col(integer(Posts::Id).not_null().primary_key())
                    .col(integer(Posts::UserId).not_null())
                    .col(string(Posts::Title).not_null())
                    .col(text(Posts::Content).not_null())
                    .col(
                        date_time(Posts::CreatedAt)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Posts::Table, Posts::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // 3. Comments table create
        manager
            .create_table(
                Table::create()
                    .table(Comments::Table)
                    .if_not_exists()
                    // .col(pk_auto(Comments::Id))
                    .col(integer(Comments::Id).not_null().primary_key())
                    .col(integer(Comments::PostId).not_null())
                    .col(integer(Comments::UserId).not_null())
                    .col(text(Comments::Text).not_null())
                    .col(
                        date_time(Comments::CreatedAt)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Comments::Table, Comments::PostId)
                            .to(Posts::Table, Posts::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Comments::Table, Comments::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // 4. Tags table create
        manager
            .create_table(
                Table::create()
                    .table(Tags::Table)
                    .if_not_exists()
                    // .col(pk_auto(Tags::Id))
                    .col(integer(Tags::Id).not_null().primary_key())
                    .col(
                        string(Tags::Name)
                            .not_null()
                            .unique_key()
                            .extra("COLLATE NOCASE"),
                    )
                    .to_owned(),
            )
            .await?;

        // 5. PostTags table create
        manager
            .create_table(
                Table::create()
                    .table(PostTags::Table)
                    .if_not_exists()
                    .col(integer(PostTags::PostId).not_null())
                    .col(integer(PostTags::TagId).not_null())
                    .primary_key(Index::create().col(PostTags::PostId).col(PostTags::TagId))
                    .foreign_key(
                        ForeignKey::create()
                            .from(PostTags::Table, PostTags::PostId)
                            .to(Posts::Table, Posts::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(PostTags::Table, PostTags::TagId)
                            .to(Tags::Table, Tags::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 1. PostTags table drop
        manager
            .drop_table(Table::drop().table(PostTags::Table).if_exists().to_owned())
            .await?;

        // 2. Comments table drop
        manager
            .drop_table(Table::drop().table(Comments::Table).if_exists().to_owned())
            .await?;

        // 3. Posts table drop
        manager
            .drop_table(Table::drop().table(Posts::Table).if_exists().to_owned())
            .await?;

        // 4. Tags table drop
        manager
            .drop_table(Table::drop().table(Tags::Table).if_exists().to_owned())
            .await?;

        // 5. Users table drop
        manager
            .drop_table(Table::drop().table(Users::Table).if_exists().to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
    Username,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Posts {
    Table,
    Id,
    UserId,
    Title,
    Content,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Comments {
    Table,
    Id,
    PostId,
    UserId,
    Text,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Tags {
    Table,
    Id,
    Name,
}

#[derive(DeriveIden)]
enum PostTags {
    Table,
    PostId,
    TagId,
}
