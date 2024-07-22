use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use crate::vhash::ValueHashMap;

#[derive(Debug,Clone,PartialEq)]
pub enum Token {
    // keywords
    And,    Break,  Do,     Else,   Elseif, End,
    False,  For,    Function, Goto, If,     In,
    Local,  Nil,    Not,    Or,     Repeat, Return,
    Then,   True,   Until,  While,

 // +       -       *       /       %       ^       #
    Add,    Sub,    Mul,    Div,    Mod,    Pow,    Len,
 // &       ~       |       <<      >>      //
    BitAnd, BitXor, BitOr,  ShiftL, ShiftR, Idiv,
 // ==       ~=     <=      >=      <       >        =
    Equal,  NotEq,  LesEq,  GreEq,  Less,   Greater, Assign,
 // (       )       {       }       [       ]       ::
    ParL,   ParR,   CurlyL, CurlyR, SqurL,  SqurR,  DoubColon,
 // ;               :       ,       .       ..      ...
    SemiColon,      Colon,  Comma,  Dot,    Concat, Dots,

    // constant values
    Integer(i64),
    Float(f64),
    String(String),

    // name of variables or table keys
    Name(String),

    // // end
    // Eos,
}

#[derive(Debug)]
pub struct Lex {
    input: File,
    ahead: Option<Token>,
}

impl Lex {
    pub fn new(input: File) -> Self {
        Lex{input,ahead:None}
    }

    /// Consume the next token
    /// 
    /// If ahead is None, read the next token, if not, return the ahead 
    pub fn next(&mut self) -> Option<Token> {
        match self.ahead.take() {
            Some(token) => Some(token),
            None => self._next(),
        }
    }

    /// Peek the next token without consuming
    pub fn peek(&mut self) -> Option<&Token> {
        if self.ahead.is_none() {
            self.ahead = self._next();
        }
        self.ahead.as_ref()
    }

    pub fn _next(&mut self) -> Option<Token> {
        let ch = self.read_char();
        match ch {
            '\n' | '\r' | '\t' | ' ' => self._next(), // skip whitespaces
            '+' => Some(Token::Add), 
            '*' => Some(Token::Mul), 
            '%' => Some(Token::Mod),
            '^' => Some(Token::Pow),
            '#' => Some(Token::Len),
            '&' => Some(Token::BitAnd),
            '|' => Some(Token::BitOr),
            '(' => Some(Token::ParL),
            ')' => Some(Token::ParR),
            '{' => Some(Token::CurlyL),
            '}' => Some(Token::CurlyR),
            '[' => Some(Token::SqurL),
            ']' => Some(Token::SqurR),
            ';' => Some(Token::SemiColon),
            ',' => Some(Token::Comma),
            '/' => Some(self.check_ahead('/', Token::Idiv, Token::Div)),
            '=' => Some(self.check_ahead('=', Token::Equal, Token::Assign)),
            '~' => Some(self.check_ahead('=', Token::NotEq, Token::BitXor)),
            ':' => Some(self.check_ahead(':', Token::DoubColon, Token::Colon)),
            '<' => Some(self.check_ahead2('=', Token::LesEq, '<', Token::ShiftL, Token::Less)),
            '>' => Some(self.check_ahead2('=', Token::GreEq, '>', Token::ShiftR, Token::Greater)),
            '\'' | '"' => Some(self.read_string(ch)),
            '.' => match self.read_char() {
                '.' => {
                    if self.read_char() == '.' {
                        Some(Token::Dots) // ...
                    } else {
                        self.putback_char();
                        Some(Token::Concat) // ..
                    }
                },
                '0'..='9' => {
                    self.putback_char();
                    Some(self.read_number_fraction(0)) // .0-9
                },
                _ => {
                    self.putback_char();
                    Some(Token::Dot) // .
                },
            },
            '-' => {
                if self.read_char() == '-' {
                    self.read_comment();
                    self._next()
                } else {
                    self.putback_char();
                    Some(Token::Sub)
                }
            },
            '0'..='9' => Some(self.read_number(ch)),
            'A'..='Z' | 'a'..='z' | '_' => Some(self.read_name(ch)),
            '\0' => None,
            _ => panic!("invalid char {ch}"),
        }
    }

    #[allow(clippy::unused_io_amount)]
    fn read_char(&mut self) -> char {
        let mut buf: [u8; 1] = [0];
        self.input.read(&mut buf).unwrap();
        buf[0] as char
    }
    fn putback_char(&mut self) {
        self.input.seek(SeekFrom::Current(-1)).unwrap();
    }

