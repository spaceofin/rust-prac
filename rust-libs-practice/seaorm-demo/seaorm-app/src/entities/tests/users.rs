use crate::entities::users;
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{Set, Unchanged},
    Database, DbConn, DbErr, DeleteResult, EntityTrait, IntoActiveModel, QueryFilter,
};

struct Mutation;

impl Mutation {
    async fn create_user(db: &DbConn, model: users::Model) -> Result<users::ActiveModel, DbErr> {
        let active = model.into_active_model();
        let inserted_user: users::Model = active.insert(db).await?;
        Ok(inserted_user.into())
    }

    async fn update_user_by_id(
        db: &DbConn,
        id: i64,
        model: users::Model,
    ) -> Result<users::Model, DbErr> {
        let active = users::ActiveModel {
            id: Unchanged(id),
            username: Set(model.username),
            created_at: Set(model.created_at),
        };
        let updated_user = active.update(db).await?;
        Ok(updated_user)
    }

    async fn delete_user(db: &DbConn, id: i64) -> Result<DeleteResult, DbErr> {
        let result = users::Entity::delete_by_id(id).exec(db).await?;
        Ok(result)
    }

    async fn delete_all_users(db: &DbConn) -> Result<DeleteResult, DbErr> {
        let result = users::Entity::delete_many().exec(db).await?;
        Ok(result)
    }
}

struct Query;

impl Query {
    async fn find_user_by_id(db: &DbConn, id: i64) -> Result<Option<users::Model>, DbErr> {
        users::Entity::find_by_id(id).one(db).await
    }
    async fn find_user_by_username(
        db: &DbConn,
        username: String,
    ) -> Result<Option<users::Model>, DbErr> {
        users::Entity::find()
            .filter(users::Entity::COLUMN.username.eq(username))
            .one(db)
            .await
    }
}

#[tokio::test]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // create in memory database
    let db = &Database::connect("sqlite::memory:").await.unwrap();

    // setup schema
    db.get_schema_builder()
        .register(users::Entity)
        .apply(db)
        .await?;

    let test_now = Utc::now().naive_utc();

    let user = Mutation::create_user(
        db,
        users::Model {
            id: 1,
            username: "User 1".to_owned(),
            created_at: test_now,
        },
    )
    .await
    .unwrap();

    assert_eq!(
        user,
        users::ActiveModel {
            id: Unchanged(1),
            username: Unchanged("User 1".to_owned()),
            created_at: Unchanged(test_now),
        }
    );

    Mutation::create_user(
        db,
        users::Model {
            id: 2,
            username: "User 2".to_owned(),
            created_at: test_now,
        },
    )
    .await
    .unwrap();

    Mutation::create_user(
        db,
        users::Model {
            id: 3,
            username: "User 3".to_owned(),
            created_at: test_now,
        },
    )
    .await
    .unwrap();

    let user = Query::find_user_by_id(db, 1).await.unwrap().unwrap();
    assert_eq!(user.id, 1);
    assert_eq!(user.username, "User 1");

    let user = Query::find_user_by_username(db, "User 2".into())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(user.id, 2);
    assert_eq!(user.username, "User 2");

    let user = Query::find_user_by_username(db, "User".into())
        .await
        .unwrap();
    assert!(user.is_none());

    let user = Mutation::update_user_by_id(
        db,
        1,
        users::Model {
            id: 1,
            username: "New User 1".to_owned(),
            created_at: test_now,
        },
    )
    .await
    .unwrap();

    assert_eq!(
        user,
        users::Model {
            id: 1,
            username: "New User 1".to_owned(),
            created_at: test_now,
        }
    );

    let result = Mutation::delete_user(db, 2).await.unwrap();
    assert_eq!(result.rows_affected, 1);

    let user = Query::find_user_by_id(db, 2).await.unwrap();
    assert!(user.is_none());

    let result = Mutation::delete_all_users(db).await.unwrap();
    assert_eq!(result.rows_affected, 2);

    Ok(())
}
