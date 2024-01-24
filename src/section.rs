// section.rs
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
	FromTokens, Key, Token,
};

/// A named section containing a collection of keys.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Section
{
	m_name: String,
	m_keys: Vec<Key>,
}
impl Default for Section
{
	fn default() -> Self
	{
		Self {
			m_name: as_valid_name(Default::default(), '_'),
			m_keys: Default::default(),
		}
	}
}
impl FromTokens for Section
{
	fn from_tokens(tokens: &Vec<Token>, index: &mut usize) -> CfgResult<Self>
	where
		Self: Sized,
	{
		let len = tokens.len();

		if !is_section_tokens(tokens, index)
		{
			return Err(box_error(
				"Cannot parse Section from tokens: Expected Section header.",
			));
		}

		*index += 1;

		let id = if let Token::Identifier(i) = &tokens[*index]
		{
			i
		}
		else
		{
			return Err(box_error(
				"Cannot parse Section from tokens: Expected Identifier.",
			));
		};
		*index += 2;

		let mut keys: Vec<Key> = Vec::new();

		while *index < len
		{
			if is_section_tokens(tokens, index)
			{
				break;
			}

			let k = match Key::from_tokens(&tokens, index)
			{
				Ok(k) => k,
				Err(e) =>
				{
					return Err(box_error(&format!(
						"Cannot parse Section from tokens: Failed loading key: {e}."
					)))
				}
			};
			if !k.is_valid()
			{
				return Err(box_error(
					"Cannot parse Section from tokens: Parsed key is invalid.",
				));
			}

			let klo = k.name().to_lowercase();

			for ky in &keys
			{
				if ky.name().to_lowercase() == klo
				{
					return Err(box_error(&format!(
						"Cannot parse Section from tokens: A key with the name {} already exists.",
						ky.name()
					)));
				}
			}

			keys.push(k);
		}

		Ok(Self::new(&id, &keys))
	}
}
impl Display for Section
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
	{
		let mut result = write!(f, "[{}]", &self.m_name);

		if result.is_err()
		{
			return result;
		}

		for key in &self.m_keys
		{
			result = write!(f, "\n{key}");

			if result.is_err()
			{
				return result;
			}
		}

		result
	}
}
impl Section
{
	/// Returns a new Section with the given name and keys.
	pub fn new(name: &str, keys: &[Key]) -> Self
	{
		Self {
			m_name: as_valid_name(name, '_'),
			m_keys: keys.to_vec(),
		}
	}

	/// Returns a reference to the sections' name.
	pub fn name(&self) -> &String { &self.m_name }
	/// Renames the section. The name may be modified, see [`as_valid_name`] for more details.
	pub fn rename(&mut self, name: &str) { self.m_name = as_valid_name(name, '_'); }

	/// Returns an iterator over the contained keys.
	pub fn iter(&self) -> std::slice::Iter<'_, Key> { self.m_keys.iter() }
	/// Returns a mutable iterator over the contained keys.
	pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, Key> { self.m_keys.iter_mut() }

	/// If the section is empty, containing no keys.
	pub fn is_empty(&self) -> bool { self.m_keys.is_empty() }
	/// The amount of keys the section contains.
	pub fn len(&self) -> usize { self.m_keys.len() }

	/// If the section is valid.
	pub fn is_valid(&self) -> bool { is_valid_name(&self.m_name) }

	/// Returns [`Some`] containing the index of the key with the given name if it exists in the
	/// section, otherwise [`None`].
	pub fn index_of(&self, key: &str) -> Option<usize>
	{
		let mut i = 0usize;
		let key = key.to_lowercase();

		while i < self.m_keys.len()
		{
			if self.m_keys[i].name().to_lowercase() == key
			{
				return Some(i);
			}

			i += 1;
		}

		None
	}
	/// Returns true if the section contains a key with the given name, otherwise false.
	pub fn contains(&self, key: &str) -> bool { self.index_of(key).is_some() }
	/// Returns [`Some`] containing a reference to the key with the given name if it exists in the
	/// section, otherwise [`None`].
	pub fn get(&self, key: &str) -> Option<&Key>
	{
		match self.index_of(key)
		{
			Some(i) => Some(&self.m_keys[i]),
			_ => None,
		}
	}
	/// Returns [`Some`] containing a mutable reference to the key with the given name if it exists
	/// in the section, otherwise [`None`].
	pub fn get_mut(&mut self, key: &str) -> Option<&mut Key>
	{
		match self.index_of(key)
		{
			Some(i) => Some(&mut self.m_keys[i]),
			_ => None,
		}
	}
	/// Returns [`Some`] containing a reference to the key at the given index, or [`None`] if the
	/// index is out of range.
	pub fn get_at(&self, index: usize) -> Option<&Key>
	{
		if index >= self.m_keys.len()
		{
			None
		}
		else
		{
			Some(&self.m_keys[index])
		}
	}
	/// Returns [`Some`] containing a mutable reference to the key at the given index, or [`None`]
	/// if the index is out of range.
	pub fn get_at_mut(&mut self, index: usize) -> Option<&mut Key>
	{
		if index >= self.m_keys.len()
		{
			None
		}
		else
		{
			Some(&mut self.m_keys[index])
		}
	}
	/// Adds a new key to the end of the section. Returns true on success or false if the key is not
	/// valid or the section already contains a key with the same name.
	pub fn push(&mut self, key: Key) -> bool
	{
		if !key.is_valid() || self.contains(&key.name())
		{
			return false;
		}

		self.m_keys.push(key);
		true
	}
	/// Inserts a new key at the given index. Returns true on success or false if the key is not
	/// valid or the section already contains a key with the same name.
	pub fn insert(&mut self, index: usize, key: Key) -> bool
	{
		if index >= self.m_keys.len() || !key.is_valid() || self.contains(&key.name())
		{
			return false;
		}

		self.m_keys.insert(index, key);
		true
	}
	/// Removes the key with the given name if it exists in the section and returns true; returns
	/// false if a key with the given name does not exist within the section.
	pub fn remove(&mut self, key: &str) -> bool
	{
		if let Some(index) = self.index_of(key)
		{
			self.remove_at(index);
			return true;
		}

		false
	}
	/// Removes the key at the given index from the section.
	pub fn remove_at(&mut self, index: usize)
	{
		if index >= self.m_keys.len()
		{
			return;
		}

		self.m_keys.remove(index);
	}
	/// Clears the section, removing all keys.
	pub fn clear(&mut self) { self.m_keys.clear(); }
}

/// Returns true if the next tokens, starting from `index`, within `tokens`, describe a section
/// header, otherwise false.
pub fn is_section_tokens(tokens: &Vec<Token>, index: &usize) -> bool
{
	if index + 2 >= tokens.len()
		|| tokens[*index] != Token::OpenBracket
		|| tokens[*index + 2] != Token::CloseBracket
	{
		return false;
	}

	if let Token::Identifier(_) = tokens[*index + 1]
	{
		true
	}
	else
	{
		false
	}
}
