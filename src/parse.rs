use crate::ast::ASTNode;
use crate::lexer::{LexerStack,TokenType};

fn parse_unary(lexer_stack: &mut LexerStack, unary_complete: bool) -> Option<Box<ASTNode>>{
    if unary_complete{
        return None;
    }

    match lexer_stack.pop_front(){
        Some((token_type, token_value)) =>{
            if token_type == TokenType::SYMBOL{
                if token_value == Some("-".to_string()) || token_value == Some("+".to_string()){
                    match parse_expression(lexer_stack, true){
                        Some(expr) => {
                            return Some(Box::<ASTNode>::new(ASTNode::UnaryExpr { op: token_value.unwrap(), lhs: expr }));
                        }
                        None => return None
                    }
                    
                }
                else{
                    return None;
                }
            }
            else{
                lexer_stack.push_front((token_type, token_value));
                return parse_expression(lexer_stack, true)
            }
        }
        None => return None
    }
}

fn parse_series(lexer_stack: &mut LexerStack, lhs: Box<ASTNode>) -> Option<Box<ASTNode>>{
    match lexer_stack.pop_front(){
        Some((token_type, token_value)) =>{
            if token_type == TokenType::SYMBOL{
                match parse_expression(lexer_stack, false){
                    Some(n_expr) => {

                        return Some(
                            Box::<ASTNode>::new(
                                ASTNode::BinaryExpr { 
                                    op: (token_value).unwrap(),  
                                    lhs: lhs,  
                                    rhs: n_expr
                                }
                            )
                        )
                    },
                    None => return None,
                }
            }
            else{
                lexer_stack.push_front((token_type, token_value));
                return Some(lhs);
            }
        },
        None => return Some(lhs)
    }
}

fn parse_primary(lexer_stack: &mut LexerStack) -> Option<Box<ASTNode>>{
    match lexer_stack.pop_front(){
        Some((token_type, token_value)) =>{
            match(token_type){
                TokenType::IDENT => {
                    if token_value == Some("x".to_string()) || token_value == Some("y".to_string()){
                        return Some(Box::<ASTNode>::new(ASTNode::Variable(token_value.unwrap())));
                    }
                    match lexer_stack.front(){
                        Some((n_token_type, n_token_value)) => {
                            if *n_token_type == TokenType::OBRAC{
                                lexer_stack.pop_front();
                                let mut out: Option<Box<ASTNode>> = None;
                                match parse_expression(lexer_stack, false){
                                    Some(expr) => {
                                        out = Some(Box::<ASTNode>::new(
                                            ASTNode::MethodCall { name: token_value.unwrap(), parameter: expr }
                                        ))
                                    }
                                    None => ()
                                }
                                lexer_stack.pop_front();
                                return out;
                            }
                            return None;
                        }
                        None => return None
                    }
                },
                TokenType::NUMBER => {
                    return Some(Box::<ASTNode>::new(
                        ASTNode::Number(token_value.unwrap().parse::<f64>().unwrap())
                    ))
                },
                _ => return None
            }
        }
        None => return None
    }
}

fn parse_expression(lexer_stack: &mut LexerStack, unary_complete: bool) -> Option<Box<ASTNode>>{
    let mut expr = parse_unary(lexer_stack, unary_complete);
    if expr.is_some(){
        return parse_series(lexer_stack, expr.unwrap());
    }
    
    match lexer_stack.front(){
        Some((token_type, token_value)) =>{
            if *token_type == TokenType::OBRAC{
                lexer_stack.pop_front();
                expr = parse_expression(lexer_stack, false);
                lexer_stack.pop_front();
            }
        }
        None => {}
    }

    if expr.is_some(){
        return parse_series(lexer_stack, expr.unwrap())
    }

    return parse_primary(lexer_stack);
}

pub fn parse(lexer_stack: &mut LexerStack) -> Option<Box<ASTNode>>{
    parse_expression(lexer_stack, false)
}
