pub mod error;

use std::str::FromStr;

use error::{ParseError, ParseResult};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Handle {
	pub user: String,
	pub domain: String,
}

impl Handle {
	/// Parse a Canonical Handle
	pub fn parse_canonical(handle: &str) -> Option<Self> {
		let (domain, user) = handle.split_once(':')?;
		Some(
			Self {
				user: user.to_string(),
				domain: domain.to_string(),
			}
		)
	}

	/// Parse a Common Handle
	pub fn parse_common(handle: &str) -> Option<Self> {
		let handle = &handle[1..]; // Spooky!
		let (user, domain) = handle.split_once('@')?;
		Some(
			Self {
				user: user.to_string(),
				domain: domain.to_string(),
			}
		)
	}

	pub fn check_parsed(parsed: Option<Self>, handle: &str) -> ParseResult<Self> {
		if parsed.is_none() {
			return Err(ParseError::InvalidFormat(handle.to_string()))
		} else {
			return Ok(parsed.unwrap())
		}
	}

	/// Creates a Canonical Handle.
	pub fn to_canonical(&self) -> String {
		format!("{}:{}", self.domain, self.user)
	}

	/// Creates a Common Handle.
	pub fn to_common(&self) -> String {
		format!("@{}@{}", self.user, self.domain)
	}
}

impl FromStr for Handle {
	type Err = error::ParseError;

	fn from_str(handle: &str) -> Result<Self, Self::Err> {
		let format = HandleFormat::from(handle);
		match format {
			HandleFormat::Canonical => return Handle::check_parsed(
				Self::parse_canonical(handle),
				handle,
			),
			HandleFormat::Common => Handle::check_parsed(
				Self::parse_common(handle),
				handle,
			),
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
