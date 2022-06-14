use crate::{ParseResult};
use crate::lexer::{C1Lexer, C1Token};
pub struct C1Parser<'a> {
    pub lexer: C1Lexer<'a>,
    pub result: ParseResult,
}

impl<'a> C1Parser<'a>{

    pub fn parse(text: &str) -> ParseResult {
        let lexer = C1Lexer::new(text);
        let result = Result::Ok(());
        let mut parse = C1Parser{lexer, result}; 
        while parse.lexer.current_token()!= None && parse.result == Result::Ok(()){
            parse.program();
        }
        parse.result
    }

    pub fn eat(&mut self) {
        let _ = &(self.lexer).eat();
    }
    
    pub fn check_and_eat_token(&mut self, token: C1Token) {
        if self.result != Result::Ok(()) {
            return;
        }
        if self.current_matches(token){
            self.eat();
        }else {
            self.erro();
        }
    }

    pub fn current_matches(&self, token: C1Token) -> bool{
        if self.lexer.current_token().unwrap() ==  token{
            return true;
        }
        false
    }
    
    pub fn next_matches(&self, token: C1Token) -> bool{
        if self.lexer.peek_token().unwrap() ==  token{
            return true;
        }
        false
    }
    
    pub fn erro(&mut self){
        let err = self.lexer.current_line_number().unwrap();
        self.result = Result::Err(err.to_string());
        return;
    }

    pub fn factor(&mut self) {
        if self.result != Result::Ok(()) {
            return;
        }
        if self.current_matches(C1Token::ConstInt) {
            self.check_and_eat_token(C1Token::ConstInt);
        }else if self.current_matches(C1Token::ConstFloat) {
            self.check_and_eat_token(C1Token::ConstFloat);
        }else if self.current_matches(C1Token::ConstBoolean) {
            self.check_and_eat_token(C1Token::ConstBoolean);
        }else if self.current_matches(C1Token::Identifier) {
            if self.next_matches(C1Token::LeftParenthesis) {
                self.functioncall();
            }else {
                self.check_and_eat_token(C1Token::Identifier);
            }         
        }else {
            self.check_and_eat_token(C1Token::LeftParenthesis);
            self.assignment();
            self.check_and_eat_token(C1Token::RightParenthesis);
        }
    }
    
    pub fn term(&mut self) {
        if self.result != Result::Ok(()) {
            return;
        }
        self.factor();
        self.term1();
    }


    pub fn term1(&mut self) {
        if self.current_matches(C1Token::Asterisk)||self.current_matches(C1Token::Slash)|| self.current_matches(C1Token::And){
            if self.current_matches(C1Token::Asterisk) {
                self.check_and_eat_token(C1Token::Asterisk);
                self.factor();
            }else if self.current_matches(C1Token::Slash) {
                self.check_and_eat_token(C1Token::Slash);
                self.factor();
            }else if self.current_matches(C1Token::And) {
                self.check_and_eat_token(C1Token::And);
                self.factor();
            }
            self.term1();
        } 
    }
    
    pub fn simpexpr(&mut self) {
        if self.result != Result::Ok(()) {
            return;
        }
        if self.current_matches(C1Token::Minus) {
            self.check_and_eat_token(C1Token::Minus);
        }
        self.term();
        self.simpexpr1();
    }


    pub fn simpexpr1(&mut self) {
        if self.current_matches(C1Token::Plus)||self.current_matches(C1Token::Minus)|| self.current_matches(C1Token::Or){
            if self.current_matches(C1Token::Or) {
                self.check_and_eat_token(C1Token::Or);
                self.term();
            }else if self.current_matches(C1Token::Minus) {
                self.check_and_eat_token(C1Token::Minus);
                self.term();
            }else if self.current_matches(C1Token::Plus) {
                self.check_and_eat_token(C1Token::Plus);
                self.term();
            }
            self.simpexpr1();
        } 
    }

    pub fn expr(&mut self) {
        if self.result != Result::Ok(()) {
            return;
        }
        self.simpexpr();
        if self.current_matches(C1Token::Equal) {
            self.check_and_eat_token(C1Token::Equal);
            self.simpexpr();
        }else if self.current_matches(C1Token::NotEqual) {
            self.check_and_eat_token(C1Token::NotEqual);
            self.simpexpr();
        }else if self.current_matches(C1Token::LessEqual) {
            self.check_and_eat_token(C1Token::LessEqual);
            self.simpexpr()
        }else if self.current_matches(C1Token::GreaterEqual) {
            self.check_and_eat_token(C1Token::GreaterEqual);
            self.simpexpr();
        }else if self.current_matches(C1Token::Less) {
            self.check_and_eat_token(C1Token::Less);
            self.simpexpr();
        }else if self.current_matches(C1Token::Greater) {
            self.check_and_eat_token(C1Token::Greater);
            self.simpexpr();
        }
    }
    
