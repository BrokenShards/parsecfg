// document.rs
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
use crate::{
	error::{box_error, make_error, CfgError, CfgResult},
	lexer::*,
	Section,
};
use std::{fmt::Display, fs, str::FromStr};

/// A cfg document containing a collection of [`Section`]s.
pub struct Document
{
	m_sections: Vec<Section>,
}
impl Default for Document
{
	fn default() -> Self
	{
		Self {
			m_sections: Default::default(),
		}
	}
}
impl FromLexer for Document
{
	fn from_lexer(lexer: &mut Lexer) -> CfgResult<Self>
	where
		Self: Sized,
	{
		if lexer.is_empty()
		{
			return Err(box_error(
				"Cannot parse Document from tokens: Index out of range.",
			));
		}

		let mut sects: Vec<Section> = Vec::new();

		while !lexer.is_empty()
		{
			let s = Section::from_lexer(lexer)?;

			if !s.is_valid()
			{
				return Err(box_error(&format!(
					"Cannot parse Document from tokens: The section {} is invalid.",
					s.name(),
				)));
			}

			let slo = s.name().to_lowercase();

			for sect in &sects
			{
				if sect.name().to_lowercase() == slo
				{
					return Err(box_error(&format!(
						"Cannot parse Document from tokens: A section with the name {} already \
						 exists.",
						sect.name(),
					)));
				}
			}

			sects.push(s);
		}

		Ok(Self::new(&sects))
	}
}
impl FromStr for Document
{
	type Err = CfgError;

	fn from_str(s: &str) -> Result<Self, Self::Err>
	{
		let mut lexer = Lexer::new();

		match lexer.parse_string(s)
		{
			Err(e) =>
			{
				return Err(make_error(&format!(
					"Cannot parse string into tokens to create a document: {e}"
				)))
			}
			_ =>
			{}
		};

		match Document::from_lexer(&mut lexer)
		{
			Ok(k) => Ok(k),
			Err(e) =>
			{
				return Err(make_error(&format!(
					"Cannot parse document from string: {e}"
				)))
			}
		}
	}
}
impl Display for Document
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
	{
		for section in &self.m_sections
		{
			let result = writeln!(f, "{section}\n");

			if result.is_err()
			{
				return result;
			}
		}

		Ok(())
	}
}
impl Document
{
	/// Creates and returns a new empty Document.
	pub fn new(sections: &[Section]) -> Self
	{
		Self {
			m_sections: sections.to_vec(),
		}
	}
	/// Creates and returns a new Document loaded from a file.
	pub fn from_file(path: &str) -> CfgResult<Self>
	{
		let filedata = match fs::read_to_string(path)
		{
			Ok(fd) => fd,
			Err(e) => return Err(box_error(&format!("Cannot read document from file: {e}"))),
		};
		match Self::from_str(&filedata)
		{
			Ok(s) => Ok(s),
			Err(e) => return Err(box_error(&format!("Cannot read document from file: {e}"))),
		}
	}

	/// Returns an iterator over the contained sections.
	pub fn iter(&self) -> std::slice::Iter<'_, Section> { self.m_sections.iter() }
	/// Returns a mutable iterator over the contained [`Section`]s.
	pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, Section> { self.m_sections.iter_mut() }

	/// If the document is empty, containing no sections.
	pub fn is_empty(&self) -> bool { self.m_sections.is_empty() }
	/// The amount of sections the document contains.
	pub fn len(&self) -> usize { self.m_sections.len() }

	/// Returns [`Some`] containing the index of the section with the given name if it exists in the
	/// document, otherwise [`None`].
	pub fn index_of(&self, section: &str) -> Option<usize>
	{
		let mut i = 0usize;
		let key = section.to_lowercase();

		while i < self.m_sections.len()
		{
			if self.m_sections[i].name().to_lowercase() == key
			{
				return Some(i);
			}

			i += 1;
		}

		None
	}
	/// Returns true if the document contains a section with the given name, otherwise false.
	pub fn contains(&self, section: &str) -> bool { self.index_of(section).is_some() }
	/// Returns [`Some`] containing a reference to the section with the given name if it exists in
	/// the document, otherwise [`None`].
	pub fn get(&self, section: &str) -> Option<&Section>
	{
		match self.index_of(section)
		{
			Some(i) => Some(&self.m_sections[i]),
			_ => None,
		}
	}
	/// Returns [`Some`] containing a mutable reference to the section with the given name if it
	/// exists in the document, otherwise [`None`].
	pub fn get_mut(&mut self, section: &str) -> Option<&mut Section>
	{
		match self.index_of(section)
		{
			Some(i) => Some(&mut self.m_sections[i]),
			_ => None,
		}
	}
	/// Returns [`Some`] containing a reference to the section at the given index, or [`None`] if
	/// the index is out of range.
	pub fn get_at(&self, index: usize) -> Option<&Section>
	{
		if index >= self.m_sections.len()
		{
			None
		}
		else
		{
			Some(&self.m_sections[index])
		}
	}
	/// Returns [`Some`] containing a mutable reference to the section at the given index, or
	/// [`None`] if the index is out of range.
	pub fn get_at_mut(&mut self, index: usize) -> Option<&mut Section>
	{
		if index >= self.m_sections.len()
		{
			None
		}
		else
		{
			Some(&mut self.m_sections[index])
		}
	}

	/// Adds a new section to the end of the document. Returns true on success or false if the
	/// section is not valid or the document already contains a section with the same name.
	pub fn push(&mut self, section: Section) -> bool
	{
		if !section.is_valid() || self.contains(&section.name())
		{
			return false;
		}

		self.m_sections.push(section);
		true
	}
	/// Inserts a new section at the given index. Returns true on success or false if the section is
	/// not valid or the document already contains a section with the same name.
	pub fn insert(&mut self, index: usize, section: Section) -> bool
	{
		if index > self.m_sections.len() || !section.is_valid() || self.contains(&section.name())
		{
			return false;
		}
		if index == self.m_sections.len()
		{
			return self.push(section);
		}

		self.m_sections.insert(index, section);
		true
	}
	/// Removes the section with the given name if it exists in the document and returns true;
	/// returns false if a section with the given name does not exist within the document.
	pub fn remove(&mut self, section: &str) -> bool
	{
		if let Some(index) = self.index_of(section)
		{
			self.remove_at(index);
			return true;
		}

		false
	}
	/// Removes the section at the given index from the document.
	pub fn remove_at(&mut self, index: usize)
	{
		if index >= self.m_sections.len()
		{
			return;
		}

		self.m_sections.remove(index);
	}
	/// Clears the document, removing all sections.
	pub fn clear(&mut self) { self.m_sections.clear(); }
}
