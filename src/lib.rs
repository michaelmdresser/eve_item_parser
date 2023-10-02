use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests {
    // Thanks to https://github.com/harrelchris/eveparse/blob/main/tests/test_parse.py for many of these test cases.

    use super::*;

    #[test]
    fn intermediate_empty() {
        assert_eq!(
            parse(
                "Paladin

Harpy 1


Golem x3
"
            )
            .unwrap(),
            vec!(
                Item {
                    type_name: String::from("Paladin"),
                    quantity: 1,
                },
                Item {
                    type_name: String::from("Harpy"),
                    quantity: 1,
                },
                Item {
                    type_name: String::from("Golem"),
                    quantity: 3,
                },
            )
        );
    }

    #[test]
    fn mixed_character_module() {
        assert_eq!(
            parse("5MN Y-T8 Compact Microwarpdrive").unwrap(),
            vec!(Item {
                type_name: String::from("5MN Y-T8 Compact Microwarpdrive"),
                quantity: 1,
            },)
        );
    }

    #[test]
    fn bracketed_name_with_subbrackets() {
        assert_eq!(
            parse("[Paladin, [ABC] Pala]").unwrap(),
            vec!(Item {
                type_name: String::from("Paladin"),
                quantity: 1,
            },)
        );
    }

    #[test]
    fn name_only() {
        assert_eq!(
            lex("Paladin").unwrap(),
            vec![string(String::from("Paladin")), eof(),]
        );
        assert_eq!(
            parse("Paladin").unwrap(),
            vec![Item {
                type_name: String::from("Paladin"),
                quantity: 1
            }]
        );
        assert_eq!(
            parse_with_id("Paladin").unwrap(),
            vec![ItemWithId {
                type_name: String::from("Paladin"),
                type_id: 28659,
                quantity: 1
            }]
        );
    }
    #[test]
    fn name_space_quantity() {
        assert_eq!(
            lex("Paladin 2").unwrap(),
            vec![
                string(String::from("Paladin")),
                space(),
                number(String::from("2")),
                eof(),
            ]
        );
        assert_eq!(
            parse("Paladin 2").unwrap(),
            vec![Item {
                type_name: String::from("Paladin"),
                quantity: 2
            }]
        );
    }
    #[test]
    fn name_star_space_quantity() {
        assert_eq!(
            lex("Paladin* 2").unwrap(),
            vec![
                string(String::from("Paladin*")),
                space(),
                number(String::from("2")),
                eof(),
            ]
        );
        assert_eq!(
            parse("Paladin* 2").unwrap(),
            vec![Item {
                type_name: String::from("Paladin"),
                quantity: 2
            }]
        );
    }
    #[test]
    fn name_space_x_quantity() {
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
        assert_eq!(
            parse("Paladin x2").unwrap(),
            vec![Item {
                type_name: String::from("Paladin"),
                quantity: 2
            }]
        );
    }
    #[test]
    fn name_tab_quantity() {
        assert_eq!(
            lex("Paladin	2").unwrap(),
            vec![
                string(String::from("Paladin")),
                tab(),
                number(String::from("2")),
                eof(),
            ]
        );
        assert_eq!(
            parse("Paladin	2").unwrap(),
            vec![Item {
                type_name: String::from("Paladin"),
                quantity: 2
            }]
        );
    }
    #[test]
    fn name_star_tab_quantity() {
        assert_eq!(
            lex("Paladin*	2").unwrap(),
            vec![
                string(String::from("Paladin*")),
                tab(),
                number(String::from("2")),
                eof(),
            ]
        );
        assert_eq!(
            parse("Paladin*	2").unwrap(),
            vec![Item {
                type_name: String::from("Paladin"),
                quantity: 2
            }]
        );
    }

    // I currently don't believe that these number-before-name cases are valid
    // tests. Will reintroduce if that changes.
    // #[test]
    // fn quantity_space_name() {
    //     assert_eq!(
    //         lex("2 Paladin").unwrap(),
    //         vec![
    //             number(String::from("2")),
    //             space(),
    //             string(String::from("Paladin")),
    //             eof(),
    //         ]
    //     );
    //     assert_eq!(
    //         parse("2 Paladin").unwrap(),
    //         vec![Item {
    //             type_name: String::from("Paladin"),
    //             quantity: 2
    //         }]
    //     );
    // }
    // #[test]
    // fn quantity_space_x_space_name() {
    //     assert_eq!(
    //         lex("2 x Paladin").unwrap(),
    //         vec![
    //             number(String::from("2")),
    //             space(),
    //             x(),
    //             space(),
    //             string(String::from("Paladin")),
    //             eof(),
    //         ]
    //     );
    //     assert_eq!(
    //         parse("2 x Paladin").unwrap(),
    //         vec![Item {
    //             type_name: String::from("Paladin"),
    //             quantity: 2
    //         }]
    //     );
    // }
    // #[test]
    // fn quantity_x_space_name() {
    //     assert_eq!(
    //         lex("2x Paladin").unwrap(),
    //         vec![
    //             number(String::from("2")),
    //             x(),
    //             space(),
    //             string(String::from("Paladin")),
    //             eof(),
    //         ]
    //     );
    //     assert_eq!(
    //         parse("2x Paladin").unwrap(),
    //         vec![Item {
    //             type_name: String::from("Paladin"),
    //             quantity: 2
    //         }]
    //     );
    // }
    #[test]
    fn ship_fit_name() {
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
        assert_eq!(
            parse("[Paladin, Joe's Paladin]").unwrap(),
            vec![Item {
                type_name: String::from("Paladin"),
                quantity: 1
            }]
        );
    }
    #[test]
    fn view_contents() {
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
        assert_eq!(
            parse("Burned Logic Circuit	Salvaged Materials	Cargo Hold	26").unwrap(),
            vec![Item {
                type_name: String::from("Burned Logic Circuit"),
                quantity: 26,
            }]
        );
    }
    #[test]
    fn contract() {
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
        assert_eq!(
            parse("Capital Transverse Bulkhead I	2	Rig Armor	Module	Rig Slot").unwrap(),
            vec![Item {
                type_name: String::from("Capital Transverse Bulkhead I"),
                quantity: 2,
            }]
        );
    }
    #[test]
    fn contract_no_details() {
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
        assert_eq!(
            parse("Cybernetic Subprocessor - Basic	2	Cyber Learning	Implant	").unwrap(),
            vec![Item {
                type_name: String::from("Cybernetic Subprocessor - Basic"),
                quantity: 2,
            }]
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

////////
////////
////////
////////
////////
////////

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Item {
    pub type_name: String,
    pub quantity: i64,
}
impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.type_name, self.quantity)
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ItemWithId {
    pub type_name: String,
    pub type_id: u64,
    pub quantity: i64,
}
impl std::fmt::Display for ItemWithId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "({}, {}, {})",
            self.type_id, self.type_name, self.quantity
        )
    }
}

