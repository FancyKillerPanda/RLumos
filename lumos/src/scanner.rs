// Represents a Scanner object
pub struct Scanner<'a> {
	// source_str is currently unused, may be used in future
	source_str: &'a String,
	source: std::slice::Iter<'a, u8>,
	current_lex: String,
	line: i32
}

impl<'a> Scanner<'a> {
	// Creates a new scanner
	pub fn new(s: &'a String) -> Scanner<'a> {
		Scanner {
			source_str: s,
			source: s.as_bytes().iter(),
			current_lex: String::new(),
			line: 1,
		}
	}

	// Scans for the next token
	pub fn scan_token(&mut self) -> Token {
		self.skip_whitespace();
		
		// Resets the current lexeme
		self.current_lex = String::new();
		
		if self.is_at_end() {
			return self.make_token(TokenType::EoF);
		}

		// Moves to the next character
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

		// Error if it doesn't match any previous character
		return self.error_token("Unexpected character.");
	}

	// Returns true if the scanner has scanned the whole source string
	fn is_at_end(&self) -> bool {
		self.source.clone().next() == None
	}

	// Makes a regular token
	fn make_token(&self, t: TokenType) -> Token {
		Token {
			type_: t,
			string: self.current_lex.clone(),
			line: self.line,
		}
	}

	// Makes an error token
	fn error_token(&self, msg: &str) -> Token {
		Token {
			type_: TokenType::Error,
			string: String::from(msg),
			line: self.line,
		}
	}

	// Advances one character
	fn advance(&mut self) -> char {
		// Gets the next character if it exists
		let next = match self.source.next() {
			Some(t) => t,
			None => return '\0',
		};
		
		// Pushes that character into the current lexeme
		let next = next.clone() as char;
		self.current_lex.push(next);

		next
	}

	// Checks if next character is what is expected
	fn match_next(&mut self, expected: char) -> bool {
		if self.is_at_end() {
			return false;
		}

		// Looks at the next character and compares it
		if self.peek().clone() as char == expected {
			self.current_lex.push(self.peek().clone() as char);
			self.source.next();
			return true;
		}

		false
	}

	// Skips whitespace and comments
	fn skip_whitespace(&mut self) {
		loop {
			// Looks at the next character
			let c = self.peek();

			// Removes comments
			if c == '/' {
				if self.peek_next() == '/' {
					// Comments go to the end of the line
					while self.peek() != '\n' && !self.is_at_end() {
						self.advance();
					}
				}
			}

			// Increases the line count on a newline character
			if c == '\n' {
				self.line += 1;
			}
			
			// Continues if whitespace, stops if not
			if c.is_whitespace() {
				self.advance();
			} else {
				return;
			}
		}
	}

	// Peeks at the next character
	fn peek(&self) -> char {
		// Gets next character if it exists, else returns null
		let next = match self.source.clone().next() {
			Some(t) => t,
			None => return '\0',
		};
		
		next.clone() as char
	}

	// Peeks two characters ahead
	fn peek_next(&self) -> char {
		// Advances by one character
		let mut s = self.source.clone();
		s.next();

		// Gets character after if it exists, else returns null
		let next = match s.next() {
			Some(t) => t,
			None => return '\0',
		};
		
		next.clone() as char
	}

	// Handles string literals
	fn string(&mut self) -> Token {
		// While not the closing quote, advance
		while self.peek() != '"' && !self.is_at_end() {
			// Increments line count on newline
			if self.peek() == '\n' {
				self.line += 1;
			}

			self.advance();
		}

		// String not terminated
		if self.is_at_end() {
			return self.error_token("Unterminated string.");
		}

		// The closing '"'
		self.advance();
		
		self.make_token(TokenType::Str)
	}

	// Handles number literals
	fn number(&mut self) ->  Token {
		// While still part of the number
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

	// Handles identifiers
	fn identifier(&mut self) -> Token {
		// While any letter, number, or underscore
		while self.peek().is_alphanumeric() || self.peek() == '_' {
			self.advance();
		}

		self.make_token(self.identifier_type())
	}

	// Gets the correct type of the identifier
	fn identifier_type(&self) -> TokenType {
		// Gets first character of the identifier
		let mut lex_iter = self.current_lex.as_bytes().iter();
		let next = match lex_iter.next() {
			Some(t) => t,
			None => unreachable!(),
		};

		// Checks if the character matches any keyword
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

	// Checks if a token is a keyword
	fn check_keyword(&self, lex_iter: &mut std::slice::Iter<u8>, rest: &str, type_: TokenType) -> TokenType {
		// Gets an iterator over the rest of the keyword
		let rest = String::from(rest);
		let mut rest_iter = rest.as_bytes().iter();
		
		loop {
			// Advances one character from both strings
			let lex_next = lex_iter.next();
			let rest_next = rest_iter.next();

			// When not the same, return identifier
			if lex_next != rest_next {
				return TokenType::Identifier;
			}

			// If both are None they are equal
			if lex_next == None && rest_next == None {
				return type_;
			}
		}
	}
}


// Represents a Token object
pub struct Token {
	pub type_: TokenType,
	pub string: String,
	pub line: i32,
}

// Represents every type a Token can be
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
