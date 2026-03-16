use crate::entities::{comments, posts, users};
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{Set, Unchanged},
    Database, DbConn, DbErr, DeleteResult, EntityTrait, IntoActiveModel, PaginatorTrait,
    QueryFilter,
};

struct Mutation;

impl Mutation {
    async fn create_comment(
        db: &DbConn,
        model: comments::Model,
    ) -> Result<comments::ActiveModel, DbErr> {
        let active = model.into_active_model();
        let inserted: comments::Model = active.insert(db).await?;
        Ok(inserted.into())
    }

    async fn update_comment_by_id(
        db: &DbConn,
        id: i64,
        model: comments::Model,
    ) -> Result<comments::Model, DbErr> {
        let active = comments::ActiveModel {
            id: Unchanged(id),
            post_id: Set(model.post_id),
            user_id: Set(model.user_id),
            text: Set(model.text),
            created_at: Set(model.created_at),
        };
        Ok(active.update(db).await?)
    }

    async fn delete_comment(db: &DbConn, id: i64) -> Result<DeleteResult, DbErr> {
        Ok(comments::Entity::delete_by_id(id).exec(db).await?)
    }

    async fn delete_all_comments(db: &DbConn) -> Result<DeleteResult, DbErr> {
        Ok(comments::Entity::delete_many().exec(db).await?)
    }
}

struct Query;

impl Query {
    async fn find_comment_by_id(db: &DbConn, id: i64) -> Result<Option<comments::Model>, DbErr> {
        comments::Entity::find_by_id(id).one(db).await
    }

    async fn find_comments_by_user_id(
        db: &DbConn,
        user_id: i64,
    ) -> Result<Vec<comments::Model>, DbErr> {
        comments::Entity::find()
            .filter(comments::Entity::COLUMN.user_id.eq(user_id))
            .all(db)
            .await
    }

    async fn get_comment_count_by_post_id(db: &DbConn, post_id: i64) -> Result<u64, DbErr> {
        let count = comments::Entity::find()
            .filter(comments::Entity::COLUMN.post_id.eq(post_id))
            .count(db)
            .await?;
        Ok(count)
    }
}

#[tokio::test]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = &Database::connect("sqlite::memory:").await.unwrap();

    db.get_schema_builder()
        .register(comments::Entity)
        .register(posts::Entity)
        .register(users::Entity)
        .apply(db)
        .await?;

    let test_now = Utc::now().naive_utc();

    let users = vec![
        users::Model {
            id: 1,
            username: "User 1".to_owned(),
            created_at: test_now,
        }
        .into_active_model(),
        users::Model {
            id: 2,
            username: "User 2".to_owned(),
            created_at: test_now,
        }
        .into_active_model(),
    ];
    users::Entity::insert_many(users).exec(db).await?;

    posts::Model {
        id: 1,
        user_id: 1,
        title: "Post 1".to_owned(),
        content: "Content 1".to_owned(),
        created_at: test_now,
    }
    .into_active_model()
    .insert(db)
    .await?;

    let comment = Mutation::create_comment(
        db,
        comments::Model {
            id: 1,
            post_id: 1,
            user_id: 1,
            text: "Nice post".to_owned(),
            created_at: test_now,
        },
    )
    .await?;

    assert_eq!(
        comment,
        comments::ActiveModel {
            id: Unchanged(1),
            post_id: Unchanged(1),
            user_id: Unchanged(1),
            text: Unchanged("Nice post".to_owned()),
            created_at: Unchanged(test_now),
        }
    );

    Mutation::create_comment(
        db,
        comments::Model {
            id: 2,
            post_id: 1,
            user_id: 2,
            text: "Good post".to_owned(),
            created_at: test_now,
        },
    )
    .await?;

    let comment = Query::find_comment_by_id(db, 1).await?.unwrap();
    assert_eq!(comment.id, 1);
    assert_eq!(comment.text, "Nice post");

    let comment = Mutation::update_comment_by_id(
        db,
        1,
        comments::Model {
            id: 1,
            post_id: 1,
            user_id: 1,
            text: "Nice post - Edited".to_owned(),
            created_at: test_now,
        },
    )
    .await?;

    assert_eq!(
        comment,
        comments::Model {
            id: 1,
            post_id: 1,
            user_id: 1,
            text: "Nice post - Edited".to_owned(),
            created_at: test_now,
        }
    );

    let comment_count = Query::get_comment_count_by_post_id(db, 1).await?;
    assert_eq!(comment_count, 2);

    let comments = Query::find_comments_by_user_id(db, 2).await?;
    assert_eq!(
        comments,
        vec![comments::Model {
            id: 2,
            post_id: 1,
            user_id: 2,
            text: "Good post".to_owned(),
            created_at: test_now,
        },]
    );

    let result = Mutation::delete_comment(db, 2).await?;
    assert_eq!(result.rows_affected, 1);

    let comment = Query::find_comment_by_id(db, 2).await?;
    assert!(comment.is_none());

    let comments = Query::find_comments_by_user_id(db, 2).await?;
    assert!(comments.is_empty());

    let result = Mutation::delete_all_comments(db).await?;
    assert_eq!(result.rows_affected, 1);

    let comment_count = Query::get_comment_count_by_post_id(db, 1).await?;
    assert_eq!(comment_count, 0);

    Ok(())
}
