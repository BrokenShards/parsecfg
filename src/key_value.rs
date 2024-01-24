// key_value.rs
//
// ParseCfg - A simple cfg file parser.
// Copyright(C) 2024 Michael Furlong.
//
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
	error::{box_error, CfgResult},
	indent, FromTokens, Key, Token,
};
use std::fmt::Display;

/// Possible values a [`Key`] can contain.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum KeyValue
{
	String(String),
	Array(Vec<String>),
	Table(Vec<Key>),
}
impl Default for KeyValue
{
	fn default() -> Self { Self::String(String::default()) }
}
impl FromTokens for KeyValue
{
	fn from_tokens(tokens: &Vec<Token>, index: &mut usize) -> CfgResult<Self>
	where
		Self: Sized,
	{
		let len = tokens.len();

		if *index >= len
		{
			return Err(box_error(
				"Trying to load KeyValue from tokens when the index is out of range.",
			));
		}

		match &tokens[*index]
		{
			Token::String(s) =>
			{
				*index += 1;
				return Ok(Self::String(s.clone()));
			}
			Token::OpenBracket =>
			{
				let mut result: Vec<String> = Vec::new();

				*index += 1;

				let mut ready = true;
				let mut closed = false;

				while *index < len
				{
					match &tokens[*index]
					{
						Token::String(s) =>
						{
							if !ready
							{
								return Err(box_error(
									"Unexpected token when loading Array; expected separator or \
									 close bracket.",
								));
							}
							result.push(s.clone());
							ready = false;
						}
						Token::Separator =>
						{
							if ready
							{
								return Err(box_error(
									"Unexpected token when loading Array; expected string or \
									 close bracket.",
								));
							}

							ready = true;
						}
						Token::CloseBracket =>
						{
							closed = true;
							*index += 1;
							break;
						}
						_ =>
						{
							return Err(box_error(&format!(
								"Unexpected token when loading Array: {}.",
								&tokens[*index]
							)))
						}
					}

					*index += 1;
				}

				if !closed
				{
					Err(box_error("Array missing closing square bracket."))
				}
				else
				{
					Ok(Self::Array(result))
				}
			}
			Token::OpenBrace =>
			{
				let mut result: Vec<Key> = Vec::new();
				let mut ready = true;
				let mut closed = false;

				*index += 1;

				while *index < len
				{
					if tokens[*index] == Token::CloseBrace
					{
						closed = true;
						*index += 1;
						break;
					}

					if !ready
					{
						if tokens[*index] == Token::Separator
						{
							ready = true;
							*index += 1;
							continue;
						}

						return Err(box_error(&format!(
							"Unexpected token when loading Table: {}. Expected comma.",
							&tokens[*index]
						)));
					}

					let key = Key::from_tokens(&tokens, index)?;

					if !key.is_valid()
					{
						return Err(box_error(&format!(
							"Error when loading Table: The key {} is invalid.",
							key.name()
						)));
					}

					result.push(key);
					ready = false;
				}

				if !closed
				{
					Err(box_error("Table missing closing square bracket."))
				}
				else
				{
					Ok(Self::Table(result))
				}
			}
			_ => Err(box_error(
				"Unable to load KeyValue from tokens, unexpected token found.",
			)),
		}
	}
}
impl Display for KeyValue
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
	{
		match self
		{
			KeyValue::String(s) => write!(f, "\"{s}\""),
			KeyValue::Array(a) =>
			{
				let mut result = writeln!(f, "[");

				if result.is_err()
				{
					return result;
				}

				for s in a
				{
					result = writeln!(f, "\t\"{s}\",");

					if result.is_err()
					{
						return result;
					}
				}

				write!(f, "]")
			}
			KeyValue::Table(t) =>
			{
				let mut result = writeln!(f, "{{");

				if result.is_err()
				{
					return result;
				}

				for s in t
				{
					result = writeln!(f, "{},", indent(&s.to_string(), 1));

					if result.is_err()
					{
						return result;
					}
				}

				write!(f, "}}")
			}
		}
	}
}
