use std::io::Read;
use crate::lex::{Token, Lex};
use crate::value::Value;
use crate::bytecode::ByteCode;

#[derive(Debug)]
pub struct ParseProto<T:Read> {
    pub constants: Vec::<Value>,
    pub byte_codes: Vec::<ByteCode>,
    locals: Vec::<String>,
    lex: Lex<T>,
}

impl<T:Read> ParseProto<T> {
    pub fn load(input: T) -> ParseProto<T> {
        let mut proto = ParseProto {
            constants: Vec::new(),
            byte_codes: Vec::new(),
            locals: Vec::new(),
            lex: Lex::new(input),
        };

        proto.chunk();

        println!("\n---- Start of ParseProto ----");
        println!("constants: {:?}", &proto.constants);
        println!("byte_codes:");
        for c in proto.byte_codes.iter() {
            println!("  {:?}", c);
        }
        println!("---- End of ParseProto ----\n");

        proto
    }

    fn chunk(&mut self) {
        loop {
            match self.lex.next() {
                Some(Token::Name(name)) => {
                    match self.lex.peek() {
                        Some(&Token::Assign) => self.assignment(name),
                        Some(_) => self.function_call(name),
                        None => {}
                    }
                }
                Some(Token::Local) => self.local(),
                Some(t) => panic!("unexpected token: {t:?}"),
                None => break
            }
        }
    }

    // Name LiteralString
    // Name ( exp )
    fn function_call(&mut self, name: String) {
        let ifunc = self.locals.len();
        let iarg = ifunc + 1;

        // function, variable
        let code = self.load_var(ifunc, name);
        self.byte_codes.push(code);

        // argument, (exp) or "string"
        match self.lex.next() {
            Some(Token::ParL) => { // '('
                self.load_exp(iarg);

                if self.lex.next() != Some(Token::ParR) { // ')'
                    panic!("Expect `)` get None");
                }
            }
            Some(Token::String(s)) => {
                let code = self.load_const(iarg, s);
                self.byte_codes.push(code);
            }
            _ => panic!("expected string"),
        }

        self.byte_codes.push(ByteCode::Call(ifunc as u8, 1));
    }

    // local Name = exp
    fn local(&mut self) {
        let var = match self.lex.next() {
            Some(Token::Name(var)) => var,
            Some(t) => panic!("Expect variable, got {t:?}"),
            _ => panic!("Invalid end")
        };

        match self.lex.next() {
            Some(Token::Assign) => {},
            _ => panic!("Expected `=`"),
        }

        self.load_exp(self.locals.len());
        // add to locals after load_exp()
        self.locals.push(var);
    }

    fn assignment(&mut self, var: String) {
        self.lex.next(); // `=`

        if let Some(i) = self.get_local(&var) {
            // local variable
            self.load_exp(i);
        } else {
            // global variable
            let dst = self.add_const(var) as u8;

            let code = match self.lex.next() {
                // from const values
                Some(Token::Nil) => ByteCode::SetGlobalConst(dst, self.add_const(()) as u8),
                Some(Token::True) => ByteCode::SetGlobalConst(dst, self.add_const(true) as u8),
                Some(Token::False) => ByteCode::SetGlobalConst(dst, self.add_const(false) as u8),
                Some(Token::Integer(i)) => ByteCode::SetGlobalConst(dst, self.add_const(i) as u8),
                Some(Token::Float(f)) => ByteCode::SetGlobalConst(dst, self.add_const(f) as u8),
                Some(Token::String(s)) => ByteCode::SetGlobalConst(dst, self.add_const(s) as u8),

                // from variable
                Some(Token::Name(var_name)) =>
                    if let Some(i) = self.get_local(&var_name) {
                        // local variable
                        ByteCode::SetGlobal(dst, i as u8)
                    } else {
                        // global variable
                        ByteCode::SetGlobalGlobal(dst, self.add_const(var_name) as u8)
                    }

                _ => panic!("invalid argument"),
            };
            self.byte_codes.push(code);
        }
    }

    /// check if a value is already in constants table, if is return the postion
    /// else, return the top index of the constants table
    fn add_const<K>(&mut self, c: K) -> usize 
        where K:Into<Value>
    {
        let c = c.into();
        let constants = &mut self.constants;
        constants.iter().position(|v| v == &c)
            .unwrap_or_else(|| {
                constants.push(c);
                constants.len() - 1
            })
    }

    fn load_const<K>(&mut self, dst: usize, c: K) -> ByteCode
        where K:Into<Value>
    {
        ByteCode::LoadConst(dst as u8, self.add_const(c) as u16)
    }

    fn load_var(&mut self, dst: usize, name: String) -> ByteCode {
        if let Some(i) = self.get_local(&name) {
            // local variable
            ByteCode::Move(dst as u8, i as u8)
        } else {
            // global variable
            let ic = self.add_const(name);
            ByteCode::GetGlobal(dst as u8, ic as u8)
        }
    }

    fn get_local(&self, name: &str) -> Option<usize> {
        self.locals.iter().rposition(|v| v == name)
    }

    fn load_exp(&mut self, dst: usize) {
        let code = match self.lex.next() {
            Some(Token::Nil) => ByteCode::LoadNil(dst as u8),
            Some(Token::True) => ByteCode::LoadBool(dst as u8, true),
            Some(Token::False) => ByteCode::LoadBool(dst as u8, false),
            Some(Token::Integer(i)) =>
                if let Ok(ii) = i16::try_from(i) {
                    ByteCode::LoadInt(dst as u8, ii)
                } else {
                    self.load_const(dst, Value::Integer(i))
                }
            Some(Token::Float(f)) => self.load_const(dst, Value::Float(f)),
            Some(Token::String(s)) => self.load_const(dst, s),
            Some(Token::Name(var)) => self.load_var(dst, var),
            _ => panic!("invalid argument"),
        };
        self.byte_codes.push(code);
    }
}