use crate::entities::{post_tags, posts, tags, users};
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Unchanged, ColumnTrait, Database, DbConn, DbErr, DeleteResult,
    EntityTrait, IntoActiveModel, PaginatorTrait, QueryFilter,
};

struct Mutation;

impl Mutation {
    async fn create_post_tag(
        db: &DbConn,
        model: post_tags::Model,
    ) -> Result<post_tags::ActiveModel, DbErr> {
        let active = model.into_active_model();
        let inserted: post_tags::Model = active.insert(db).await?;
        Ok(inserted.into())
    }

    async fn delete_post_tag(
        db: &DbConn,
        post_id: i64,
        tag_id: i64,
    ) -> Result<DeleteResult, DbErr> {
        Ok(post_tags::Entity::delete_by_id((post_id, tag_id))
            .exec(db)
            .await?)
    }

    async fn delete_all_post_tags(db: &DbConn) -> Result<DeleteResult, DbErr> {
        Ok(post_tags::Entity::delete_many().exec(db).await?)
    }

    async fn replace_tag_for_post(
        db: &DbConn,
        post_id: i64,
        old_tag_id: i64,
        new_tag_id: i64,
    ) -> Result<post_tags::ActiveModel, DbErr> {
        Self::delete_post_tag(db, post_id, old_tag_id).await?;
        let post_tag = Self::create_post_tag(
            db,
            post_tags::Model {
                post_id,
                tag_id: new_tag_id,
            },
        )
        .await?;
        Ok(post_tag)
    }
}

struct Query;

impl Query {
    async fn find_post_tag(
        db: &DbConn,
        post_id: i64,
        tag_id: i64,
    ) -> Result<Option<post_tags::Model>, DbErr> {
        post_tags::Entity::find_by_id((post_id, tag_id))
            .one(db)
            .await
    }

    async fn count_posts_by_tag_id(db: &DbConn, tag_id: i64) -> Result<u64, DbErr> {
        post_tags::Entity::find()
            .filter(post_tags::Column::TagId.eq(tag_id))
            .count(db)
            .await
    }
}

#[tokio::test]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = &Database::connect("sqlite::memory:").await.unwrap();

    db.get_schema_builder()
        .register(post_tags::Entity)
        .register(posts::Entity)
        .register(tags::Entity)
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

    tags::Model {
        id: 1,
        name: "rust".to_owned(),
    }
    .into_active_model()
    .insert(db)
    .await?;

    tags::Model {
        id: 2,
        name: "seaorm".to_owned(),
    }
    .into_active_model()
    .insert(db)
    .await?;

    let mapping = Mutation::create_post_tag(
        db,
        post_tags::Model {
            post_id: 1,
            tag_id: 1,
        },
    )
    .await?;

    assert_eq!(
        mapping,
        post_tags::ActiveModel {
            post_id: Unchanged(1),
            tag_id: Unchanged(1),
        }
    );

    let mapping = Query::find_post_tag(db, 1, 1).await?.unwrap();
    assert_eq!(mapping.post_id, 1);
    assert_eq!(mapping.tag_id, 1);

    let post_tag = Mutation::replace_tag_for_post(db, 1, 1, 2).await?;
    assert_eq!(
        post_tag,
        post_tags::ActiveModel {
            post_id: Unchanged(1),
            tag_id: Unchanged(2)
        }
    );

    let result = Query::find_post_tag(db, 1, 1).await?;
    assert!(result.is_none());

    let post_count = Query::count_posts_by_tag_id(db, 2).await?;
    assert_eq!(post_count, 1);

    let result = Mutation::delete_post_tag(db, 1, 2).await?;
    assert_eq!(result.rows_affected, 1);

    let result = Mutation::delete_all_post_tags(db).await?;
    assert_eq!(result.rows_affected, 0);

    Ok(())
}
