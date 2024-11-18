use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
	#[error("unknown Handle format: {0}")]
	UnknownFormat(String),
}
