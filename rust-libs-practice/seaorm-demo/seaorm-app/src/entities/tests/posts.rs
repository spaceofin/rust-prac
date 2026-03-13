use crate::entities::{posts, users};
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, Database, EntityTrait, IntoActiveModel};

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

    let post: posts::Model = posts::Model {
        id: 1,
        user_id: 1,
        title: "Title A".to_owned(),
        content: "Text A".to_owned(),
        created_at: test_now,
    }
    .into_active_model()
    .insert(db)
    .await?;

    assert_eq!(
        post,
        posts::Model {
            id: 1,
            user_id: 1,
            title: "Title A".to_owned(),
            content: "Text A".to_owned(),
            created_at: test_now,
        }
    );

    posts::Model {
        id: 2,
        user_id: 1,
        title: "Title B".to_owned(),
        content: "Text B".to_owned(),
        created_at: test_now,
    }
    .into_active_model()
    .insert(db)
    .await?;

    posts::Model {
        id: 3,
        user_id: 1,
        title: "Title C".to_owned(),
        content: "Text C".to_owned(),
        created_at: test_now,
    }
    .into_active_model()
    .insert(db)
    .await?;

    let post = posts::Entity::find_by_id(1).one(db).await.unwrap().unwrap();

    assert_eq!(post.id, 1);
    assert_eq!(post.title, "Title A");

    let mut post: posts::ActiveModel = post.into();

    post.title = Set("New Title A".to_owned());
    post.content = Set("New Text A".to_owned());

    let post: posts::Model = post.update(db).await?;

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

    let result = posts::Entity::delete_by_id(2).exec(db).await.unwrap();
    assert_eq!(result.rows_affected, 1);

    let post = posts::Entity::find_by_id(2).one(db).await.unwrap();
    assert!(post.is_none());

    let result = posts::Entity::delete_many().exec(db).await.unwrap();
    assert_eq!(result.rows_affected, 2);

    Ok(())
}
