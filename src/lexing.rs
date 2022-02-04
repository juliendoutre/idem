use std::str::Chars;

#[derive(Debug)]
pub struct LocatedToken {
	pub location: Location,
	pub token: Token,
}

#[derive(Debug, Clone)]
pub struct Location {
	pub path: String,
	pub line: usize,
	pub column: usize,
}

#[derive(Debug)]
pub enum Token {
	Word(String),
	Symbol(Symbol),
}

#[derive(Debug)]
pub enum Symbol {
	OpeningParenthesis,
	ClosingParenthesis,
	OpeningBrace,
	ClosingBrace,
	OpeningBracket,
	ClosingBracket,
	Comma,
	Colon,
}

pub struct Lexer {
	path: String,
	line: usize,
	column: usize,
	word: Option<String>,
	in_comment: bool,
}

impl Lexer {
	pub fn new(path: &str) -> Self {
		Self {
			path: path.to_owned(),
			line: 1,
			column: 1,
			word: None,
			in_comment: false,
		}
	}

	pub fn tokenize(&mut self, source: Chars) -> Vec<LocatedToken> {
		let mut tokens = Vec::new();

		for c in source {
			if self.in_comment {
				match c {
					'\n' => {
						self.in_comment = false;
						self.line += 1;
						self.column = 0;
					}
					_ => {
						self.column += 1;
					}
				}
			} else {
				match c {
					'(' => {
						self.extend_word(&mut tokens);
						tokens.push(self.wrap(Token::Symbol(Symbol::OpeningParenthesis)));
						self.column += 1;
					}
					')' => {
						self.extend_word(&mut tokens);
						tokens.push(self.wrap(Token::Symbol(Symbol::ClosingParenthesis)));
						self.column += 1;
					}
					'{' => {
						self.extend_word(&mut tokens);
						tokens.push(self.wrap(Token::Symbol(Symbol::OpeningBrace)));
						self.column += 1;
					}
					'}' => {
						self.extend_word(&mut tokens);
						tokens.push(self.wrap(Token::Symbol(Symbol::ClosingBrace)));
						self.column += 1;
					}
					'[' => {
						self.extend_word(&mut tokens);
						tokens.push(self.wrap(Token::Symbol(Symbol::OpeningBracket)));
						self.column += 1;
					}
					']' => {
						self.extend_word(&mut tokens);
						tokens.push(self.wrap(Token::Symbol(Symbol::ClosingBracket)));
						self.column += 1;
					}
					',' => {
						self.extend_word(&mut tokens);
						tokens.push(self.wrap(Token::Symbol(Symbol::Comma)));
						self.column += 1;
					}
					':' => {
						self.extend_word(&mut tokens);
						tokens.push(self.wrap(Token::Symbol(Symbol::Colon)));
						self.column += 1;
					}
					'#' => {
						self.in_comment = true;
						self.column += 1;
					}
					'\n' => {
						self.extend_word(&mut tokens);
						self.line += 1;
						self.column = 1;
					}
					'\t' => {
						self.extend_word(&mut tokens);
						self.column += 4;
					}
					' ' => {
						self.extend_word(&mut tokens);
						self.column += 1;
					}
					_ => {
						if let Some(w) = &mut self.word {
							w.push(c);
						} else {
							self.word = Some(String::from_iter([c]));
						}
					}
				}
			}
		}

		tokens
	}

	fn extend_word(&mut self, tokens: &mut Vec<LocatedToken>) {
		if let Some(w) = &self.word {
			tokens.push(self.wrap(Token::Word(w.to_string())));
			self.column += w.len();
			self.word = None;
		}
	}

	fn wrap(&self, token: Token) -> LocatedToken {
		LocatedToken {
			token,
			location: Location {
				path: self.path.to_owned(),
				line: self.line,
				column: self.column,
			},
		}
	}
}
