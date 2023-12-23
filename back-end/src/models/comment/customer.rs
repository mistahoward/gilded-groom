use crate::schema::customer_comment;
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(belongs_to(Customer))]
#[diesel(table_name = customer_comment)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct CustomerComment {
	pub id: Option<i32>,
	pub comment_id: i32,
	pub customer_id: i32
}
