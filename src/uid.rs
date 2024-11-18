use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Handle {
	pub username: String,
	pub domain: String,
}

impl Handle {
	pub fn parse_canonical(handle: String) -> Option<Self> {
		let (username, domain) = handle.split_once(':')?;
		Some(
			Self {
				username: username.to_string(),
				domain: domain.to_string(),
			}
		)
	}
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserID {
	pub handle: String,
	pub pubkey: String,
}
