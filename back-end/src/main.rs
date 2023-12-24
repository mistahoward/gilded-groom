use dotenvy::dotenv;

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
	dotenv().expect(".env file not found");

	rocket::build().mount("/", routes![index])
		.mount("/auth", routes::auth::get_routes())
}