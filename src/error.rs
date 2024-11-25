use rocket::Responder;
use serde::{ser::SerializeStruct, Serialize};
use strum::IntoStaticStr;
use thiserror::Error;

#[derive(Error, IntoStaticStr, Responder, Debug)]
pub enum ParseError {
	#[response(status = 400, content_type = "json")]
	#[error("unknown Handle format: {0}")]
	UnknownHandleFormat(String),
	#[response(status = 400, content_type = "json")]
	#[error("invalid Handle format: {0}")]
	InvalidHandle(String),
	#[response(status = 404, content_type = "json")]
	#[error("unknown API: {0}")]
	UnknownApi(String),
}

impl Into<Error> for ParseError {
	fn into(self) -> Error {
		let error = <&'static str>::from(&self);
		Error {
			error: error.to_string(),
			source: self.into()
		}
	}
}

pub type ParseResult<T> = core::result::Result<T, ParseError>;

#[derive(Error, Debug)]
#[error("{error}: {source}")]
pub struct Error {
	pub error: String,
	#[source]
	pub source: anyhow::Error,
}

pub type Result<T> = core::result::Result<T, Error>;

impl Serialize for Error {
	fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
	where
		S: serde::Serializer {
		let mut state = serializer.serialize_struct("Error", 2)?;
		state.serialize_field("error", &self.error)?;
		state.serialize_field("description", &format!("{}", self.source))?;
		state.end()
	}
}
