use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
	#[error("unknown Handle format: {0}")]
	UnknownFormat(String),
	#[error("invalid Handle format: {0}")]
	InvalidFormat(String),
}

pub type ParseResult<T> = Result<T, ParseError>;