// Similarly, the parser is initially taken from dicelang, which is itself
// heavily inspired by Crafting Intepreters
struct Parser {
    tokens: Vec<Token>,
    current: usize,
}
impl Parser {
    fn at_end(&self) -> bool {
        match self.peek().kind {
            TokenKind::EOF => true,
            _ => false,
        }
    }
    fn advance(&mut self) -> Token {
        if !self.at_end() {
            self.current += 1
        }
        return self.previous();
    }
    fn previous(&self) -> Token {
        return self.tokens[self.current - 1].clone();
    }
    fn match_kind(&mut self, kind: TokenKind) -> bool {
        if self.check(kind) {
            self.advance();
            return true;
        }
        return false;
    }
    fn check(&self, kind: TokenKind) -> bool {
        if self.at_end() {
            return false;
        }
        let next = self.peek();
        return kind == next.kind;
    }
    fn peek(&self) -> Token {
        return self.tokens[self.current].clone();
    }
    fn consume(&mut self, kind: TokenKind, message: &str) -> Result<Token, String> {
        if self.check(kind) {
            return Ok(self.advance());
        }
        return Err(message.to_string());
    }

    /////////
    /////////
    /////////

    // TODO: synchronize
    fn item(&mut self) -> Result<Item, String> {
        if self.check(TokenKind::SquareBracketLeft) {
            self.consume(
                TokenKind::SquareBracketLeft,
                "checking a left bracket must consume a left bracket",
            )?;
            let full_name = match self.full_name() {
                Ok(s) => s,
                Err(e) => {
                    return Err(format!(
                        "left bracket must be followed by a name, err: {}",
                        e
                    ))
                }
            };
            self.consume(
                TokenKind::Comma,
                "bracketed name must be followed by a comma",
            )?;
            if self.check(TokenKind::Space) {
                self.consume(TokenKind::Space, "checking a space must consume a space")?;
            }

            // The actual individual item (ship, usually) name can include sub-brackets,
            // which I don't want to bother supporting parsing of at the moment. Once we've
            // parsed the comma we know we have the item and no more information is required.
            // match self.full_name() {
            //     Err(e) => {
            //         return Err(format!(
            //             "bracketed name must have a second name after comma, err: {}",
            //             e,
            //         ))
            //     }
            //     _ => (),
            // };
            // self.consume(
            //     TokenKind::SquareBracketRight,
            //     "bracketed names must be terminated by a right bracket",
            // )?;

            return Ok(Item {
                type_name: full_name,
                quantity: 1,
            });
        }
        if self.check(TokenKind::String) || self.check(TokenKind::Number) {
            let full_name = match self.full_name() {
                Ok(s) => s,
                Err(e) => {
                    return Err(format!(
                        "starting with a string or number demands a full name match, err: {}",
                        e
                    ))
                }
            };
            if self.at_end() {
                return Ok(Item {
                    type_name: full_name,
                    quantity: 1,
                });
            }
            if self.check(TokenKind::Space) {
                self.consume(TokenKind::Space, "checking space must consume space")?;
            } else if self.check(TokenKind::Tab) {
                self.consume(TokenKind::Tab, "checking tab must consume tab")?;
                if self.check(TokenKind::Number) {
                    let qty = match self.quantity() {
                        Ok(q) => q,
                        Err(e) => return Err(format!("full name followed by tab then number must match a quantity for the number, err: {}", e)),
                    };
                    return Ok(Item {
                        type_name: full_name,
                        quantity: qty,
                    });
                }

                self.full_name()?;
                self.consume(TokenKind::Tab, "contents view is expected to be name-tab-name-tab-tab-quantity, second tab is missing")?;
                self.full_name()?;
                self.consume(TokenKind::Tab, "contents view is expected to be name-tab-name-tab-tab-quantity, third tab is missing")?;
            }
            let qty = match self.quantity() {
                Ok(q) => q,
                Err(e) => {
                    return Err(format!(
                        "starting with a full name not followed by EOF demands a quantity, err: {}",
                        e
                    ))
                }
            };
            return Ok(Item {
                type_name: full_name,
                quantity: qty,
            });
        }

        return Err(format!("invalid starting token: {:?}", self.peek()));
    }
    fn full_name(&mut self) -> Result<String, String> {
        let mut full_string: String = "".to_owned();
        loop {
            if self.check(TokenKind::Number) {
                if self.current == 0 {
                    let tok =
                        self.consume(TokenKind::Number, "checking number must consume number")?;
                    full_string.push_str(&format!("{}", tok.s));
                } else {
                    let prev = self.previous();
                    match prev.kind {
                        TokenKind::String | TokenKind::Number => {
                            let tok = self.consume(
                                TokenKind::Number,
                                "checking number must consume number",
                            )?;
                            full_string.push_str(&format!("{}", tok.s));
                        }
                        // Numbers preceded by a space are assumed to be quantities.
                        // I'm not certain if this holds up -- if there is a module
                        // which has a word after the first which starts with a
                        // number then we're in trouble and have to do more
                        // lookahead than I want to to.
                        TokenKind::Space => break,
                        _ => break,
                    };
                }
            } else if self.check(TokenKind::String) {
                let tok =
                    self.consume(TokenKind::String, "checking a string must consume a string")?;
                full_string.push_str(&tok.s);
            } else if self.check(TokenKind::Space) {
                self.consume(TokenKind::Space, "checking a space must consume a space")?;
                full_string.push_str(" ");
            } else {
                break;
            }
        }
        let trimmed = full_string.trim();
        let cleaned: String = match trimmed.strip_suffix("*") {
            Some(s) => s.to_string(),
            None => trimmed.to_string(),
        };
        if cleaned.is_empty() {
            return Err("empty string after cleaning asterisks and trimming".to_string());
        }
        return Ok(cleaned.to_string());
    }
    fn quantity(&mut self) -> Result<i64, String> {
        if self.check(TokenKind::X) {
            self.consume(TokenKind::X, "checking x must consume x")?;
            if self.check(TokenKind::Space) {
                self.consume(TokenKind::Space, "checking space must consume space")?;
            }
            let tok = self.consume(
                TokenKind::Number,
                "quantities must have a number after x (optional space in between)",
            )?;
            let q: i64 = match tok.s.parse() {
                Ok(u) => u,
                Err(e) => return Err(format!("parsing {} to i64: {}", tok.s, e)),
            };
            return Ok(q);
        } else {
            let tok = self.consume(TokenKind::Number, "quantities must be a number")?;
            if self.check(TokenKind::Space) {
                self.consume(TokenKind::Space, "checking space must consume space")?;
            }
            if self.check(TokenKind::X) {
                self.consume(TokenKind::X, "checking x must consume x")?;
            }
            let q: i64 = match tok.s.parse() {
                Ok(u) => u,
                Err(e) => return Err(format!("parsing {} to i64: {}", tok.s, e)),
            };
            return Ok(q);
        }
    }
}

