use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests {
    // Thanks to https://github.com/harrelchris/eveparse/blob/main/tests/test_parse.py for many of these test cases.

    use super::*;

    #[test]
    fn comma_number() {
        assert_eq!(
            parse(" Republic Fleet EMP S	3,200	Projectile Ammo	Charge	Cargo Hold").unwrap(),
            vec!(Item {
                type_name: String::from("Republic Fleet EMP S"),
                quantity: 3200,
            })
        );
    }

    #[test]
    fn comma_number_long() {
        assert_eq!(
            parse(" Republic Fleet EMP S	3,200,189	Projectile Ammo	Charge	Cargo Hold").unwrap(),
            vec!(Item {
                type_name: String::from("Republic Fleet EMP S"),
                quantity: 3200189,
            })
        );
    }

    #[test]
    fn intermediate_empty() {
        assert_eq!(
            parse(
                "Paladin

Harpy x1


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
    fn cap_booster() {
        assert_eq!(
            parse("Navy Cap Booster 3200 x9").unwrap(),
            vec!(Item {
                type_name: String::from("Navy Cap Booster 3200"),
                quantity: 9,
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
    fn empty_med_slot() {
        assert_eq!(parse("[Empty Med slot]").unwrap(), vec!());
    }
    #[test]
    fn empty_low_slot() {
        assert_eq!(parse("[Empty Low slot]").unwrap(), vec!());
    }
    #[test]
    fn empty_high_slot() {
        assert_eq!(parse("[Empty High slot]").unwrap(), vec!());
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

    // It turns out that they are valid input. Multibuy, for example,
    // accepts this format -- it is the format output by jEveAssets
    // when selecting "Copy+ -> EVE MultiBuy". To be able to diff things
    // from jEveAssets I believe these situations will need to be supported.
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

    #[test]
    fn module_with_charge() {
        assert_eq!(
            lex("Shield Command Burst II, Shield Harmonizing Charge").unwrap(),
            vec![
                string(String::from("Shield")),
                space(),
                string(String::from("Command")),
                space(),
                string(String::from("Burst")),
                space(),
                string(String::from("II")),
                comma(),
                space(),
                string(String::from("Shield")),
                space(),
                string(String::from("Harmonizing")),
                space(),
                string(String::from("Charge")),
                eof(),
            ]
        );
        assert_eq!(
            parse("Shield Command Burst II, Shield Harmonizing Charge").unwrap(),
            vec![
                Item {
                    type_name: String::from("Shield Command Burst II"),
                    quantity: 1,
                },
                Item {
                    type_name: String::from("Shield Harmonizing Charge"),
                    // TODO: Should this be the max number that can fit in
                    // this module?
                    quantity: 1,
                }
            ]
        );

        // TODO: add this as a test? also this
        // Capital Capacitor Booster II, Navy Cap Booster 3200
        // Shield Command Burst II, Shield Harmonizing Charge
        // Shield Command Burst II, Active Shielding Charge
    }

    #[test]
    fn module_with_quotes() {
        assert_eq!(
            lex("'Vehemence' Compact Large EMP Smartbomb x4").unwrap(),
            vec![
                string(String::from("'Vehemence'")),
                space(),
                string(String::from("Compact")),
                space(),
                string(String::from("Large")),
                space(),
                string(String::from("EMP")),
                space(),
                string(String::from("Smartbomb")),
                space(),
                x(),
                number(String::from("4")),
                eof(),
            ]
        );

        assert_eq!(
            parse_with_id("'Vehemence' Compact Large EMP Smartbomb x4").unwrap(),
            vec![ItemWithId {
                type_name: String::from("'Vehemence' Compact Large EMP Smartbomb"),
                type_id: 9678,
                quantity: 4,
            }]
        );
    }

    #[test]
    fn defunct_offline_syntax() {
        assert_eq!(
            lex("Armor Command Burst II /OFFLINE").unwrap(),
            vec![
                string(String::from("Armor")),
                space(),
                string(String::from("Command")),
                space(),
                string(String::from("Burst")),
                space(),
                string(String::from("II")),
                space(),
                string(String::from("/OFFLINE")),
                eof(),
            ]
        );
        assert_eq!(
            parse_with_id("Armor Command Burst II /OFFLINE").unwrap(),
            vec![ItemWithId {
                type_name: String::from("Armor Command Burst II"),
                type_id: 43552,
                quantity: 1,
            }]
        )
    }

    #[test]
    fn module_with_period() {
        assert_eq!(
            lex("Eifyr and Co. 'Rogue' Navigation NN-602 x1").unwrap(),
            vec![
                string(String::from("Eifyr")),
                space(),
                string(String::from("and")),
                space(),
                string(String::from("Co.")),
                space(),
                string(String::from("'Rogue'")),
                space(),
                string(String::from("Navigation")),
                space(),
                string(String::from("NN-602")),
                space(),
                x(),
                number(String::from("1")),
                eof(),
            ]
        );

        assert_eq!(
            parse_with_id("'Vehemence' Compact Large EMP Smartbomb x4").unwrap(),
            vec![ItemWithId {
                type_name: String::from("'Vehemence' Compact Large EMP Smartbomb"),
                type_id: 9678,
                quantity: 4,
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

#[derive(PartialEq, Clone)]
struct Token {
    kind: TokenKind,
    s: String,
}

impl std::fmt::Debug for Token {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.kind {
            TokenKind::X => fmt.write_str(" X ")?,
            TokenKind::SquareBracketLeft => fmt.write_str(" [ ")?,
            TokenKind::SquareBracketRight => fmt.write_str(" ] ")?,
            TokenKind::Tab => fmt.write_str(" TAB ")?,
            TokenKind::Space => fmt.write_str(" SPC ")?,
            TokenKind::Comma => fmt.write_str(" CMA ")?,
            TokenKind::String => fmt.write_str(&format!(" '{}' ", self.s))?,
            TokenKind::Number => fmt.write_str(&format!(" n{} ", self.s))?,
            TokenKind::EOF => fmt.write_str(" EOF")?,
        }
        Ok(())
    }
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
    // Digits can be in the middle of names. I don't think digits can start
    // names. We mostly handle this in the parser by checking is_digit to do a
    // number parse before we try to do a name parse.
    if is_digit(c) {
        return true;
    }

    match c {

        'A'..='Z'
            | 'a'..='z'
            | '-' // hyphen because of e.g. implant - basic
            | '*' // asterisk because of "Paladin" vs. "Paladin*"
            | '\'' // apostrophe because of e.g. "Joe's Paladin"
            | '/' // forward slash because of weird /OFFLINE syntax
            | '.' // period because of e.g. "Eifyr and Co."
            => true,
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
    // Check the second. This is lookahead and makes me sad but is required for
    // a multibuy format, I think, unless I'm missing something quite clever.
    fn check2(&self, kind: TokenKind) -> bool {
        if self.at_end() {
            return false;
        }
        let next2 = self.peek2();
        return kind == next2.kind;
    }
    fn peek(&self) -> Token {
        return self.tokens[self.current].clone();
    }
    fn peek2(&self) -> Token {
        return self.tokens[self.current + 1].clone();
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
    fn item(&mut self) -> Result<Option<Vec<Item>>, String> {
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
            if full_name == "Empty High slot"
                || full_name == "Empty Med slot"
                || full_name == "Empty Low slot"
            {
                return Ok(None);
            }
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

            return Ok(Some(vec![Item {
                type_name: full_name,
                quantity: 1,
            }]));
        }
        if self.check(TokenKind::String) || self.check(TokenKind::Number) {
            let full_name = match self.full_name() {
                Ok(s) => s,
                Err(e) => {
                    return Err(format!(
                        "starting with a string or number demands a full name match, tokens: {:?}, err: {}",
                        self.tokens,
                        e
                    ))
                }
            };
            if self.at_end() {
                return Ok(Some(vec![Item {
                    type_name: full_name,
                    quantity: 1,
                }]));
            }

            // If we consume a name, then get a comma, the next thing should be a charge
            // loaded into that module
            if self.check(TokenKind::Comma) {
                let first_item = Item {
                    type_name: full_name,
                    quantity: 1,
                };

                self.consume(TokenKind::Comma, "checking comma must consume comma")?;
                self.consume(
                    TokenKind::Space,
                    "a loaded charge should be preceded by a comma followed by a space",
                )?;

                match self.item() {
                    Ok(Some(items)) => {
                        if items.len() != 1 {
                            return Err(format!(
                                "Unexpected number of loaded charge item types {:?}",
                                items
                            ));
                        }

                        if !self.at_end() {
                            return Err(format!(
                                "Unexpectedly at the end for a module with loaded charges {:?}",
                                self.tokens
                            ));
                        }

                        return Ok(Some(vec![first_item, items[0].clone()]));
                    }
                    Ok(None) => return Ok(Some(vec![first_item])),
                    Err(e) => {
                        return Err(format!("Failed to match sub-item for loaded charge: {e}"));
                    }
                }
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
                    return Ok(Some(vec![Item {
                        type_name: full_name,
                        quantity: qty,
                    }]));
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
                        "starting with a full name not followed by EOF demands a quantity, tokens: {:?} err: {}",
                        self.tokens,
                        e
                    ))
                }
            };
            return Ok(Some(vec![Item {
                type_name: full_name,
                quantity: qty,
            }]));
        }

        return Err(format!("invalid starting token: {:?}", self.peek()));
    }
    fn full_name(&mut self) -> Result<String, String> {
        let mut full_string: String = "".to_owned();
        loop {
            if self.check(TokenKind::Number) {
                // A number followed by EOF indicates a quantity. E.g. "Paladin 2" is 2 Paladins
                if self.check2(TokenKind::EOF) {
                    break;
                }

                let tok = self.consume(TokenKind::Number, "checking number must consume number")?;
                full_string.push_str(&format!("{}", tok.s));
            } else if self.check(TokenKind::String) {
                let tok =
                    self.consume(TokenKind::String, "checking a string must consume a string")?;

                // Some fits have something like this "Armor Command Burst II /OFFLINE" where the
                // /OFFLINE means nothing for the sake of the item diff.
                if tok.s == "/OFFLINE" {
                    continue;
                }
                full_string.push_str(&tok.s);
            } else if self.check(TokenKind::Space) {
                self.consume(TokenKind::Space, "checking a space must consume a space")?;
                if self.at_end() {
                    break;
                }

                // TODO: I really don't like this. The idea is that a " x" implies
                // this is about to be a quantity, so we're done parsing the name.
                let after_space = self.peek();
                if after_space.kind == TokenKind::String && after_space.s == "x" {
                    break;
                }

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
            let mut num_sections: Vec<i64> = Vec::new();
            let tok = self.consume(TokenKind::Number, "quantities must be a number")?;
            let q: i64 = match tok.s.parse() {
                Ok(u) => u,
                Err(e) => return Err(format!("parsing {} to i64: {}", tok.s, e)),
            };
            num_sections.push(q);

            while self.check(TokenKind::Comma) {
                self.consume(TokenKind::Comma, "checking comma must consume comma")?;
                let tok = self.consume(
                    TokenKind::Number,
                    "numbers followed by comma must also be followed by numbers",
                )?;
                let q: i64 = match tok.s.parse() {
                    Ok(u) => u,
                    Err(e) => return Err(format!("parsing {} to i64: {}", tok.s, e)),
                };
                num_sections.push(q);
            }

            // Each num_section is comma-separated, meaning it has to be multiplied
            // by 10^x, where x is a multiple of 3 (increasing with the number of
            // sections).
            let result: i64 = num_sections
                .iter()
                .enumerate()
                .map(|(place, num_section)| {
                    let base: u32 = 10;
                    let multiple = base.pow((num_sections.len() as u32 - (place + 1) as u32) * 3);
                    println!(
                        "num_section: {}, place: {}, multiple: {}",
                        num_section, place, multiple
                    );
                    num_section * (multiple as i64)
                })
                .fold(0, |a, b| a + b);

            if self.check(TokenKind::Space) {
                self.consume(TokenKind::Space, "checking space must consume space")?;
            }
            if self.check(TokenKind::X) {
                self.consume(TokenKind::X, "checking x must consume x")?;
            }
            return Ok(result);
        }
    }
}

pub fn parse(s: &str) -> Result<Vec<Item>, String> {
    let results: Vec<Result<Vec<Item>, String>> = s
        .lines()
        .map(|line| line.trim())
        .filter(|line| line.len() > 0)
        .enumerate()
        .filter_map(|(i, line)| {
            let tokens = match lex(line.trim()) {
                Ok(tokens) => tokens,
                Err(errs) => {
                    let s = errs.iter().fold(String::new(), |acc, e| acc + e);
                    return Some(Err(format!("line {i}: {s}")));
                }
            };
            let mut p = Parser { tokens, current: 0 };
            match p.item() {
                Ok(Some(item)) => Some(Ok(item)),
                Ok(None) => None,
                Err(e) => Some(Err(e)),
            }
        })
        .collect();

    let mut items = Vec::new();
    let mut errors = Vec::new();
    for result in results {
        match result {
            Ok(is) => items.extend(is),
            Err(e) => errors.push(e),
        }
    }

    if errors.len() > 0 {
        return Err(format!("{:?}", errors));
    }

    return Ok(items);
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

pub fn format_x(items: Vec<ItemWithId>) -> String {
    items
        .iter()
        .map(|item| format!("{} x{}\n", item.type_name, item.quantity))
        .fold("".to_string(), |cur, next| cur + &next)
}
