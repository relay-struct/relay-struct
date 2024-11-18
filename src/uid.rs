pub mod error;

use std::str::FromStr;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Handle {
	pub user: String,
	pub domain: String,
}

impl Handle {
	/// Parse a Canonical Handle
	///
	/// # Safety
	/// This operation is unchecked and uses [Option::unwrap_unchecked].
	/// It is up to the caller to guarantee that the handle is indeed of Canonical format.
	pub unsafe fn parse_canonical_unchecked(handle: &str) -> Self {
		let (domain, user) = unsafe { handle.split_once(':').unwrap_unchecked() };
		Self {
			user: user.to_string(),
			domain: domain.to_string(),
		}
	}

	/// Parse a Common Handle
	///
	/// # Safety
	/// This operation is unchecked and uses [Option::unwrap_unchecked].
	/// It is up to the caller to guarantee that the handle is indeed of Common format.
	pub unsafe fn parse_common_unchecked(handle: &str) -> Self {
		let handle = &handle[1..]; // Spooky!
		let (user, domain) = unsafe { handle.split_once('@').unwrap_unchecked() };
		Self {
			user: user.to_string(),
			domain: domain.to_string(),
		}
	}

	/// Creates a Canonical Handle.
	pub fn to_canonical(&self) -> String {
		format!("{}:{}", self.domain, self.user)
	}
}

impl FromStr for Handle {
	type Err = error::ParseError;

	fn from_str(handle: &str) -> Result<Self, Self::Err> {
		// SAFETY:
		// The operations are checked, fulfilling the safety qualifications.
		let format = HandleFormat::from(handle);
		match format {
			HandleFormat::Canonical => Ok(unsafe { Self::parse_canonical_unchecked(handle) }),
			HandleFormat::Common => Ok(unsafe { Self::parse_common_unchecked(handle) }),
			HandleFormat::Unknown => Err(Self::Err::UnknownFormat(handle.to_string())),
		}
	}
}

pub enum HandleFormat {
	Canonical,
	Common,
	Unknown,
}

impl From<&str> for HandleFormat {
	fn from(handle: &str) -> Self {
		let at_handle = handle.chars().nth(0) == Some('@');
		let at_count = handle.matches('@').count();
		let colon_count = handle.matches(':').count();

		if at_handle && at_count == 2 {
			return Self::Common
		} else if !at_handle && colon_count == 1 {
			return Self::Canonical
		} else {
			return Self::Unknown
		}
	}
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserID {
	pub handle: String,
	pub pubkey: String,
}
