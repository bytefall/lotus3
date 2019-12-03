use failchain::{BoxedError, ChainErrorKind};
use failure::Fail;

pub type Error = BoxedError<ErrorKind>;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Eq, PartialEq, Debug, Fail)]
pub enum ErrorKind {
	#[fail(display = "{}", 0)]
	BuildWindow(String),

	#[fail(display = "Context {} error", 0)]
	Context(&'static str),

	#[fail(display = "System {} failed for `{}`.", 0, 1)]
	System(&'static str, &'static str),
}

impl ChainErrorKind for ErrorKind {
	type Error = Error;
}
