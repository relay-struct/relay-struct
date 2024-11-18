use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Handle {
	pub domain: String,
	pub user: String,
}

impl Handle {
	pub fn parse_canonical(handle: String) -> Option<Self> {
		let (domain, user) = handle.split_once(':')?;
		Some(
			Self {
				domain: domain.to_string(),
				user: user.to_string(),
			}
		)
	}
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserID {
	pub handle: String,
	pub pubkey: String,
}
