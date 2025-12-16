#![allow(dead_code)]
#![allow(unused_variables)]

use diesel::prelude::*;
use diesel_sqlite::*;
use self::schema::articles::dsl::*;
use diesel::{debug_query, sqlite::Sqlite, dsl::now};
use chrono::{NaiveDateTime, Utc};

#[derive(Queryable, Identifiable, AsChangeset, Debug)]
#[diesel(table_name = self::schema::articles)]
struct Article {
    id: i32,
    title: String,
    body: String,
    draft: bool,
    publish_at: NaiveDateTime,
    visit_count: i32,
}

fn update_basic(connection: &mut SqliteConnection) {
  let query1 = diesel::update(articles).set(draft.eq(false));
  // UPDATE `articles` SET `draft` = ? -- binds: [false]
  println!("{}", debug_query::<Sqlite, _>(&query1));

  let query2 = diesel::update(articles)
    .filter(publish_at.lt(now))
    .set(draft.eq(false));
  // UPDATE `articles` SET `draft` = ? WHERE (`articles`.`publish_at` < CURRENT_TIMESTAMP) -- binds: [false]
  println!("{}", debug_query::<Sqlite, _>(&query2));

  let article1 = articles.find(1)
    .first::<Article>(connection)
    .expect("no article with given id");
  println!("{article1:#?}");

  let mut updated_row_count = diesel::update(articles)
    .filter(id.le(5))
    .set(draft.eq(true))
    .execute(connection)
    .expect("failed to update articles");

  println!("updated_row_count: {updated_row_count:?}");
  
  let updated_articles = articles.filter(id.le(5))
    .load::<Article>(connection)
    .expect("failed to update articles");
  println!("updated articles: {updated_articles:#?}");

  updated_row_count = diesel::update(articles)
    .set(visit_count.eq(visit_count + 1))
    .execute(connection)
    .expect("failed to update articles");
  println!("updated_row_count: {updated_row_count:?}");

  updated_row_count = diesel::update(articles)
    .filter(id.le(5))
    .set((
      visit_count.eq(0),
      draft.eq(false)
    ))
    .execute(connection)
    .expect("failed to update articles");
  println!("updated_row_count: {updated_row_count:?}");
  
}

fn update_with_aschangeset(connection: &mut SqliteConnection) {

  #[derive(AsChangeset)]
  #[diesel(table_name = self::schema::articles)]
  struct ArticleChangeSet {
      title: String,
      body: String,
      draft: bool,
      publish_at: NaiveDateTime,
      visit_count: i32,
  }

  let current_time = Utc::now().naive_utc();

  let article = ArticleChangeSet {
    title: "".to_string(),
    body: "".to_string(),
    draft: false,
    publish_at: current_time,
    visit_count: 0,
  };

  let article_with_id = Article {
    id: 0,
    title: "".to_string(),
    body: "".to_string(),
    draft: false,
    publish_at: current_time,
    visit_count: 0,
  };

  let query1 = diesel::update(articles).set(article);
  println!("query1:\n{}", debug_query::<Sqlite, _>(&query1));

  // #[derive(AsChangeset)] automatically ignores the primary key if it is `id`; otherwise, use #[diesel(primary_key(...))].
  let query2 = diesel::update(articles).set(article_with_id);
  println!("query2:\n{}", debug_query::<Sqlite, _>(&query2));

  #[derive(AsChangeset)]
  #[diesel(table_name = self::schema::articles)]
  struct ArticleForm<'a> {
      title: Option<&'a str>,
      body: Option<&'a str>,
  }

  let query3 = diesel::update(articles)
    .set(&ArticleForm {
        title: None,
        body: Some("My new post"),
    });
  println!("query3:\n{}", debug_query::<Sqlite, _>(&query3));
}

fn execute_query(connection:&mut SqliteConnection) {

  #[derive(Queryable, Debug)]
  #[diesel(table_name = self::schema::articles)]
  struct ArticleVisit {
    id: i32,
    visit_count: i32,
  }

  let query1_result: ArticleVisit = diesel::update(articles)
    .filter(id.eq(2))
    .set(visit_count.eq(visit_count+1))
    .returning((id, visit_count))
    .get_result(connection)
    .expect("failed to update article");
  println!("query1_result:\n{query1_result:?}");

  let query2_result: Vec<ArticleVisit> = diesel::update(articles)
      .filter(id.le(3))
      .set(visit_count.eq(visit_count+1))
      .returning((id, visit_count))
      .get_results(connection)
      .expect("failed to update articles");
  println!("query2_result:\n{query2_result:?}");
  
  #[derive(AsChangeset, Debug)]
  #[diesel(table_name = self::schema::articles)]
  struct ArticleDraft {
    draft: bool,
  }

  #[derive(Queryable, Debug)]
  struct ArticleDraftResult {
      id: i32,
      draft: bool,
  }

  let query3_result: Vec<ArticleDraftResult> = diesel::update(articles)
      .filter(id.le(3))
      .set(&ArticleDraft {
        draft: false,
      })
      .returning((id, draft))
      .get_results(connection)
      .expect("failed to update articles");
  println!("query3_result:\n{query3_result:?}");
}

fn main() {
  let connection = &mut establish_connection();
  // update_basic(connection);
  // update_with_aschangeset(connection);
  execute_query(connection);
}