use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::user)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
	pub id: Option<i32>,
	pub name: String,
	pub first_name: String,
	pub last_name: String,
	pub email: String,
	pub password: String,
	pub salt: String,
	pub created_at: i32,
	pub last_login: i32,
	pub active: bool,
}
