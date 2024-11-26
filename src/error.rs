use std::fmt::Debug;

use rocket::{http::Status, response::Responder, serde::json::Json};
use serde::{ser::SerializeStruct, Serialize};
use strum::IntoStaticStr;
use thiserror::Error;

#[derive(Error, Serialize, IntoStaticStr, Responder, Debug)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum ParseError {
	#[response(status = 400)]
	#[error("unknown Handle format: {0}")]
	UnknownHandleFormat(String),
	#[response(status = 400)]
	#[error("invalid Handle format: {0}")]
	InvalidHandle(String),
	#[response(status = 404)]
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

#[derive(Error, IntoStaticStr, Serialize, Debug)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum ErrorType {
	#[error("parse error: {0}")]
	ParseError(#[from] ParseError)
}

#[derive(Error, Debug)]
#[error("{error}: {source}")]
pub struct Error {
	pub error: String,
	#[source]
	pub source: ErrorType,
}

pub type Result<T> = core::result::Result<T, Error>;
pub type JsonResult<T> = core::result::Result<Json<T>, Error>;

impl Serialize for Error {
	fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
	where
		S: serde::Serializer {
		let mut state = serializer.serialize_struct("Error", 2)?;
		let error_type = <&'static str>::from(&self.source);
		state.serialize_field("error", &self.error)?;
		state.serialize_field("error_type", error_type)?;
		state.serialize_field("description", &format!("{}", self.source))?;
		state.end()
	}
}

impl<'r, 'o: 'r> Responder<'r, 'o> for Error {
	fn respond_to(self, request: &'r rocket::Request<'_>) -> rocket::response::Result<'o> {
		let json = serde_json::to_string(&self)
				.map_err(|e| {
					log::error!("JSON failed to serialize: {:?}", e);
					Status::InternalServerError
				})?;
		let response = rocket::response::content::RawJson(json).respond_to(&request);
		// todo: make this a macro for every kind of supported error
		let source = self.source;
		if let Ok(mut response) = response {
			match source {
				ErrorType::ParseError(parse_error) => {
					let err_response = parse_error.respond_to(&request);
					if let Ok(err_response) = err_response {
						response.set_status(err_response.status());
					}
					return Ok(response)
				},
			}
		} else {
			return response
		}
	}
}
