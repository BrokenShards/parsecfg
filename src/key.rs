// key.rs
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
use std::fmt::Display;

use crate::{
	error::{box_error, CfgResult},
	name::{as_valid_name, is_valid_name},
	FromTokens, KeyValue, Token,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Key
{
	m_name: String,

	/// The value of the key.
	pub value: KeyValue,
}
impl Default for Key
{
	fn default() -> Self
	{
		Self {
			m_name: as_valid_name(Default::default(), '_'),
			value: Default::default(),
		}
	}
}
impl FromTokens for Key
{
	fn from_tokens(tokens: &Vec<Token>, index: &mut usize) -> CfgResult<Self>
	where
		Self: Sized,
	{
		let len = tokens.len();

		if *index + 2 >= len
		{
			return Err(box_error("Not enough tokens left to load Key."));
		}

		let id = if let Token::Identifier(i) = &tokens[*index]
		{
			i
		}
		else
		{
			return Err(box_error("Unexpected token. Expected Identifier."));
		};

		*index += 1;

		if tokens[*index] != Token::Equals
		{
			return Err(box_error("Unexpected token. Expected equals sign."));
		}

		*index += 1;

		let val = KeyValue::from_tokens(&tokens, index)?;
		Ok(Self::new(&id, val))
	}
}
impl Display for Key
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
	{
		write!(f, "{} = {}", &self.m_name, self.value)
	}
}
impl Key
{
	/// Creates a new key with the given name and value.
	pub fn new(name: &str, value: KeyValue) -> Self
	{
		Self {
			m_name: as_valid_name(name, '_'),
			value,
		}
	}

	/// Returns the name of the key.
	pub fn name(&self) -> &String { &self.m_name }
	/// Renames the key. The given name may be modified to be valid.
	pub fn rename(&mut self, name: &str) { self.m_name = as_valid_name(name, '_'); }

	/// If the key is valid.
	pub fn is_valid(&self) -> bool { is_valid_name(&self.m_name) }
}
