// error.rs
//
// ParseCfg - A simple cfg file parser.
// Copyright(C) 2024 Michael Furlong.
//
// This program is free software: you can redistribute it and/or modify it under the terms of
// the GNU General Public License as published by the Free Software Foundation, either version 3
// of the License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
// without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See
// the GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along with this program.
// If not, see <https://www.gnu.org/licenses/>.
//
use std::{error::Error, fmt};

/// Error type used by parsecfg.
#[derive(Debug)]
pub struct CfgError
{
	message: String,
}
impl CfgError
{
	/// Creates a new error with the given message.
	pub fn new(msg: &str) -> Self
	{
		Self {
			message: String::from(msg),
		}
	}
}
impl fmt::Display for CfgError
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", &self.message) }
}
impl Error for CfgError {}

/// Creates a new error with the given message.
pub fn make_error(msg: &str) -> CfgError { CfgError::new(msg) }
/// Creates a new boxed error with the given message.
pub fn box_error(msg: &str) -> Box<CfgError> { Box::new(make_error(msg)) }

/// Result type used by parsecfg. `T` is type contained in [`Ok`] variant.
pub type CfgResult<T> = Result<T, Box<dyn Error>>;
