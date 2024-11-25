use std::{collections::HashMap, str::FromStr};

use rocket::{get, serde::json::Json, State};
use serde::{Deserialize, Serialize};

use crate::error::{self, ParseError};

use super::Api;

#[derive(Serialize, Deserialize, Debug)]
pub struct Capabilities {
	pub capabilities: Vec<String>,
}

#[get("/capabilities")]
pub fn capabilities(capabilities: &State<Capabilities>) -> Json<&Capabilities> {
	Json(capabilities.inner())
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VersionRange {
	pub min_version: u32,
	pub max_version: u32,
}

impl VersionRange {
	pub fn new(min_version: u32, max_version: u32) -> Self {
		VersionRange { min_version, max_version }
	}
}

#[get("/capabilities/<api_name>")]
pub fn api_capability<'a>(
	api_name: &'a str,
	version_ranges: &'a State<HashMap<Api, VersionRange>>,
) -> error::JsonResult<&'a VersionRange> {
	if let Ok(ref api) = Api::from_str(api_name) {
		Ok(Json(&version_ranges[api]))
	} else {
		Err(ParseError::UnknownApi(api_name.to_string()).into())
	}
}
