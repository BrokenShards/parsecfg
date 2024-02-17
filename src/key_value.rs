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
#[derive(Clone, Debug, PartialEq)]
pub enum KeyValue
{
	String(String),
	Integer(i64),
	Unsigned(u64),
	Float(f64),

	StringArray(Vec<String>),
	IntegerArray(Vec<i64>),
	UnsignedArray(Vec<u64>),
	FloatArray(Vec<f64>),

	Tuple(Vec<KeyValue>),
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
				Ok(Self::String(s.clone()))
			}
			Token::Integer(s) =>
			{
				*index += 1;
				Ok(Self::Integer(*s))
			}
			Token::Unsigned(s) =>
			{
				*index += 1;
				Ok(Self::Unsigned(*s))
			}
			Token::Float(s) =>
			{
				*index += 1;
				Ok(Self::Float(*s))
			}
			Token::OpenBracket =>
			{
				*index += 1;

				if *index >= len
				{
					return Err(box_error("Unexpected end of tokens: Incomplete Array."));
				}

				match &tokens[*index]
				{
					Token::String(_) =>
					{
						let mut ready = true;
						let mut closed = false;
						let mut result: Vec<String> = Vec::new();

						while *index < len
						{
							match &tokens[*index]
							{
								Token::String(s) =>
								{
									if !ready
									{
										return Err(box_error(
											"Unexpected token; expected separator or close \
											 bracket.",
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
											"Unexpected token; expected string or close bracket.",
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
										"Unexpected token: {}.",
										&tokens[*index]
									)))
								}
							}

							*index += 1;
						}

						if !closed
						{
							Err(box_error("StringArray missing closing square bracket."))
						}
						else
						{
							Ok(Self::StringArray(result))
						}
					}
					Token::Integer(_) =>
					{
						let mut ready = true;
						let mut closed = false;
						let mut result: Vec<i64> = Vec::new();

						while *index < len
						{
							match &tokens[*index]
							{
								Token::Integer(s) =>
								{
									if !ready
									{
										return Err(box_error(
											"Unexpected token; expected separator or close \
											 bracket.",
										));
									}
									result.push(*s);
									ready = false;
								}
								Token::Separator =>
								{
									if ready
									{
										return Err(box_error(
											"Unexpected token; expected integer or close bracket.",
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
										"Unexpected token: {}.",
										&tokens[*index]
									)))
								}
							}

							*index += 1;
						}

						if !closed
						{
							Err(box_error("IntegerArray missing closing square bracket."))
						}
						else
						{
							Ok(Self::IntegerArray(result))
						}
					}
					Token::Unsigned(_) =>
					{
						let mut ready = true;
						let mut closed = false;
						let mut result: Vec<u64> = Vec::new();

						while *index < len
						{
							match &tokens[*index]
							{
								Token::Unsigned(s) =>
								{
									if !ready
									{
										return Err(box_error(
											"Unexpected token; expected separator or close \
											 bracket.",
										));
									}
									result.push(*s);
									ready = false;
								}
								Token::Separator =>
								{
									if ready
									{
										return Err(box_error(
											"Unexpected token; expected unsigned integer or close \
											 bracket.",
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
										"Unexpected token: {}.",
										&tokens[*index]
									)))
								}
							}

							*index += 1;
						}

						if !closed
						{
							Err(box_error("UnsignedArray missing closing square bracket."))
						}
						else
						{
							Ok(Self::UnsignedArray(result))
						}
					}
					Token::Float(_) =>
					{
						let mut ready = true;
						let mut closed = false;
						let mut result: Vec<f64> = Vec::new();

						while *index < len
						{
							match &tokens[*index]
							{
								Token::Float(s) =>
								{
									if !ready
									{
										return Err(box_error(
											"Unexpected token; expected separator or close \
											 bracket.",
										));
									}
									result.push(*s);
									ready = false;
								}
								Token::Separator =>
								{
									if ready
									{
										return Err(box_error(
											"Unexpected token; expected float or close bracket.",
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
										"Unexpected token: {}.",
										&tokens[*index]
									)))
								}
							}

							*index += 1;
						}

						if !closed
						{
							Err(box_error("FloatArray missing closing square bracket."))
						}
						else
						{
							Ok(Self::FloatArray(result))
						}
					}
					Token::CloseBracket =>
					{
						*index += 1;
						Ok(Self::StringArray(vec![]))
					}
					_ => Err(box_error(
						"Unexpected token; expected value or close bracket.",
					)),
				}
			}
			Token::OpenParen =>
			{
				let mut result: Vec<KeyValue> = Vec::new();
				let mut ready = true;
				let mut closed = false;

				*index += 1;

				while *index < len
				{
					if tokens[*index] == Token::CloseParen
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
							"Unexpected token: {}. Expected comma.",
							&tokens[*index]
						)));
					}

					let key = KeyValue::from_tokens(&tokens, index)?;
					result.push(key);
					ready = false;
				}

				if !closed
				{
					Err(box_error("Tuple missing closing parenthesis."))
				}
				else
				{
					Ok(Self::Tuple(result))
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
							"Unexpected token: {}. Expected comma.",
							&tokens[*index]
						)));
					}

					let key = Key::from_tokens(&tokens, index)?;

					if !key.is_valid()
					{
						return Err(box_error(&format!(
							"Parsed Key: {} invalid in Table.",
							&key.name()
						)));
					}

					result.push(key);
					ready = false;
				}

				if !closed
				{
					Err(box_error("Table missing closing bracket."))
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
			KeyValue::Integer(s) => write!(f, "{s}"),
			KeyValue::Unsigned(s) => write!(f, "{s}"),
			KeyValue::Float(s) => write!(f, "{s}"),
			KeyValue::StringArray(a) =>
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
			KeyValue::IntegerArray(a) =>
			{
				let mut result = writeln!(f, "[");

				if result.is_err()
				{
					return result;
				}

				for s in a
				{
					result = writeln!(f, "\t{s},");

					if result.is_err()
					{
						return result;
					}
				}

				write!(f, "]")
			}
			KeyValue::UnsignedArray(a) =>
			{
				let mut result = writeln!(f, "[");

				if result.is_err()
				{
					return result;
				}

				for s in a
				{
					result = writeln!(f, "\t{s},");

					if result.is_err()
					{
						return result;
					}
				}

				write!(f, "]")
			}
			KeyValue::FloatArray(a) =>
			{
				let mut result = writeln!(f, "[");

				if result.is_err()
				{
					return result;
				}

				for s in a
				{
					result = writeln!(f, "\t{s},");

					if result.is_err()
					{
						return result;
					}
				}

				write!(f, "]")
			}
			KeyValue::Tuple(t) =>
			{
				let mut result = writeln!(f, "(");

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

				write!(f, ")")
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
