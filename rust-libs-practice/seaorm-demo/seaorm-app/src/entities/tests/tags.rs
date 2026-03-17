use crate::entities::{post_tags, posts, tags, users};
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{Set, Unchanged},
    Database, DbConn, DbErr, DeleteResult, EntityTrait, IntoActiveModel, QueryFilter, QueryOrder,
    QuerySelect,
};

struct Mutation;

impl Mutation {
    async fn create_tag(db: &DbConn, model: tags::Model) -> Result<tags::ActiveModel, DbErr> {
        let active = model.into_active_model();
        let inserted: tags::Model = active.insert(db).await?;
        Ok(inserted.into())
    }

    async fn update_tag_by_id(
        db: &DbConn,
        id: i64,
        model: tags::Model,
    ) -> Result<tags::Model, DbErr> {
        let active = tags::ActiveModel {
            id: Unchanged(id),
            name: Set(model.name),
        };
        Ok(active.update(db).await?)
    }

    async fn delete_tag(db: &DbConn, id: i64) -> Result<DeleteResult, DbErr> {
        Ok(tags::Entity::delete_by_id(id).exec(db).await?)
    }

    async fn delete_all_tags(db: &DbConn) -> Result<DeleteResult, DbErr> {
        Ok(tags::Entity::delete_many().exec(db).await?)
    }
}

struct Query;

impl Query {
    async fn find_all_tag_names(db: &DbConn) -> Result<Vec<String>, DbErr> {
        let tags = tags::Entity::find()
            .select_only()
            .column(tags::Column::Name)
            .order_by_asc(tags::Column::Name)
            .into_tuple::<String>()
            .all(db)
            .await?;

        Ok(tags)
    }

    async fn find_tag_by_id(db: &DbConn, id: i64) -> Result<Option<tags::Model>, DbErr> {
        tags::Entity::find_by_id(id).one(db).await
    }

    async fn find_tags_with_posts(
        db: &DbConn,
    ) -> Result<Vec<(tags::Model, Vec<posts::Model>)>, DbErr> {
        let tags_with_posts = tags::Entity::find()
            .find_with_related(posts::Entity)
            .all(db)
            .await?;

        Ok(tags_with_posts)
    }
}

#[tokio::test]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = &Database::connect("sqlite::memory:").await.unwrap();

    db.get_schema_builder()
        .register(users::Entity)
        .register(posts::Entity)
        .register(tags::Entity)
        .register(post_tags::Entity)
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
        title: "Title A".to_owned(),
        content: "Text A".to_owned(),
        created_at: test_now,
    }
    .into_active_model()
    .insert(db)
    .await?;

    let tag = Mutation::create_tag(
        db,
        tags::Model {
            id: 1,
            name: "rust".to_owned(),
        },
    )
    .await?;

    post_tags::Model {
        post_id: 1,
        tag_id: 1,
    }
    .into_active_model()
    .insert(db)
    .await?;

    assert_eq!(
        tag,
        tags::ActiveModel {
            id: Unchanged(1),
            name: Unchanged("rust".to_owned()),
        }
    );

    Mutation::create_tag(
        db,
        tags::Model {
            id: 2,
            name: "seaorm".to_owned(),
        },
    )
    .await?;

    let tag = Query::find_tag_by_id(db, 1).await?.unwrap();
    assert_eq!(tag.id, 1);
    assert_eq!(tag.name, "rust");

    let tag = Mutation::update_tag_by_id(
        db,
        1,
        tags::Model {
            id: 1,
            name: "rust-lang".to_owned(),
        },
    )
    .await?;

    assert_eq!(
        tag,
        tags::Model {
            id: 1,
            name: "rust-lang".to_owned(),
        }
    );

    let tag_names = Query::find_all_tag_names(db).await?;

    assert_eq!(tag_names, vec!["rust-lang", "seaorm"]);

    let tags_with_posts = Query::find_tags_with_posts(db).await?;

    assert_eq!(
        tags_with_posts,
        vec![
            (
                tags::Model {
                    id: 1,
                    name: "rust-lang".to_owned(),
                },
                vec![posts::Model {
                    id: 1,
                    user_id: 1,
                    title: "Title A".to_owned(),
                    content: "Text A".to_owned(),
                    created_at: test_now,
                }]
            ),
            (
                tags::Model {
                    id: 2,
                    name: "seaorm".to_owned(),
                },
                vec![]
            )
        ]
    );

    let result = Mutation::delete_tag(db, 2).await?;
    assert_eq!(result.rows_affected, 1);

    let tag = Query::find_tag_by_id(db, 2).await?;
    assert!(tag.is_none());

    let result = Mutation::delete_all_tags(db).await?;
    assert_eq!(result.rows_affected, 1);

    Ok(())
}
