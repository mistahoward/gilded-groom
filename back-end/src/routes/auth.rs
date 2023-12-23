#[get("/token?<code>&<error>")]
pub fn token_test(code: Option<String>, error: Option<String>) -> &'static str {
	println!("{:?}", code);
	println!("{:?}", error);
	"Hello, world!"
}
