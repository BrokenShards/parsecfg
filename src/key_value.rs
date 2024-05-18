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
	indent,
	lexer::{FromLexer, Lexer},
	Key, Token,
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
impl FromLexer for KeyValue
{
	fn from_lexer(lexer: &mut Lexer) -> CfgResult<Self>
	where
		Self: Sized,
	{
		if lexer.is_empty()
		{
			return Err(box_error("Trying to load KeyValue from an empty lexer."));
		}

		let token = lexer.pop_front().unwrap();

		match &token
		{
			Token::String(s) => Ok(Self::String(s.clone())),
			Token::Integer(s) => Ok(Self::Integer(*s)),
			Token::Unsigned(s) => Ok(Self::Unsigned(*s)),
			Token::Float(s) => Ok(Self::Float(*s)),
			Token::OpenBracket =>
			{
				if lexer.is_empty()
				{
					return Err(box_error("Unexpected end of tokens: Incomplete Array."));
				}

				let tok = lexer.pop_front().unwrap();

				match &tok
				{
					Token::String(_) =>
					{
						let mut first = true;
						let mut ready = true;
						let mut closed = false;
						let mut result: Vec<String> = Vec::new();

						while !lexer.is_empty()
						{
							let t = if first
							{
								first = false;
								tok.clone()
							}
							else
							{
								lexer.pop_front().unwrap()
							};

							match &t
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
									break;
								}
								_ => return Err(box_error(&format!("Unexpected token: {}.", t))),
							}
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
						let mut first = true;
						let mut ready = true;
						let mut closed = false;
						let mut result: Vec<i64> = Vec::new();

						while !lexer.is_empty()
						{
							let t = if first
							{
								first = false;
								tok.clone()
							}
							else
							{
								lexer.pop_front().unwrap()
							};

							match &t
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
									break;
								}
								_ => return Err(box_error("Unexpected token.")),
							}
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
						let mut first = true;
						let mut ready = true;
						let mut closed = false;
						let mut result: Vec<u64> = Vec::new();

						while !lexer.is_empty()
						{
							let t = if first
							{
								first = false;
								tok.clone()
							}
							else
							{
								lexer.pop_front().unwrap()
							};

							match &t
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
									break;
								}
								_ => return Err(box_error("Unexpected token.")),
							}
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
						let mut first = true;
						let mut ready = true;
						let mut closed = false;
						let mut result: Vec<f64> = Vec::new();

						while !lexer.is_empty()
						{
							let t = if first
							{
								first = false;
								tok.clone()
							}
							else
							{
								lexer.pop_front().unwrap()
							};

							match &t
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
									break;
								}
								_ => return Err(box_error("Unexpected token.")),
							}
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
					Token::CloseBracket => Ok(Self::StringArray(vec![])),
					_ =>
					{
						return Err(box_error(
							"Unexpected token; expected value or close bracket.",
						))
					}
				}
			}
			Token::OpenParen =>
			{
				let mut result: Vec<KeyValue> = Vec::new();
				let mut ready = true;
				let mut closed = false;

				while !lexer.is_empty()
				{
					let tok = lexer.peek().unwrap();

					if tok == &Token::CloseParen
					{
						closed = true;
						lexer.pop_front();
						break;
					}

					if !ready
					{
						if tok == &Token::Separator
						{
							ready = true;
							lexer.pop_front();
							continue;
						}

						return Err(box_error(&format!(
							"Unexpected token: {}. Expected comma.",
							lexer.pop_front().unwrap()
						)));
					}

					let key = KeyValue::from_lexer(lexer)?;
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

				while !lexer.is_empty()
				{
					let tok = lexer.peek().unwrap();

					if tok == &Token::CloseBrace
					{
						closed = true;
						lexer.pop_front();
						break;
					}

					if !ready
					{
						if tok == &Token::Separator
						{
							ready = true;
							lexer.pop_front();
							continue;
						}

						return Err(box_error(&format!(
							"Unexpected token: {}. Expected comma.",
							tok
						)));
					}

					let key = Key::from_lexer(lexer)?;

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
