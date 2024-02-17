// token.rs
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

use crate::error::{box_error, CfgResult};

/// The character used to start an inline comment.
pub const COMMENT_CHAR: char = '#';

/// Possible tokens.
#[derive(Debug, PartialEq)]
pub enum Token
{
	Identifier(String),
	String(String),
	Integer(i64),
	Unsigned(u64),
	Float(f64),
	Equals,       // =
	Separator,    // ,
	OpenBracket,  // [
	CloseBracket, // ]
	OpenBrace,    // {
	CloseBrace,   // }
	OpenParen,    // (
	CloseParen,   // )
}
impl Display for Token
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
	{
		match self
		{
			Token::Identifier(s) => write!(f, "{s}"),
			Token::String(s) => write!(f, "\"{s}\""),
			Token::Integer(s) => write!(f, "{s}"),
			Token::Unsigned(s) => write!(f, "{s}"),
			Token::Float(s) => write!(f, "{s}"),
			Token::Equals => write!(f, "="),
			Token::Separator => write!(f, ","),
			Token::OpenBracket => write!(f, "["),
			Token::CloseBracket => write!(f, "]"),
			Token::OpenBrace => write!(f, "{{"),
			Token::CloseBrace => write!(f, "}}"),
			Token::OpenParen => write!(f, "("),
			Token::CloseParen => write!(f, ")"),
		}
	}
}

/// Trait for types that can be loaded from tokens.
pub trait FromTokens
{
	/// Read from `tokens` starting at `index`. Must 'consume' all tokens that are used by
	/// incrementing `index`.
	fn from_tokens(tokens: &Vec<Token>, index: &mut usize) -> CfgResult<Self>
	where
		Self: Sized;
}

enum NumberType
{
	Integer,
	Unsigned,
	Float,
}

/// Parses the string `s` into a vector of tokens wrapped in [`Ok`] on success, otherwise an error
/// wrapped in [`Err`] on failure.
pub fn string_to_tokens(s: &str) -> CfgResult<Vec<Token>>
{
	let chars: Vec<char> = s.chars().collect();

	let slen = s.len();

	if chars.len() != slen
	{
		return Err(box_error(
			"Unable to parse strings containing multi-byte characters to tokens.",
		));
	}

	let mut result: Vec<Token> = Vec::new();
	let mut i = 0;

	while i < slen
	{
		if chars[i].is_whitespace()
		{
			i += 1;
			continue;
		}
		if chars[i] == COMMENT_CHAR
		{
			i = match s[i + 1..].find('\n')
			{
				Some(e) => e + i + 2,
				None => slen,
			};

			continue;
		}

		let numdot = chars[i] == '.' && (i + 1) < slen && chars[i + 1].is_ascii_digit();

		if numdot || chars[i].is_ascii_digit()
		{
			let mut hasdot = numdot;
			let mut end = i + 1;

			let mut numtype: Option<NumberType> = None;

			while end < slen
			{
				if chars[end] == '.'
				{
					if hasdot
					{
						return Err(box_error("Number has multiple decimal points."));
					}

					hasdot = true;
					end += 1;
					continue;
				}

				if !chars[end].is_ascii_digit()
				{
					numtype = match chars[end]
					{
						'i' | 'I' => Some(NumberType::Integer),
						'u' | 'U' => Some(NumberType::Unsigned),
						'f' | 'F' => Some(NumberType::Float),
						_ => None,
					};

					break;
				}

				end += 1;
			}

			let inc = numtype.is_some();

			if numtype.is_none()
			{
				numtype = Some(
					if hasdot
					{
						NumberType::Float
					}
					else
					{
						NumberType::Integer
					},
				);
			}

			let rstr = if numdot
			{
				"0".to_owned() + &s[i..end]
			}
			else
			{
				s[i..end].to_owned()
			};

			match numtype.unwrap()
			{
				NumberType::Integer =>
				{
					let r = {
						if hasdot
						{
							match rstr.parse::<f64>()
							{
								Ok(r) => r as i64,
								Err(e) =>
								{
									return Err(box_error(&format!("Failed parsing float: {e}.")))
								}
							}
						}
						else
						{
							match rstr.parse::<i64>()
							{
								Ok(r) => r,
								Err(e) =>
								{
									return Err(box_error(&format!("Failed parsing integer: {e}.")))
								}
							}
						}
					};

					result.push(Token::Integer(r));
				}
				NumberType::Unsigned =>
				{
					let r = {
						if hasdot
						{
							match rstr.parse::<f64>()
							{
								Ok(r) => r as u64,
								Err(e) =>
								{
									return Err(box_error(&format!("Failed parsing float: {e}.")))
								}
							}
						}
						else
						{
							match rstr.parse::<u64>()
							{
								Ok(r) => r,
								Err(e) =>
								{
									return Err(box_error(&format!(
										"Failed parsing unsigned integer: {e}."
									)))
								}
							}
						}
					};

					result.push(Token::Unsigned(r));
				}
				NumberType::Float =>
				{
					let r = match rstr.parse::<f64>()
					{
						Ok(r) => r,
						Err(e) => return Err(box_error(&format!("Failed parsing float: {e}."))),
					};

					result.push(Token::Float(r));
				}
			}

			i = end;

			if inc
			{
				i += 1;
			}

			continue;
		}
		else if chars[i].is_ascii_alphabetic() || chars[i] == '_'
		{
			let mut end = i + 1;

			while end < slen
			{
				if !chars[end].is_ascii_alphabetic()
					&& !chars[end].is_ascii_alphanumeric()
					&& chars[end] != '_'
				{
					break;
				}

				end += 1;
			}

			result.push(Token::Identifier(String::from(&s[i..end])));
			i = end;
			continue;
		}
		else if chars[i] == '='
		{
			result.push(Token::Equals);
		}
		else if chars[i] == ','
		{
			result.push(Token::Separator);
		}
		else if chars[i] == '['
		{
			result.push(Token::OpenBracket);
		}
		else if chars[i] == ']'
		{
			result.push(Token::CloseBracket);
		}
		else if chars[i] == '{'
		{
			result.push(Token::OpenBrace);
		}
		else if chars[i] == '}'
		{
			result.push(Token::CloseBrace);
		}
		else if chars[i] == '('
		{
			result.push(Token::OpenParen);
		}
		else if chars[i] == ')'
		{
			result.push(Token::CloseParen);
		}
		else if chars[i] == '"'
		{
			let end = match s[i + 1..].find('"')
			{
				Some(e) => e + i + 1,
				None => return Err(box_error("String has no ending quote.")),
			};

			let val = String::from(&s[i + 1..end]);

			let laststr = match result.last()
			{
				Some(l) => match l
				{
					Token::String(s) => Some(s.clone()),
					_ => None,
				},
				_ => None,
			};

			let rlen = result.len();

			if let Some(s) = laststr
			{
				result[rlen - 1] = Token::String(s + &val);
			}
			else
			{
				result.push(Token::String(val));
			}

			i = end;
		}
		else
		{
			return Err(box_error(&format!("Unrecognised token: {}", chars[i])));
		}

		i += 1;
	}

	Ok(result)
}