    pub fn assignment(&mut self) {
        if self.result != Result::Ok(()) {
            return;
        }
        if self.current_matches(C1Token::Identifier) && self.next_matches(C1Token::Assign){
            self.check_and_eat_token(C1Token::Identifier);
            self.check_and_eat_token(C1Token::Assign);
            self.assignment();
        }else {
            self.expr();
        }
    }
    
    pub fn statassignment(&mut self) {
        if self.result != Result::Ok(()) {
            return;
        }
        self.check_and_eat_token(C1Token::Identifier);
        self.check_and_eat_token(C1Token::Assign);
        self.assignment();
    }
    
    pub fn rtype(&mut self) {
        if self.result != Result::Ok(()) {
            return;
        }
        if self.current_matches(C1Token::KwBoolean) {
            self.check_and_eat_token(C1Token::KwBoolean);
        }else if self.current_matches(C1Token::KwFloat) {
            self.check_and_eat_token(C1Token::KwFloat);
        }else if self.current_matches(C1Token::KwInt) {
            self.check_and_eat_token(C1Token::KwInt);
        }else if self.current_matches(C1Token::KwVoid) {
            self.check_and_eat_token(C1Token::KwVoid);
        }else {
            self.erro();
        }
    }
    
    pub fn printf(&mut self) {
        if self.result != Result::Ok(()) {
            return;
        }
        self.check_and_eat_token(C1Token::KwPrintf);
        self.check_and_eat_token(C1Token::LeftParenthesis);
        self.assignment();
        self.check_and_eat_token(C1Token::RightParenthesis);

    }
    
    pub fn returnstatement(&mut self) {
        if self.result != Result::Ok(()) {
            return;
        }
        self.check_and_eat_token(C1Token::KwReturn);
        if self.current_matches(C1Token::Semicolon) == false {
            self.assignment();
        }
    }
    
    pub fn ifstatement(&mut self) {
        if self.result != Result::Ok(()) {
            return;
        }
        self.check_and_eat_token(C1Token::KwIf);
        self.check_and_eat_token(C1Token::LeftParenthesis);
        self.assignment();
        self.check_and_eat_token(C1Token::RightParenthesis);
        self.block();
    }
    
    pub fn statement(&mut self) {
        if self.result != Result::Ok(()) {
            return;
        }
        if self.current_matches(C1Token::KwIf) {
            self.ifstatement();            
        }else if self.current_matches(C1Token::KwReturn) {
            self.returnstatement();
            self.check_and_eat_token(C1Token::Semicolon);
        }else if self.current_matches(C1Token::KwPrintf) {
            self.printf();
            self.check_and_eat_token(C1Token::Semicolon);
        }else if self.current_matches(C1Token::Identifier) && self.next_matches(C1Token::Assign){
            self.statassignment();
            self.check_and_eat_token(C1Token::Semicolon);
        }else if self.current_matches(C1Token::Identifier) && self.next_matches(C1Token::LeftParenthesis){
            self.functioncall();
            self.check_and_eat_token(C1Token::Semicolon);
        }else {
            self.erro();
        }
    }
    
    pub fn block(&mut self) {
        if self.result != Result::Ok(()) {
            return;
        }
        if self.current_matches(C1Token::LeftBrace) {
            self.check_and_eat_token(C1Token::LeftBrace);
            self.statementlist();
            self.check_and_eat_token(C1Token::RightBrace);
        }else {
            self.statement();
        }
    }
    

    pub fn statementlist(&mut self) {
        if self.result != Result::Ok(()) {
            return;
        }
        self.statementlist1(); 
    }


    pub fn statementlist1(&mut self) {
        if self.current_matches(C1Token::RightBrace)==false {
            self.block();
            self.statementlist();
        }    
    }

    pub fn functioncall(&mut self) {
        if self.result != Result::Ok(()) {
            return;
        }
        self.check_and_eat_token(C1Token::Identifier);
        self.check_and_eat_token(C1Token::LeftParenthesis);
        self.check_and_eat_token(C1Token::RightParenthesis);
    }
    
    pub fn functiondefinition(&mut self) {
        if self.result != Result::Ok(()) {
            return;
        }
        self.rtype();
        self.check_and_eat_token(C1Token::Identifier);
        self.check_and_eat_token(C1Token::LeftParenthesis);
        self.check_and_eat_token(C1Token::RightParenthesis);
        self.check_and_eat_token(C1Token::LeftBrace);
        self.statementlist();
        self.check_and_eat_token(C1Token::RightBrace);
    }
    
    pub fn program(&mut self) {
        if self.result != Result::Ok(()) {
            return;
        }
        self.functiondefinition();
    }
    
}
