use diesel::prelude::*;
use crate::schema::service;

#[derive(Queryable, Selectable)]
#[diesel(table_name = service)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Service {
	pub id: Option<i32>,
	pub name: String,
	pub description: String,
	pub price: f32
}
