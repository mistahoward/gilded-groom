pub mod schema;
pub mod models;
pub mod structs;
pub mod routes;

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
	"Hello, world!"
}

#[launch]
fn rocket() -> _ {
	rocket::build().mount("/", routes![index])
	.mount("/auth", routes![routes::auth::token_test])
}