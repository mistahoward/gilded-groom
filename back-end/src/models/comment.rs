use diesel::prelude::*;
use crate::schema::comment;

#[derive(Queryable, Selectable)]
#[diesel(belongs_to(User))]
#[diesel(table_name = comment)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Comment {
	pub id: Option<i32>,
	pub user_id: i32,
	pub details: String,
	pub timestamp: chrono::NaiveDateTime
}
