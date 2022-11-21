#[derive(Debug)]
pub enum ASTNode{
    Number(f64),
    Variable(String),
    UnaryExpr{
        op: String,
        lhs: Box<ASTNode>
    },
    BinaryExpr{
        op: String,
        lhs: Box<ASTNode>,
        rhs: Box<ASTNode>,
    },
    MethodCall{
        name: String,
        parameter: Box<ASTNode>,
    }
}
