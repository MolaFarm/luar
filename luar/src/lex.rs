use std::io::{Read, Bytes};
use std::iter::Peekable;
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
    String(Vec<u8>),

    // name of variables or table keys
    Name(String),

    // // end
    // Eos,
}

#[derive(Debug)]
pub struct Lex<T: Read> {
    input: Peekable::<Bytes::<T>>,
    ahead: Option<Token>,
}

impl<T:Read> Lex<T> {
    pub fn new(input: T) -> Self {
        Lex{
            input:input.bytes().peekable(),
            ahead:None
        }
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
        if let Some(_nextbyte) = self.read_next_byte() {
            match _nextbyte {
                b'\n' | b'\r' | b'\t' | b' ' => self._next(), // skip whitespaces
                b'+' => Some(Token::Add), 
                b'*' => Some(Token::Mul), 
                b'%' => Some(Token::Mod),
                b'^' => Some(Token::Pow),
                b'#' => Some(Token::Len),
                b'&' => Some(Token::BitAnd),
                b'|' => Some(Token::BitOr),
                b'(' => Some(Token::ParL),
                b')' => Some(Token::ParR),
                b'{' => Some(Token::CurlyL),
                b'}' => Some(Token::CurlyR),
                b'[' => Some(Token::SqurL),
                b']' => Some(Token::SqurR),
                b';' => Some(Token::SemiColon),
                b',' => Some(Token::Comma),
                b'/' => Some(self.check_ahead(b'/', Token::Idiv, Token::Div)),
                b'=' => Some(self.check_ahead(b'=', Token::Equal, Token::Assign)),
                b'~' => Some(self.check_ahead(b'=', Token::NotEq, Token::BitXor)),
                b':' => Some(self.check_ahead(b':', Token::DoubColon, Token::Colon)),
                b'<' => Some(self.check_ahead2(b'=', Token::LesEq, b'<', Token::ShiftL, Token::Less)),
                b'>' => Some(self.check_ahead2(b'=', Token::GreEq, b'>', Token::ShiftR, Token::Greater)),
                b'\'' | b'"' => Some(self.read_string(_nextbyte)),
                b'.' => match self.peek_byte() {
                    b'.' => {
                        self.read_next_byte();
                        if self.peek_byte() == b'.' {
                            self.read_next_byte();
                            Some(Token::Dots)
                        } else {
                            Some(Token::Concat)
                        }
                    },
                    b'0'..=b'9' => {
                        Some(self.read_number_fraction(0))
                    },
                    _ => {
                        Some(Token::Dot)
                    },
                },
                b'-' => {
                    if self.peek_byte() == b'-' {
                        self.read_next_byte();
                        self.read_comment();
                        self._next()
                    } else {
                        Some(Token::Sub)
                    }
                },
                b'0'..=b'9' => Some(self.read_number(_nextbyte)),
                b'A'..=b'Z' | b'a'..=b'z' | b'_' => Some(self.read_name(_nextbyte)),
                _ => panic!("invalid char {_nextbyte}"),
            }
        } else {
            None
        }

    }

    fn read_next_byte(&mut self) -> Option<u8> {
        self.input.next().and_then(|r|Some(r.unwrap()))
    }
    
    fn peek_byte(&mut self) -> u8 {
        match self.input.peek() {
            Some(Ok(byt)) => *byt,
            Some(_) => panic!("lex peek error"),
            None => b'\0', // good for usage
        }
    }

    /// ## check_ahead 
    /// 
    /// Is used to infer the combined operator
    /// 
    /// ```ignore
    /// use luar::lex::Token;
    /// // Assume the frist char is '='
    /// check_head(b'=', Token::Equal,Token::Assign);
    /// // check_head will infer if the target operator is Assign op '=' or Equal op'=='
    /// ```
    fn check_ahead(&mut self, ahead: u8, long: Token, short: Token) -> Token {
        if self.peek_byte() == ahead {
            self.read_next_byte();
            long
        } else {
            short
        }
    }
    /// Simliar to `check_ahead` but has 2 instead of 1 branches
    fn check_ahead2(&mut self, ahead1: u8, long1: Token, ahead2: u8, long2: Token, short: Token) -> Token {
        let byte:u8 = self.peek_byte();
        match byte {
            b if b == ahead1 => {
                self.read_next_byte();
                long1
            },
            b if b == ahead2 => {
                self.read_next_byte();
                long2
            },
            _ => short
        }
    }

