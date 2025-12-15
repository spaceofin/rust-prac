use diesel::prelude::*;
use diesel_sqlite::*;
use self::schema::articles::dsl::*;
use diesel::{debug_query, sqlite::Sqlite, dsl::now};
use chrono::NaiveDateTime;

fn update_basic(connection: &mut SqliteConnection) {
  let query1 = diesel::update(articles).set(draft.eq(false));
  // UPDATE `articles` SET `draft` = ? -- binds: [false]
  println!("{}", debug_query::<Sqlite, _>(&query1));

  let query2 = diesel::update(articles)
    .filter(publish_at.lt(now))
    .set(draft.eq(false));
  // UPDATE `articles` SET `draft` = ? WHERE (`articles`.`publish_at` < CURRENT_TIMESTAMP) -- binds: [false]
  println!("{}", debug_query::<Sqlite, _>(&query2));

  #[derive(Queryable, Identifiable, AsChangeset, Debug)]
  #[diesel(table_name = self::schema::articles)]
  pub struct Article {
      pub id: i32,
      pub title: String,
      pub body: String,
      pub draft: bool,
      pub publish_at: NaiveDateTime,
      pub visit_count: i32,
  }

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

fn main() {
  let connection = &mut establish_connection();
  update_basic(connection);
}