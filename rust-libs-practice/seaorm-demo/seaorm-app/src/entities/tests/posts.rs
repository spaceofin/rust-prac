use crate::entities::{posts, users};
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{Set, Unchanged},
    Database, DbConn, DbErr, DeleteResult, EntityTrait, IntoActiveModel,
};

struct Mutation;

impl Mutation {
    async fn create_post(db: &DbConn, model: posts::Model) -> Result<posts::ActiveModel, DbErr> {
        let active = model.into_active_model();
        let inserted_post: posts::Model = active.insert(db).await?;
        Ok(inserted_post.into())
    }

    async fn update_post_by_id(
        db: &DbConn,
        id: i64,
        model: posts::Model,
    ) -> Result<posts::Model, DbErr> {
        let active = posts::ActiveModel {
            id: Unchanged(id),
            user_id: Set(model.user_id),
            title: Set(model.title),
            content: Set(model.content),
            created_at: Set(model.created_at),
        };
        let updated_post = active.update(db).await?;
        Ok(updated_post)
    }

    async fn delete_post(db: &DbConn, id: i64) -> Result<DeleteResult, DbErr> {
        let result = posts::Entity::delete_by_id(id).exec(db).await?;
        Ok(result)
    }

    async fn delete_all_posts(db: &DbConn) -> Result<DeleteResult, DbErr> {
        let result = posts::Entity::delete_many().exec(db).await?;
        Ok(result)
    }
}

struct Query;

impl Query {
    async fn find_post_by_id(db: &DbConn, id: i64) -> Result<Option<posts::Model>, DbErr> {
        posts::Entity::find_by_id(id).one(db).await
    }
}

#[tokio::test]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // create in memory database
    let db = &Database::connect("sqlite::memory:").await.unwrap();

    // setup schema
    db.get_schema_builder()
        .register(posts::Entity)
        .register(users::Entity)
        .apply(db)
        .await?;

    let test_now = Utc::now().naive_utc();

    users::Model {
        id: 1,
        username: "User 1".to_owned(),
        created_at: test_now,
    }
    .into_active_model()
    .insert(db)
    .await?;

    let post = Mutation::create_post(
        db,
        posts::Model {
            id: 1,
            user_id: 1,
            title: "Title A".to_owned(),
            content: "Text A".to_owned(),
            created_at: test_now,
        },
    )
    .await
    .unwrap();

    assert_eq!(
        post,
        posts::ActiveModel {
            id: Unchanged(1),
            user_id: Unchanged(1),
            title: Unchanged("Title A".to_owned()),
            content: Unchanged("Text A".to_owned()),
            created_at: Unchanged(test_now),
        }
    );

    Mutation::create_post(
        db,
        posts::Model {
            id: 2,
            user_id: 1,
            title: "Title B".to_owned(),
            content: "Text B".to_owned(),
            created_at: test_now,
        },
    )
    .await
    .unwrap();

    Mutation::create_post(
        db,
        posts::Model {
            id: 3,
            user_id: 1,
            title: "Title C".to_owned(),
            content: "Text C".to_owned(),
            created_at: test_now,
        },
    )
    .await
    .unwrap();

    let post = Query::find_post_by_id(db, 1).await.unwrap().unwrap();

    assert_eq!(post.id, 1);
    assert_eq!(post.title, "Title A");

    let post = Mutation::update_post_by_id(
        db,
        1,
        posts::Model {
            id: 1,
            user_id: 1,
            title: "New Title A".to_owned(),
            content: "New Text A".to_owned(),
            created_at: test_now,
        },
    )
    .await
    .unwrap();

    assert_eq!(
        post,
        posts::Model {
            id: 1,
            user_id: 1,
            title: "New Title A".to_owned(),
            content: "New Text A".to_owned(),
            created_at: test_now,
        }
    );

    let result = Mutation::delete_post(db, 2).await.unwrap();
    assert_eq!(result.rows_affected, 1);

    let post = Query::find_post_by_id(db, 2).await.unwrap();
    assert!(post.is_none());

    let result = Mutation::delete_all_posts(db).await.unwrap();
    assert_eq!(result.rows_affected, 2);

    Ok(())
}
