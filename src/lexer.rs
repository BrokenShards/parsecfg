// lexer.rs //

use std::{collections::VecDeque, fs};

use crate::{
	error::{box_error, CfgResult},
	Token, COMMENT_CHAR,
};

enum NumberType
{
	Integer,
	Unsigned,
	Float,
}

pub struct Lexer
{
	tokens: VecDeque<Token>,
}

impl Lexer
{
	pub fn new() -> Self
	{
		Self {
			tokens: VecDeque::new(),
		}
	}

	pub fn parse_string(&mut self, s: &str) -> CfgResult<()>
	{
		let chars: Vec<char> = s.chars().collect();

		let slen = s.len();

		if chars.len() != slen
		{
			return Err(box_error(
				"Unable to parse strings containing multi-byte characters to tokens.",
			));
		}

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
										return Err(box_error(&format!(
											"Failed parsing float: {e}."
										)))
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
										return Err(box_error(&format!(
											"Failed parsing integer: {e}."
										)))
									}
								}
							}
						};

						self.tokens.push_back(Token::Integer(r));
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
										return Err(box_error(&format!(
											"Failed parsing float: {e}."
										)))
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

						self.tokens.push_back(Token::Unsigned(r));
					}
					NumberType::Float =>
					{
						let r = match rstr.parse::<f64>()
						{
							Ok(r) => r,
							Err(e) =>
							{
								return Err(box_error(&format!("Failed parsing float: {e}.")))
							}
						};

						self.tokens.push_back(Token::Float(r));
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

				self.tokens
					.push_back(Token::Identifier(String::from(&s[i..end])));
				i = end;
				continue;
			}
			else if chars[i] == '='
			{
				self.tokens.push_back(Token::Equals);
			}
			else if chars[i] == ','
			{
				self.tokens.push_back(Token::Separator);
			}
			else if chars[i] == '+'
			{
				self.tokens.push_back(Token::Add);
			}
			else if chars[i] == '-'
			{
				self.tokens.push_back(Token::Subtract);
			}
			else if chars[i] == '*'
			{
				self.tokens.push_back(Token::Multiply);
			}
			else if chars[i] == '/'
			{
				self.tokens.push_back(Token::Divide);
			}
			else if chars[i] == '%'
			{
				self.tokens.push_back(Token::Modulo);
			}
			else if chars[i] == '['
			{
				self.tokens.push_back(Token::OpenBracket);
			}
			else if chars[i] == ']'
			{
				self.tokens.push_back(Token::CloseBracket);
			}
			else if chars[i] == '{'
			{
				self.tokens.push_back(Token::OpenBrace);
			}
			else if chars[i] == '}'
			{
				self.tokens.push_back(Token::CloseBrace);
			}
			else if chars[i] == '('
			{
				self.tokens.push_back(Token::OpenParen);
			}
			else if chars[i] == ')'
			{
				self.tokens.push_back(Token::CloseParen);
			}
			else if chars[i] == '"'
			{
				let end = match s[i + 1..].find('"')
				{
					Some(e) => e + i + 1,
					None => return Err(box_error("String has no ending quote.")),
				};

				let val = String::from(&s[i + 1..end]);

				let laststr = match &self.tokens[self.tokens.len() - 1]
				{
					Token::String(s) => Some(s.clone()),
					_ => None,
				};

				let rlen = self.tokens.len();

				if let Some(s) = laststr
				{
					self.tokens[rlen - 1] = Token::String(s + &val);
				}
				else
				{
					self.tokens.push_back(Token::String(val));
				}

				i = end;
			}
			else
			{
				return Err(box_error(&format!("Unrecognised token: {}", chars[i])));
			}

			i += 1;
		}

		Ok(())
	}
	pub fn parse_file(&mut self, path: &str) -> CfgResult<()>
	{
		match fs::read_to_string(path)
		{
			Ok(s) => self.parse_string(&s),
			Err(e) => Err(box_error(&format!("Unable to parse file to tokens: {e}.",))),
		}
	}
	pub fn clear(&mut self) { self.tokens.clear(); }

	pub fn is_empty(&self) -> bool { self.tokens.is_empty() }
	pub fn len(&self) -> usize { self.tokens.len() }
	pub fn push_front(&mut self, token: Token) { self.tokens.push_front(token); }
	pub fn pop_front(&mut self) -> Option<Token> { self.tokens.pop_front() }
	pub fn peek(&self) -> Option<&Token>
	{
		if self.is_empty()
		{
			None
		}
		else
		{
			Some(&self.tokens[0])
		}
	}
	pub fn peek_to(&self, count: usize) -> Vec<&Token>
	{
		let mut vector: Vec<&Token> = Vec::new();

		let count = if count < self.len()
		{
			count
		}
		else
		{
			self.len()
		};

		let mut i = 0;

		while i < count
		{
			vector.push(&self.tokens[i]);
			i += 1;
		}

		vector
	}
	pub fn check(&self, check: fn(&Token) -> bool) -> bool
	{
		if self.is_empty()
		{
			false
		}
		else
		{
			check(&self.tokens[0])
		}
	}

	pub fn expect(&mut self, check: fn(&Token) -> bool, msg: &str) -> CfgResult<Token>
	{
		if self.is_empty()
		{
			return Err(box_error(&format!("Expected token but lexer is empty.",)));
		}

		if !self.check(check)
		{
			return Err(box_error(msg));
		}
		else
		{
			Ok(self.pop_front().unwrap())
		}
	}
}

/// Trait for types that can be loaded from tokens.
pub trait FromLexer
{
	/// Read tokens from `lexer` to create an instance of `Self`.
	fn from_lexer(lexer: &mut Lexer) -> CfgResult<Self>
	where
		Self: Sized;
}
