#[derive(Debug, PartialEq, Clone)]
pub struct Item {
    id: i64,
    name: String,
    quantity: i64,
}

#[cfg(test)]
mod tests {
    // Thanks to https://github.com/harrelchris/eveparse/blob/main/tests/test_parse.py for many of these test cases.

    use super::*;

    #[test]
    fn lex_name_only() {
        assert_eq!(
            lex("Paladin").unwrap(),
            vec![string(String::from("Paladin")), eof(),]
        );
    }
    #[test]
    fn lex_name_space_quantity() {
        assert_eq!(
            lex("Paladin 2").unwrap(),
            vec![
                string(String::from("Paladin")),
                space(),
                number(String::from("2")),
                eof(),
            ]
        );
    }
    #[test]
    fn lex_name_star_space_quantity() {
        assert_eq!(
            lex("Paladin* 2").unwrap(),
            vec![
                string(String::from("Paladin*")),
                space(),
                number(String::from("2")),
                eof(),
            ]
        );
    }
    #[test]
    fn lex_name_space_x_quantity() {
        assert_eq!(
            lex("Paladin x2").unwrap(),
            vec![
                string(String::from("Paladin")),
                space(),
                x(),
                number(String::from("2")),
                eof(),
            ]
        );
    }
    #[test]
    fn lex_name_tab_quantity() {
        assert_eq!(
            lex("Paladin	2").unwrap(),
            vec![
                string(String::from("Paladin")),
                tab(),
                number(String::from("2")),
                eof(),
            ]
        );
    }
    #[test]
    fn lex_name_star_tab_quantity() {
        assert_eq!(
            lex("Paladin*	2").unwrap(),
            vec![
                string(String::from("Paladin*")),
                tab(),
                number(String::from("2")),
                eof(),
            ]
        );
    }
    #[test]
    fn lex_quantity_space_name() {
        assert_eq!(
            lex("2 Paladin").unwrap(),
            vec![
                number(String::from("2")),
                space(),
                string(String::from("Paladin")),
                eof(),
            ]
        );
    }
    #[test]
    fn lex_quantity_space_x_space_name() {
        assert_eq!(
            lex("2 x Paladin").unwrap(),
            vec![
                number(String::from("2")),
                space(),
                x(),
                space(),
                string(String::from("Paladin")),
                eof(),
            ]
        );
    }
    #[test]
    fn lex_quantity_x_space_name() {
        assert_eq!(
            lex("2x Paladin").unwrap(),
            vec![
                number(String::from("2")),
                x(),
                space(),
                string(String::from("Paladin")),
                eof(),
            ]
        );
    }
    #[test]
    fn lex_ship_fit_name() {
        assert_eq!(
            lex("[Paladin, Joe's Paladin]").unwrap(),
            vec![
                squarebracketleft(),
                string(String::from("Paladin")),
                comma(),
                space(),
                string(String::from("Joe's")),
                space(),
                string(String::from("Paladin")),
                squarebracketright(),
                eof(),
            ]
        );
    }
    #[test]
    fn lex_view_contents() {
        assert_eq!(
            lex("Burned Logic Circuit	Salvaged Materials	Cargo Hold	26").unwrap(),
            vec![
                string(String::from("Burned")),
                space(),
                string(String::from("Logic")),
                space(),
                string(String::from("Circuit")),
                tab(),
                string(String::from("Salvaged")),
                space(),
                string(String::from("Materials")),
                tab(),
                string(String::from("Cargo")),
                space(),
                string(String::from("Hold")),
                tab(),
                number(String::from("26")),
                eof(),
            ]
        );
    }
    #[test]
    fn lex_contract() {
        assert_eq!(
            lex("Capital Transverse Bulkhead I	1	Rig Armor	Module	Rig Slot").unwrap(),
            vec![
                string(String::from("Capital")),
                space(),
                string(String::from("Transverse")),
                space(),
                string(String::from("Bulkhead")),
                space(),
                string(String::from("I")),
                tab(),
                number(String::from("1")),
                tab(),
                string(String::from("Rig")),
                space(),
                string(String::from("Armor")),
                tab(),
                string(String::from("Module")),
                tab(),
                string(String::from("Rig")),
                space(),
                string(String::from("Slot")),
                eof(),
            ]
        );
    }
    #[test]
    fn lex_contract_no_details() {
        assert_eq!(
            lex("Cybernetic Subprocessor - Basic	1	Cyber Learning	Implant	").unwrap(),
            vec![
                string(String::from("Cybernetic")),
                space(),
                string(String::from("Subprocessor")),
                space(),
                string(String::from("-")),
                space(),
                string(String::from("Basic")),
                tab(),
                number(String::from("1")),
                tab(),
                string(String::from("Cyber")),
                space(),
                string(String::from("Learning")),
                tab(),
                string(String::from("Implant")),
                tab(),
                eof(),
            ]
        );
    }
}

// Lexer/scanner borrowed from dicelang which is heavily inspired by Crafting Interpreters
#[derive(Debug, PartialEq, Clone)]
enum TokenKind {
    X,                  // literally "x"
    SquareBracketLeft,  // [
    SquareBracketRight, // ]
    Tab,
    // While distinct components are usually tab-separated, this is not always the case
    Space,
    Comma,

    // TODO: Use type name/specific instead of string?
    String,
    // TypeName,           // Unspecific name of an item, e.g. "Paladin"
    // SpecificItemName, // If a named instance of an item, e.g. "Michael's Paladin"
    Number,
    EOF,
}

