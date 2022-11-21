use std::{collections::LinkedList,result::Result,fmt};

#[derive(Debug, Clone)]
pub struct LexerError;

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Issue occurred with lexer")
    }
}

#[derive(Debug,PartialEq)]
pub enum TokenType{
    IDENT,
    SYMBOL,
    NUMBER,
    OBRAC,
    CBRAC,
}

pub type Token = (TokenType, Option<String>);
pub type LexerStack = LinkedList<Token>;

pub fn lexer(input: &String)->Result<LexerStack, LexerError>{
    let symbols = vec!['+','-','*','/'];

    let mut lexer_stack = LexerStack::new();

    for c in input.chars(){
        if c.is_whitespace() {}
        else if c == '('{
            lexer_stack.push_back((TokenType::OBRAC, None));
        }
        else if c == ')'{
            lexer_stack.push_back((TokenType::CBRAC, None));
        }
        else if symbols.contains(&c){
            let mut handled = false;    
            if !lexer_stack.is_empty(){
                let (back_type, back_value) = lexer_stack.back_mut().unwrap();
                if c == '*' && *back_type == TokenType::SYMBOL && *back_value == Some(c.to_string()) {
                    *back_value = Some("**".to_string());
                    handled = true;
                }
            }
            if !handled{
                lexer_stack.push_back((TokenType::SYMBOL, Some(c.to_string())));
            }
        }
        else if c.is_alphanumeric(){
            let mut handled = false;            
            if !lexer_stack.is_empty(){
                let (back_type, back_value) = lexer_stack.back_mut().unwrap();
                if *back_type == TokenType::IDENT{
                    *back_value = Some(back_value.clone().unwrap() + &c.to_string());
                    handled = true;
                }
                else if c.is_numeric(){
                    if *back_type == TokenType::NUMBER{
                        *back_value = Some(back_value.clone().unwrap() + &c.to_string());
                        handled = true;
                    }
                }
            }
            if !handled {
                if c.is_numeric(){
                    lexer_stack.push_back((TokenType::NUMBER, Some(c.to_string())))
                }
                else{
                    lexer_stack.push_back((TokenType::IDENT, Some(c.to_string())))
                }
            }
        }
        else{
            return Err(LexerError);
        }
    }   
    Ok(lexer_stack)
}
