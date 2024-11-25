mod core;

use core::v0::Capabilities;

use rocket::{launch, routes};

#[launch]
fn hello_fediverse() -> _ {
	rocket::build()
		.manage(Capabilities {
			capabilities: vec![
				"core".to_string(),
			]
		})
		.mount(
			"/_rs/core/v0",
			routes![
				core::v0::capabilities
			]
		)
}
