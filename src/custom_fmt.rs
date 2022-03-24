//! TODO: make this a separate crate, in a separate project

use std::default::default;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};

pub(crate) trait CustomFormat {
	type Options;

	fn custom_format(&self, options: Self::Options) -> CustomFormatWrapper<Self> {
		CustomFormatWrapper {
			value: self,
			options,
		}
	}

	fn default_format(&self) -> CustomFormatWrapper<Self> where Self::Options: Default {
		CustomFormatWrapper {
			value: self,
			options: default(),
		}
	}

	/// Implement this to provide custom formatting for this type.
	fn fmt(&self, f: &mut fmt::Formatter<'_>, options: &Self::Options) -> fmt::Result;
}

pub(crate) struct CustomFormatWrapper<'a, T: CustomFormat + ?Sized> {
	value: &'a T,
	options: T::Options,
}

impl<'a, T: CustomFormat> Debug for CustomFormatWrapper<'a, T> {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		self.value.fmt(f, &self.options)
	}
}

impl<'a, T: CustomFormat> Display for CustomFormatWrapper<'a, T> {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		self.value.fmt(f, &self.options)
	}
}