// @generated automatically by Diesel CLI.

diesel::table! {
    articles (id) {
        id -> Integer,
        title -> Text,
        body -> Text,
        draft -> Bool,
        publish_at -> Timestamp,
        visit_count -> Integer,
    }
}

diesel::table! {
    posts (id) {
        id -> Integer,
        title -> Text,
        body -> Text,
        published -> Bool,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        name -> Text,
        hair_color -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(articles, posts, users,);
