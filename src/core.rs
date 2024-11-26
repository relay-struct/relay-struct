use serde::{Deserialize, Serialize};
use strum::EnumString;

pub mod v0;

#[derive(Serialize, Deserialize, EnumString, PartialEq, Eq, Hash, Debug)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum Api {
	Core,
	Post,
	Relay,
	Host,
	UserId,
}
