use relay_struct::core::{v0::{Capabilities, VersionRange}, Api};
use std::collections::HashMap;

use rocket::{launch, routes};

#[launch]
fn hello_fediverse() -> _ {
	let version_ranges = HashMap::<Api, VersionRange>::from([
		(Api::Core, VersionRange::new(0, 0))
	]);

	rocket::build()
		.manage(Capabilities {
			capabilities: vec![
				"core",
			]
				.into_iter()
				.map(str::to_string)
				.collect()
		})
		.manage(version_ranges)
		.mount(
			"/_rs/core/v0",
			routes![
				relay_struct::core::v0::capabilities,
				relay_struct::core::v0::api_capability,
			]
		)
}
