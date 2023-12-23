use crate::{schema::customer_history, structs::operation::NullableOperation};
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(belongs_to(Customer))]
#[diesel(table_name = customer_history)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct DiscoveryMethodHistory {
	pub id: Option<i32>,
	pub customer_id: i32,
	pub field_name: String,
	pub operation: NullableOperation,
	pub old_value: Option<String>,
	pub new_value: Option<String>,
	pub timestamp: chrono::NaiveDateTime
}
