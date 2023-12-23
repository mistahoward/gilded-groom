use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::user)]
pub struct User {
	pub id: u32,
	pub name: String,
	pub first_name: String,
	pub last_name: String,
	pub email: String,
	pub password: String,
	pub salt: String,
	pub created_at: u32,
	pub last_login: u32,
}