    /// ## check_ahead 
    /// 
    /// Is used to infer the combined operator
    /// 
    /// ```ignore
    /// use luar::lex::Token;
    /// // Assume the frist char is '='
    /// check_head('=', Token::Equal,Token::Assign);
    /// // check_head will infer if the target operator is Assign op '=' or Equal op'=='
    /// ```
    fn check_ahead(&mut self, ahead: char, long: Token, short: Token) -> Token {
        if self.read_char() == ahead {
            long
        } else {
            self.putback_char(); // push back the readed char
            short
        }
    }
    /// Simliar to `check_ahead` but has 2 instead of 1 branches
    fn check_ahead2(&mut self, ahead1: char, long1: Token, ahead2: char, long2: Token, short: Token) -> Token {
        let ch = self.read_char();
        if ch == ahead1 {
            long1
        } else if ch == ahead2 {
            long2
        } else {
            self.putback_char();
            short
        }
    }

    fn read_number(&mut self, first: char) -> Token {
        // heximal
        if first == '0' { // For str start with '0x' regard as Hex
            let second = self.read_char();
            if second == 'x' || second == 'X' {
                return self.read_heximal();
            }
            self.putback_char();
        }

        // decimal
        let mut n = char::to_digit(first, 10).unwrap() as i64;
        loop {
            let ch = self.read_char();
            if let Some(d) = char::to_digit(ch, 10) {
                n = n * 10 + d as i64;
            } else if ch == '.' { // . div float
                return self.read_number_fraction(n);
            } else if ch == 'e' || ch == 'E' { // float number in scientific notation
                return self.read_number_exp(n as f64);
            } else {
                self.putback_char();
                break;
            }
        }

        // check following
        let fch = self.read_char();
        if fch.is_alphabetic() || fch == '.' {
            panic!("malformat number");
        } else {
            self.putback_char();
        }

        // Else as integer
        Token::Integer(n)
    }

    fn read_number_fraction(&mut self, i: i64) -> Token {
        let mut n: i64 = 0;
        let mut x: f64 = 1.0;
        loop {
            let ch = self.read_char();
            if let Some(d) = char::to_digit(ch, 10) {
                n = n * 10 + d as i64;
                x *= 10.0;
            } else {
                self.putback_char();
                break;
            }
        }
        Token::Float(i as f64 + n as f64 / x)
    }

    fn read_number_exp(&mut self, _: f64) -> Token {
        todo!("lex number exp")
    }
    
    fn read_heximal(&mut self) -> Token {
        todo!("lex heximal")
    }

    fn read_string(&mut self, quote: char) -> Token {
        let mut s = String::new();
        loop {
            match self.read_char() {
                '\n' | '\0' => panic!("unfinished string"),
                '\\' => todo!("escape"),
                ch if ch == quote => break,
                ch => s.push(ch),
            }
        }
        Token::String(s)
    }

    fn read_name(&mut self, first: char) -> Token {
        let mut s = first.to_string();

        loop {
            let ch = self.read_char();
            if ch.is_alphanumeric() || ch == '_' {
                s.push(ch);
            } else {
                self.putback_char();
                break;
            }
        }
        // use ValueHashMap witch return exact value instead of ref->value
        let mut km = get_keyword_map();
        match km.get_value(&s) {
            Some(token) => token.clone(),
            None => Token::Name(s.to_string()),
        }
    }

    // '--' has been read
    fn read_comment(&mut self) {
        match self.read_char() {
            '[' => todo!("long comment"), // --[ ] pattern
            _ => { // line comment
                loop {
                    let ch = self.read_char();
                    if ch == '\n' || ch == '\0' {
                        break;
                    }
                }
            }
        }
    }    
}

fn get_keyword_map() -> ValueHashMap<&'static str, Token> {
    let mut keywords = ValueHashMap::new();
    keywords.insert("and", Token::And);
    keywords.insert("break", Token::Break);
    keywords.insert("do", Token::Do);
    keywords.insert("else", Token::Else);
    keywords.insert("elseif", Token::Elseif);
    keywords.insert("end", Token::End);
    keywords.insert("false", Token::False);
    keywords.insert("for", Token::For);
    keywords.insert("function", Token::Function);
    keywords.insert("goto", Token::Goto);
    keywords.insert("if", Token::If);
    keywords.insert("in", Token::In);
    keywords.insert("local", Token::Local);
    keywords.insert("nil", Token::Nil);
    keywords.insert("not", Token::Not);
    keywords.insert("or", Token::Or);
    keywords.insert("repeat", Token::Repeat);
    keywords.insert("return", Token::Return);
    keywords.insert("then", Token::Then);
    keywords.insert("true", Token::True);
    keywords.insert("until", Token::Until);
    keywords.insert("while", Token::While);
    keywords
}