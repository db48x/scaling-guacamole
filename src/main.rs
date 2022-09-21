use anyhow::{anyhow, Context};
#[macro_use] extern crate diesel;
use diesel::prelude::*;

mod schema;
mod models;

fn main() -> Result<(), anyhow::Error> {
  let database_url = "file:test.db?mode=rwc";
  let manager = diesel::r2d2::ConnectionManager::new(&*database_url);
  let pool: diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::SqliteConnection>> = diesel::r2d2::Builder::new().build(manager)
    .map_err(|e| anyhow!(e))
    .with_context(|| format!("could not connect to “{database_url}”"))?;

  let community_id: u32 = 42;
  pool.get()?.transaction(|connection| {
    use schema::*;
    diesel::sql_function!(fn ifnull<T: diesel::sql_types::SingleValue>(a: T, b: T) -> T);
    let users: Vec<(String, Option<String>)> = community_members::table
      .left_join(users::table)
      .select((community_members::columns::user_id, ifnull(users::columns::displayname, users::columns::username.nullable())))
      .filter(community_members::community_id.eq(community_id as i32))
      .load(connection)
      .map_err(|e| anyhow!(e))
      .with_context(|| format!("looking up community members {community_id}"))?;
    Ok(dbg!(users))
  })?;
  Ok(())
}
