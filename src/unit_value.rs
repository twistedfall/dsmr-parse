use core::str;
use std::ops::Deref;
use std::str::FromStr;

/// A single value with an optional unit attached to it
#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
pub struct UnitValue<T> {
	pub value: T,
	pub unit: Option<String>,
}

impl<T> UnitValue<T> {
	pub fn without_unit(value: T) -> Self {
		Self { value, unit: None }
	}

	pub fn with_unit(value: T, unit: impl Into<String>) -> Self {
		Self {
			value,
			unit: Some(unit.into()),
		}
	}
}

impl<T> Deref for UnitValue<T> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		&self.value
	}
}

impl<T: FromStr> FromStr for UnitValue<T> {
	type Err = T::Err;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let (value, unit) = s.split_once('*').map_or((s, None), |(val, unit)| (val, Some(unit)));
		Ok(Self {
			value: T::from_str(value)?,
			unit: unit.map(str::to_string),
		})
	}
}
