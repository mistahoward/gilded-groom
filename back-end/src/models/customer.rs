use diesel::prelude::*;
use crate::schema::customer;

#[derive(Queryable, Selectable)]
#[diesel(table_name = customer)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Customer {
	pub id: Option<i32>,
	pub name: String,
	pub first_name: String,
	pub last_name: String,
	pub email: String,
	pub phone_number: String,
	pub discovery_method_id: i32,
	pub created_at: chrono::NaiveDateTime,
	pub active: bool
}
