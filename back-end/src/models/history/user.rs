use crate::{schema::user_history, structs::operation::NullableOperation};
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(belongs_to(User))]
#[diesel(table_name = user_history)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct UserHistory {
	pub id: Option<i32>,
	pub user_id: i32,
	pub operation: NullableOperation,
	pub field_name: String,
	pub old_value: Option<String>,
	pub new_value: Option<String>,
	pub timestamp: chrono::NaiveDateTime
}
