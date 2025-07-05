use core::fmt;
use core::num::{ParseFloatError, ParseIntError};
use core::str::Utf8Error;
use std::io;
use std::string::FromUtf8Error;

#[derive(Debug)]
pub enum Error {
	Io(io::Error),
	NonUtf8Parse(Utf8Error),
	NonUtf8Val(FromUtf8Error),
	InvalidInt(ParseIntError),
	InvalidFloat(ParseFloatError),
	CrcMismatch(u16, u16),
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Error::Io(e) => write!(f, "IO error: {e}, details: {e:?}"),
			Error::NonUtf8Parse(e) => write!(f, "Non UTF-8 string: {e}, details: {e:?}"),
			Error::NonUtf8Val(e) => write!(f, "Non UTF-8 string: {e}, details: {e:?}"),
			Error::InvalidInt(e) => write!(f, "Invalid integer: {e}, details: {e:?}"),
			Error::InvalidFloat(e) => write!(f, "Invalid float: {e}, details: {e:?}"),
			Error::CrcMismatch(actual, expected) => write!(
				f,
				"Telegram CRC mismatch, computed: {actual:X}, doesn't match expected: {expected:X}"
			),
		}
	}
}

impl From<io::Error> for Error {
	fn from(err: io::Error) -> Self {
		Error::Io(err)
	}
}

impl From<Utf8Error> for Error {
	fn from(err: Utf8Error) -> Self {
		Error::NonUtf8Parse(err)
	}
}

impl From<FromUtf8Error> for Error {
	fn from(err: FromUtf8Error) -> Self {
		Error::NonUtf8Val(err)
	}
}

impl From<ParseIntError> for Error {
	fn from(err: ParseIntError) -> Self {
		Error::InvalidInt(err)
	}
}

impl From<ParseFloatError> for Error {
	fn from(err: ParseFloatError) -> Self {
		Error::InvalidFloat(err)
	}
}

impl std::error::Error for Error {}
