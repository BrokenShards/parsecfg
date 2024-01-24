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
#[derive(Debug, PartialEq, Eq)]
pub enum Token
{
	Identifier(String),
	String(String),
	Equals,       // =
	Separator,    // ,
	OpenBracket,  // [
	CloseBracket, // ]
	OpenBrace,    // {
	CloseBrace,   // }
}
impl Display for Token
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
	{
		match self
		{
			Token::Identifier(s) => write!(f, "{s}"),
			Token::String(s) => write!(f, "\"{s}\""),
			Token::Equals => write!(f, "="),
			Token::Separator => write!(f, ","),
			Token::OpenBracket => write!(f, "["),
			Token::CloseBracket => write!(f, "]"),
			Token::OpenBrace => write!(f, "{{"),
			Token::CloseBrace => write!(f, "}}"),
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

/// Parses the string `s` into a vector of tokens wrapped in [`Ok`] on success, otherwise an error
/// wrapped in [`Err`] on failure.
pub fn string_to_tokens(s: &str) -> CfgResult<Vec<Token>>
{
	let chars: Vec<char> = s.chars().collect();

	if chars.len() != s.len()
	{
		return Err(box_error(
			"Unable to parse strings containing multi-byte characters to tokens.",
		));
	}

	let mut result: Vec<Token> = Vec::new();
	let mut i = 0;

	while i < s.len()
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
				None => s.len(),
			};

			continue;
		}

		if chars[i] == '='
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
		else if chars[i] == '"'
		{
			let end = match s[i + 1..].find('"')
			{
				Some(e) => e + i + 1,
				None => return Err(box_error("String has no ending quote.")),
			};

			result.push(Token::String(String::from(&s[i + 1..end])));
			i = end;
		}
		else if chars[i].is_ascii_alphabetic() || chars[i] == '_'
		{
			let mut end = i + 1;

			while end < s.len()
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
		else
		{
			return Err(box_error(&format!("Unrecognised token: {}", chars[i])));
		}

		i += 1;
	}

	Ok(result)
}