#[derive(Debug, PartialEq, Clone)]
struct Token {
    kind: TokenKind,
    s: String,
}

fn x() -> Token {
    Token {
        kind: TokenKind::X,
        s: String::new(),
    }
}
fn squarebracketright() -> Token {
    Token {
        kind: TokenKind::SquareBracketRight,
        s: String::new(),
    }
}
fn squarebracketleft() -> Token {
    Token {
        kind: TokenKind::SquareBracketLeft,
        s: String::new(),
    }
}
fn tab() -> Token {
    Token {
        kind: TokenKind::Tab,
        s: String::new(),
    }
}
fn space() -> Token {
    Token {
        kind: TokenKind::Space,
        s: String::new(),
    }
}
fn comma() -> Token {
    Token {
        kind: TokenKind::Comma,
        s: String::new(),
    }
}
fn string(s: String) -> Token {
    Token {
        kind: TokenKind::String,
        s: s,
    }
}
fn number(s: String) -> Token {
    Token {
        kind: TokenKind::Number,
        s: s,
    }
}
fn eof() -> Token {
    Token {
        kind: TokenKind::EOF,
        s: String::new(),
    }
}

pub type LexErr = String;
struct Scanner {
    // TODO: Rewrite scanning as just using .chars() as an iterator?
    source: String,
    tokens: Vec<Token>,
    errors: Vec<LexErr>,

    lexeme_start: usize,
    next: usize,
}

fn is_digit(c: &char) -> bool {
    ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'].contains(c)
}
// TODO: UTF-16 necessary because of eve?
fn is_namechar(c: &char) -> bool {
    match c {
        // hyphen because of e.g. implant - basic
        // asterisk because of "Paladin" vs. "Paladin*"
        // apostrophe because of e.g. "Joe's Paladin"
        'A'..='Z' | 'a'..='z' | '-' | '*' | '\'' => true,
        _ => false,
    }
}

impl Scanner {
    fn scan_tokens(&mut self) {
        while !self.at_end() {
            self.lexeme_start = self.next;
            self.scan_token();
        }
        self.tokens.push(eof());
    }
    fn at_end(&self) -> bool {
        let len = self.source.chars().fold(0, |total, _| total + 1);
        return self.next >= len;
    }
    fn advance(&mut self) -> char {
        // The assumption is that advance will never be called if at end, so
        // unwrap() is okay
        let c = self.source.chars().nth(self.next).unwrap();
        self.next += 1;
        return c;
    }
    fn match_char(&mut self, expected: char) -> bool {
        if self.at_end() {
            return false;
        }
        // We've checked at end, so unwrap should be safe
        if self.source.chars().nth(self.next).unwrap() != expected {
            return false;
        }
        self.next += 1;
        return true;
    }

    // TODO this does an unncessary amount of copying
    fn add_token(&mut self, kind: TokenKind) {
        let source = self.source.clone();
        let mut token_str = String::from("");
        source
            .chars()
            .skip(self.lexeme_start)
            .take(self.next - self.lexeme_start)
            .for_each(|c| token_str.push(c));
        match kind {
            TokenKind::X => self.tokens.push(x()),
            TokenKind::SquareBracketRight => self.tokens.push(squarebracketright()),
            TokenKind::SquareBracketLeft => self.tokens.push(squarebracketleft()),
            TokenKind::Tab => self.tokens.push(tab()),
            TokenKind::Space => self.tokens.push(space()),
            TokenKind::Comma => self.tokens.push(comma()),
            TokenKind::String => self.tokens.push(string(token_str)),
            TokenKind::Number => self.tokens.push(number(token_str)),
            TokenKind::EOF => self.tokens.push(eof()),
        }
    }
    fn peek(&self) -> char {
        if self.at_end() {
            return '_';
        }
        // We've checked at end, so unwrap should be safe
        return self.source.chars().nth(self.next).unwrap();
    }
    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            'x' => self.add_token(TokenKind::X),
            '[' => self.add_token(TokenKind::SquareBracketLeft),
            ']' => self.add_token(TokenKind::SquareBracketRight),
            '\t' => self.add_token(TokenKind::Tab),
            ' ' => self.add_token(TokenKind::Space),
            ',' => self.add_token(TokenKind::Comma),
            '\n' => self
                .errors
                .push(format!("Unsupported newline at pos {}", self.lexeme_start)),
            _ => {
                if is_digit(&c) {
                    self.number()
                } else if is_namechar(&c) {
                    self.string()
                } else {
                    self.errors.push(format!(
                        "Unsupported character at pos {}: {}",
                        self.lexeme_start, c
                    ))
                }
            }
        }
    }
    fn number(&mut self) {
        while is_digit(&self.peek()) {
            self.advance();
        }
        self.add_token(TokenKind::Number);
    }
    fn string(&mut self) {
        while is_namechar(&self.peek()) {
            self.advance();
        }
        self.add_token(TokenKind::String)
    }
}

fn lex(s: &str) -> Result<Vec<Token>, Vec<LexErr>> {
    let mut scan = Scanner {
        source: s.to_string(),

        tokens: Vec::new(),
        errors: Vec::new(),

        lexeme_start: 0,
        next: 0,
    };

    scan.scan_tokens();
    if scan.errors.len() > 0 {
        return Err(scan.errors);
    }
    return Ok(scan.tokens);
}
