pub struct Scanner<'a> {
	source_str: &'a String,
	source: std::slice::Iter<'a, u8>,
	current_lex: String,
	line: i32
}

impl<'a> Scanner<'a> {
	pub fn new(s: &'a String) -> Scanner<'a> {
		Scanner {
			source_str: s,
			source: s.as_bytes().iter(),
			current_lex: String::new(),
			line: 1,
		}
	}

	pub fn scan_token(&mut self) -> Token {
		self.skip_whitespace();
		
		self.current_lex = String::new();
		
		if self.is_at_end() {
			return self.make_token(TokenType::EoF);
		}

		let c = self.advance();

		if c.is_numeric() {
			return self.number();
		}

		if c.is_alphabetic() || c == '_' {
			return self.identifier();
		}

		match c {
			'(' => return self.make_token(TokenType::LeftParen),
			')' => return self.make_token(TokenType::RightParen),
			'{' => return self.make_token(TokenType::LeftBrace),
			'}' => return self.make_token(TokenType::RightBrace),
			';' => return self.make_token(TokenType::Semicolon),
			',' => return self.make_token(TokenType::Comma),
			'.' => return self.make_token(TokenType::Dot),
			'-' => return self.make_token(TokenType::Minus),
			'+' => return self.make_token(TokenType::Plus),
			'/' => return self.make_token(TokenType::Slash),
			'*' => return self.make_token(TokenType::Star),
			'!' => {
				if self.match_next('=') {
					return self.make_token(TokenType::BangEqual);
				} else {
					return self.make_token(TokenType::Bang);
				}
			},
			'=' => {
				if self.match_next('=') {
					return self.make_token(TokenType::EqualEqual);
				} else {
					return self.make_token(TokenType::Equal);
				}
			},
			'>' => {
				if self.match_next('=') {
					return self.make_token(TokenType::GreaterEqual);
				} else {
					return self.make_token(TokenType::Greater);
				}
			},
			'<' => {
				if self.match_next('=') {
					return self.make_token(TokenType::LessEqual);
				} else {
					return self.make_token(TokenType::Less);
				}
			},
			'"' => return self.string(),
			_ => (),
		}

		return self.error_token("Unexpected character.");
	}

	fn is_at_end(&self) -> bool {
		self.source.clone().next() == None
	}

	fn make_token(&self, t: TokenType) -> Token {
		Token {
			type_: t,
			string: self.current_lex.clone(),
			line: self.line,
		}
	}

	fn error_token(&self, msg: &str) -> Token {
		Token {
			type_: TokenType::Error,
			string: String::from(msg),
			line: self.line,
		}
	}

	fn advance(&mut self) -> char {
		let next = match self.source.next() {
			Some(t) => t,
			None => return '\0',
		};
		
		let next = next.clone() as char;

		self.current_lex.push(next);
		next
	}

	fn match_next(&mut self, expected: char) -> bool {
		if self.is_at_end() {
			return false;
		}

		let next = match self.source.clone().next() {
			Some(t) => t,
			None => return false,
		};

		if next.clone() as char == expected {
			self.current_lex.push(next.clone() as char);
			self.source.next();
			return true;
		}

		false
	}

	fn skip_whitespace(&mut self) {
		loop {
			let c = self.peek();

			if c == '/' {
				if self.peek_next() == '/' {
					while self.peek() != '\n' && !self.is_at_end() {
						self.advance();
					}
				}
			}

			if c == '\n' {
				self.line += 1;
			}
			
			if c.is_whitespace() {
				self.advance();
			} else {
				return;
			}
		}
	}

	fn peek(&self) -> char {
		let next = match self.source.clone().next() {
			Some(t) => t,
			None => return '\0',
		};
		
		next.clone() as char
	}

	fn peek_next(&self) -> char {
		let mut s = self.source.clone();
		s.next();

		let next = match s.next() {
			Some(t) => t,
			None => return '\0',
		};
		
		next.clone() as char
	}

	fn string(&mut self) -> Token {
		while self.peek() != '"' && !self.is_at_end() {
			if self.peek() == '\n' {
				self.line += 1;
			}

			self.advance();
		}

		if self.is_at_end() {
			return self.error_token("Unterminated string.");
		}

		// The closing '"'
		self.advance();
		
		self.make_token(TokenType::Str)
	}

	fn number(&mut self) ->  Token {
		while self.peek().is_numeric() {
			self.advance();
		}

		// Look for a fractional part
		if self.peek() == '.' && self.peek_next().is_numeric() {
			// Consume the '.'
			self.advance();
			
			while self.peek().is_numeric() {
				self.advance();
			}
		}

		self.make_token(TokenType::Number)
	}

	fn identifier(&mut self) -> Token {
		while self.peek().is_alphanumeric() || self.peek() == '_' {
			self.advance();
		}

		self.make_token(self.identifier_type())
	}

	fn identifier_type(&self) -> TokenType {
		let mut lex_iter = self.current_lex.as_bytes().iter();
		let next = match lex_iter.next() {
			Some(t) => t,
			None => unreachable!(),
		};

		match next.clone() as char {                                  
			'a' => return self.check_keyword(&mut lex_iter, "nd", TokenType::And),
			'c' => return self.check_keyword(&mut lex_iter, "lass", TokenType::Class),
			'e' => return self.check_keyword(&mut lex_iter, "lse", TokenType::Else),
			'f' => {
				let next = match lex_iter.next() {
					Some(t) => t,
					None => return TokenType::Identifier,
				};

				match next.clone() as char {
					'a' => return self.check_keyword(&mut lex_iter, "lse", TokenType::False),
					'o' => return self.check_keyword(&mut lex_iter, "r", TokenType::For),
					'u' => return self.check_keyword(&mut lex_iter, "n", TokenType::Fun),
					_ => return TokenType::Identifier,
				}
			},
			'i' => return self.check_keyword(&mut lex_iter, "f", TokenType::If),
			'n' => return self.check_keyword(&mut lex_iter, "il", TokenType::Nil),
			'o' => return self.check_keyword(&mut lex_iter, "r", TokenType::Or),
			'p' => return self.check_keyword(&mut lex_iter, "rint", TokenType::Print),
			'r' => return self.check_keyword(&mut lex_iter, "eturn", TokenType::Return),
			's' => return self.check_keyword(&mut lex_iter, "uper", TokenType::Super),
			't' => {
				let next = match lex_iter.next() {
					Some(t) => t,
					None => return TokenType::Identifier,
				};

				match next.clone() as char {
					'h' => return self.check_keyword(&mut lex_iter, "is", TokenType::This),
					'r' => return self.check_keyword(&mut lex_iter, "ue", TokenType::True),
					_ => return TokenType::Identifier,
				}
			},
			'v' => return self.check_keyword(&mut lex_iter, "ar", TokenType::Var),
			'w' => return self.check_keyword(&mut lex_iter, "hile", TokenType::While),
			_ => return TokenType::Identifier,
		}                                                            
	}

	fn check_keyword(&self, lex_iter: &mut std::slice::Iter<u8>, rest: &str, type_: TokenType) -> TokenType {
		let rest = String::from(rest);
		let mut rest_iter = rest.as_bytes().iter();
		
		loop {
			let lex_next = lex_iter.next();
			let rest_next = rest_iter.next();

			if lex_next != rest_next {
				return TokenType::Identifier;
			}

			if lex_next == None && rest_next == None {
				return type_;
			}
		}
	}
}


pub struct Token {
	pub type_: TokenType,
	pub string:String,
	pub line: i32,
}

#[derive(Copy, Clone)]
pub enum TokenType {
	// Single-character tokens
	LeftParen, RightParen,
	LeftBrace, RightBrace,
	Comma, Dot, Minus, Plus,
	Semicolon, Slash, Star,

	// one or two character tokens
	Bang, BangEqual,
	Equal, EqualEqual,
	Greater, GreaterEqual,
	Less, LessEqual,                       

	// literals
	Identifier, Str, Number,

	// keywords
	And, Class, Else, False,
	For, Fun, If, Nil, Or,
	Print, Return, Super, This,
	True, Var, While,

	Error,
	EoF,
}