    fn read_number(&mut self, first: u8) -> Token {
        // heximal
        if first == b'0' {
            let second = self.peek_byte();
            if second == b'x' || second == b'X' {
                return self.read_heximal();
            }
        }

        // decimal
        let mut n = (first - b'0') as i64;
        loop {
            let byt = self.peek_byte();
            match char::to_digit(byt as char, 10) {
                Some(d) => {
                    self.read_next_byte();
                    n = n * 10 + d as i64;
                },
                None if byt == b'.' => return self.read_number_fraction(n),
                None if byt == b'e' || byt == b'E' => return self.read_number_exp(n as f64),
                None => break,
            }
        }

        // check following
        let fch = self.peek_byte();
        if (fch as char).is_alphabetic() || fch == b'.' {
            panic!("malformat number");
        }

        Token::Integer(n)
    }
    

    fn read_number_fraction(&mut self, i: i64) -> Token {
        self.read_next_byte(); // skip '.'

        let mut n: i64 = 0;
        let mut x: f64 = 1.0;
        loop {
            let byt = self.peek_byte();
            if let Some(d) = char::to_digit(byt as char, 10) {
                self.read_next_byte();
                n = n * 10 + d as i64;
                x *= 10.0;
            } else {
                break;
            }
        }
        Token::Float(i as f64 + n as f64 / x)
    }

    fn read_number_exp(&mut self, _: f64) -> Token {
        self.read_next_byte(); // skip 'e'
        todo!("lex number exp")
    }
    
    fn read_heximal(&mut self) -> Token {
        self.read_next_byte(); // skip 'x'
        todo!("lex heximal")
    }

    fn read_string(&mut self, quote: u8) -> Token {
        let mut s = Vec::new();
        loop {
            match self.read_next_byte().expect("unfinished string") {
                b'\n' => panic!("unfinished string"),
                b'\\' => s.push(self.read_escape()),
                byt if byt == quote => break,
                byt => s.push(byt),
            }
        }
        Token::String(s)
    }

    fn read_escape(&mut self) -> u8 {
        match self.read_next_byte().expect("string escape") {
            b'a' => 0x07,
            b'b' => 0x08,
            b'f' => 0x0c,
            b'v' => 0x0b,
            b'n' => b'\n',
            b'r' => b'\r',
            b't' => b'\t',
            b'\\' => b'\\',
            b'"' => b'"',
            b'\'' => b'\'',
            b'x' => { // format: \xXX
                let n1 = char::to_digit(self.read_next_byte().unwrap() as char, 16).unwrap();
                let n2 = char::to_digit(self.read_next_byte().unwrap() as char, 16).unwrap();
                (n1 * 16 + n2) as u8
            }
            ch@b'0'..=b'9' => { // format: \d[d[d]]
                let mut n = char::to_digit(ch as char, 10).unwrap(); // TODO no unwrap
                if let Some(d) = char::to_digit(self.peek_byte() as char, 10) {
                    self.read_next_byte();
                    n = n * 10 + d;
                    if let Some(d) = char::to_digit(self.peek_byte() as char, 10) {
                        self.read_next_byte();
                        n = n * 10 + d;
                    }
                }
                u8::try_from(n).expect("decimal escape too large")
            }
            _ => panic!("invalid string escape")
        }
    }

    fn read_name(&mut self, first: u8) -> Token {
        let mut s = String::new();
        s.push(first as char);

        loop {
            let ch = self.peek_byte() as char;
            if ch.is_alphanumeric() || ch == '_' {
                self.read_next_byte();
                s.push(ch);
            } else {
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
        match self.read_next_byte() {
            None => (),
            Some(b'[') => todo!("long comment"),
            Some(_) => { // line comment
                while let Some(byt) = self.read_next_byte() {
                    if byt == b'\n' {
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