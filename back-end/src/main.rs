pub mod schema;
pub mod models;
pub mod enums;

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
	"Hello, world!"
}

#[launch]
fn rocket() -> _ {
	rocket::build().mount("/", routes![index])
}