pub fn parse(s: &str) -> Result<Vec<Item>, String> {
    let items: Result<Vec<Item>, String> = s
        .lines()
        .map(|line| line.trim())
        .filter(|line| line.len() > 0)
        .enumerate()
        .map(|(i, line)| {
            let tokens = match lex(line.trim()) {
                Ok(tokens) => tokens,
                Err(errs) => {
                    let s = errs.iter().fold(String::new(), |acc, e| acc + e);
                    return Err(format!("line {}: {s}", i));
                }
            };
            let mut p = Parser {
                tokens: tokens,
                current: 0,
            };
            let item = p.item()?;
            return Ok(item);
        })
        .collect();

    return items;
}

include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

pub fn parse_with_id(s: &str) -> Result<Vec<ItemWithId>, String> {
    let items = parse(s)?;
    items
        .iter()
        .map(|item| {
            let id = ITEM_TO_CODE
                .get(&item.type_name)
                .ok_or(format!("failed to look up {}", item.type_name))?;
            return Ok(ItemWithId {
                type_name: item.type_name.clone(),
                quantity: item.quantity,
                type_id: *id,
            });
        })
        .collect()
}

pub fn lookup_id(id: u64) -> Option<String> {
    match CODE_TO_ITEM.get(&id).copied() {
        Some(s) => Some(s.to_string()),
        None => None,
    }
}

pub fn lookup_type_name(type_name: String) -> Option<u64> {
    ITEM_TO_CODE.get(&type_name).copied()
}

pub fn format_tabular(items: Vec<ItemWithId>) -> String {
    items
        .iter()
        .map(|item| format!("{}\t{}\n", item.type_name, item.quantity))
        .fold("".to_string(), |cur, next| cur + &next)
}
