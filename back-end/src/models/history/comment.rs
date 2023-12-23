use crate::{schema::comment_history, enums::operation::NullableOperation};
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(belongs_to(Comment))]
#[diesel(table_name = comment_history)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct CommentHistory {
	pub id: Option<i32>,
	pub comment_id: i32,
	pub field_name: String,
	pub operation: NullableOperation,
	pub old_value: Option<String>,
	pub new_value: Option<String>,
	pub timestamp: chrono::NaiveDateTime
}
