use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 1. User table create
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    // .col(pk_auto(User::Id))
                    .col(integer(User::Id).not_null().primary_key())
                    .col(string(User::Username).not_null().unique_key())
                    .col(
                        date_time(User::CreatedAt)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        // 2. Post table create
        manager
            .create_table(
                Table::create()
                    .table(Post::Table)
                    .if_not_exists()
                    // .col(pk_auto(Post::Id))
                    .col(integer(Post::Id).not_null().primary_key())
                    .col(integer(Post::UserId).not_null())
                    .col(string(Post::Title).not_null())
                    .col(string(Post::Content).not_null())
                    .col(
                        date_time(Post::CreatedAt)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Post::Table, Post::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // 3. Comment table create
        manager
            .create_table(
                Table::create()
                    .table(Comment::Table)
                    .if_not_exists()
                    // .col(pk_auto(Comment::Id))
                    .col(integer(Comment::Id).not_null().primary_key())
                    .col(integer(Comment::PostId).not_null())
                    .col(integer(Comment::UserId).not_null())
                    .col(string(Comment::Text).not_null())
                    .col(
                        date_time(Comment::CreatedAt)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Comment::Table, Comment::PostId)
                            .to(Post::Table, Post::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Comment::Table, Comment::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // 4. Tag table create
        manager
            .create_table(
                Table::create()
                    .table(Tag::Table)
                    .if_not_exists()
                    // .col(pk_auto(Tag::Id))
                    .col(integer(Tag::Id).not_null().primary_key())
                    .col(string(Tag::Name).not_null().unique_key())
                    .to_owned(),
            )
            .await?;

        // 5. PostTag table create
        manager
            .create_table(
                Table::create()
                    .table(PostTag::Table)
                    .if_not_exists()
                    .col(integer(PostTag::PostId).not_null())
                    .col(integer(PostTag::TagId).not_null())
                    .primary_key(Index::create().col(PostTag::PostId).col(PostTag::TagId))
                    .foreign_key(
                        ForeignKey::create()
                            .from(PostTag::Table, PostTag::PostId)
                            .to(Post::Table, Post::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(PostTag::Table, PostTag::TagId)
                            .to(Tag::Table, Tag::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 1. PostTag table drop
        manager
            .drop_table(Table::drop().table(PostTag::Table).if_exists().to_owned())
            .await?;

        // 2. Comment table drop
        manager
            .drop_table(Table::drop().table(Comment::Table).if_exists().to_owned())
            .await?;

        // 3. Post table drop
        manager
            .drop_table(Table::drop().table(Post::Table).if_exists().to_owned())
            .await?;

        // 4. Tag table drop
        manager
            .drop_table(Table::drop().table(Tag::Table).if_exists().to_owned())
            .await?;

        // 5. User table drop
        manager
            .drop_table(Table::drop().table(User::Table).if_exists().to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    Username,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Post {
    Table,
    Id,
    UserId,
    Title,
    Content,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Comment {
    Table,
    Id,
    PostId,
    UserId,
    Text,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Tag {
    Table,
    Id,
    Name,
}

#[derive(DeriveIden)]
enum PostTag {
    Table,
    PostId,
    TagId,
}
