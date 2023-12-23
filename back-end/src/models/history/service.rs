use crate::{schema::service_history, structs::operation::NullableOperation};
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(belongs_to(Service))]
#[diesel(table_name = service_history)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct ServiceHistory {
	pub id: Option<i32>,
	pub service_id: i32,
	pub field_name: String,
	pub operation: NullableOperation,
	pub old_value: Option<String>,
	pub new_value: Option<String>,
	pub timestamp: chrono::NaiveDateTime
}
