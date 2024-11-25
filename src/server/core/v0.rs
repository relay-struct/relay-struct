use rocket::{get, serde::json::Json, State};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Capabilities {
	pub capabilities: Vec<String>,
}

#[get("/capabilities")]
pub fn capabilities(capabilities: &State<Capabilities>) -> Json<&Capabilities> {
	Json(capabilities.inner())
}
