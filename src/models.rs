use diesel::{Queryable, QueryableByName};

use super::schema::*;

#[derive(Clone, Debug, Identifiable, PartialEq, Queryable)]
pub struct User {
  pub id: String,
  pub username: String,
  pub displayname: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Queryable, QueryableByName)]
#[diesel(table_name = users)]
pub struct DisplayUser {
  pub id: String,
  pub displayname: Option<String>,
}

#[derive(Clone, Debug, Identifiable, PartialEq, Queryable)]
#[diesel(table_name = community)]
pub struct Community {
  pub id: i32,
  pub name: Option<String>,
}

#[derive(Queryable, Associations)]
#[diesel(belongs_to(Community, foreign_key = community_id))]
#[diesel(primary_key(recipe_id, ingredient_id))]
pub struct CommunityMember {
  pub community_id: i32,
  pub user_id: String,
}
