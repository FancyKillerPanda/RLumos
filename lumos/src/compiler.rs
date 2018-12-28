use super::scanner::{ Scanner, TokenType };

pub fn compile(source: &String) {
	// Creates a new scanner
	let mut scanner = Scanner::new(source);
	let mut line = -1;

	// While not an EoF token
	loop {
		// Scans the next token
		let token = scanner.scan_token();

		// Prints the line if it is not the same as before
		if token.line != line {
			print!("{:04} ", token.line);
			line = token.line;
		} else {
			print!("   | ");
		}

		// Prints the token's information
		println!("{:02} '{:.*}'", token.type_ as usize, token.string.len(), token.string);

		// Breaks on an EoF token
		if token.type_ as usize == TokenType::EoF as usize {
			break;
		}
	}
}
