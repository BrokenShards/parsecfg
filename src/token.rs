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

/// The character used to start an inline comment.
pub const COMMENT_CHAR: char = '#';

/// Possible tokens.
#[derive(Clone, Debug, PartialEq)]
pub enum Token
{
	Identifier(String),
	String(String),
	Integer(i64),
	Unsigned(u64),
	Float(f64),
	Equals,       // =
	Separator,    // ,
	Add,          // +
	Subtract,     // -
	Multiply,     // *
	Divide,       // /
	Modulo,       // %
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
			Token::Add => write!(f, "+"),
			Token::Subtract => write!(f, "-"),
			Token::Multiply => write!(f, "*"),
			Token::Divide => write!(f, "/"),
			Token::Modulo => write!(f, "%"),
			Token::OpenBracket => write!(f, "["),
			Token::CloseBracket => write!(f, "]"),
			Token::OpenBrace => write!(f, "{{"),
			Token::CloseBrace => write!(f, "}}"),
			Token::OpenParen => write!(f, "("),
			Token::CloseParen => write!(f, ")"),
		}
	}
}
