// src/ast/ast.rs
use crate::lexer::lexer::Token;

#[derive(Debug)]
pub enum Statement {
    Let(LetStatement),
}

#[derive(Debug)]
pub enum Expression {
    Identifier(Identifier),
}

pub struct Program {
    pub statements: Vec<Statement>,
}

impl Program {
    fn token_literal(&self) -> &str {
        if !self.statements.is_empty() {
            match &self.statements[0] {
                Statement::Let(let_statement) => let_statement.token_literal(),
            }
        } else {
            return "";
        }
    }
}

#[derive(Debug)]
pub struct Identifier {
    pub token: Token,
    pub value: String,
}

#[derive(Debug)]
pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
    pub value: Expression,
}

impl LetStatement {
    fn statement_node(&self) {}
    pub fn token_literal(&self) -> &str {
        &self.token.token_literal()
    }
}

impl Identifier {
    fn expression_node(&self) {}
    pub fn token_literal(&self) -> &str {
        return &self.token.token_literal();
    }
}
