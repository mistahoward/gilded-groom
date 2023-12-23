use diesel::prelude::*;
use crate::schema::discovery_method;

#[derive(Queryable, Selectable)]
#[diesel(table_name = discovery_method)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct DiscoveryMethod {
	pub id: Option<i32>,
	pub name: String,
	pub description: Option<String>,
	pub active: bool
}
