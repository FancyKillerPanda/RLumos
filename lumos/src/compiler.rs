use super::scanner::{ Scanner, TokenType };

pub fn compile(source: &String) {
	let mut scanner = Scanner::new(source);
	let mut line = -1;

	loop {
		let token = scanner.scan_token();

		if token.line != line {
			print!("{:04} ", token.line);
			line = token.line;
		} else {
			print!("   | ");
		}

		println!("{:02} '{:.*}'", token.type_ as usize, token.string.len(), token.string);

		if token.type_ as usize == TokenType::EoF as usize {
			break;
		}
	}
}
