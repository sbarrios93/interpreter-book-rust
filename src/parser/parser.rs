use std::fmt;

// src/parser/parser.rs
use crate::{
    ast::ast::{Expression, Identifier, LetStatement, Program, Statement},
    lexer::lexer::{Lexer, Token},
};
use anyhow::*;

#[derive(Debug)]
pub enum ParserError {
    UnexpectedToken { want: String, got: String },
    MissingIdentifier(Token),
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParserError::UnexpectedToken { want, got } => write!(
                f,
                "parser found unexpected token: {}, expected: {}",
                got, want,
            ),
            ParserError::MissingIdentifier(token) => {
                write!(
                    f,
                    "Was expecting identifier, got {}",
                    format!("{}", token.token_literal())
                )
            }
        }
    }
}

#[derive(Debug)]
struct Parser {
    lexer: Lexer,
    current_token: Token,
    peek_token: Token,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Parser {
        let mut parser = Parser {
            lexer,
            current_token: Token::Illegal,
            peek_token: Token::Illegal,
        };

        parser.next_token().unwrap();
        parser.next_token().unwrap();

        return parser;
    }

    pub fn next_token(&mut self) -> Result<()> {
        self.current_token = std::mem::replace(&mut self.peek_token, self.lexer.next_token()?);
        return Ok(());
    }

    pub fn parse_program(&mut self) -> Result<Program> {
        let mut program = Program { statements: vec![] };

        while self.current_token != Token::EOF {
            let statement = self.parse_statement()?;
            program.statements.push(statement);
            self.next_token()
                .context("Error occurred when moving to the next token")?
        }

        return Ok(program);
    }

    fn parse_statement(&mut self) -> Result<Statement> {
        match self.current_token {
            Token::Let => Ok(Statement::Let(self.parse_let_statement()?)),
            _ => Err(anyhow!("Invalid token: {:?}", self.current_token)),
        }
    }

    fn parse_let_statement(&mut self) -> Result<LetStatement> {
        self.next_token();

        let identifier = self.read_identifier()?.clone();

        self.expect_peek(Token::Assign)?;
        self.next_token();

        while !self.current_token_is(Token::Semicolon) {
            self.next_token();
        }

        let stmt = LetStatement {
            token: Token::Let,
            name: Identifier {
                token: Token::Ident(identifier.clone()),
                value: identifier.clone(),
            },
            // TODO: We are skipping the expression for now
            value: Expression::Identifier(Identifier {
                token: Token::Int("5".into()),
                value: "5".into(),
            }),
        };

        return Ok(stmt);
    }
    fn read_identifier(&mut self) -> Result<&String> {
        match self.current_token {
            Token::Ident(ref identifier) => Ok(identifier),
            _ => bail!(ParserError::MissingIdentifier(self.current_token.clone())),
        }
    }

    fn current_token_is(&self, token: Token) -> bool {
        return self.current_token == token;
    }

    fn peek_token_is(&self, token: &Token) -> bool {
        return self.peek_token == *token;
    }

    fn expect_peek(&mut self, token: Token) -> Result<()> {
        if self.peek_token_is(&token) {
            self.next_token();
            return Ok(());
        } else {
            bail!(ParserError::UnexpectedToken {
                want: format!("{}", token.token_literal()),
                got: format!("{}", self.peek_token.token_literal())
            })
        }
    }
}
#[cfg(test)]
mod test {

    use super::*;
    use anyhow::{bail, Ok, Result};

    fn let_statement_components(statement: &Statement, name: &str) -> Result<()> {
        match statement {
            Statement::Let(let_statement) => {
                assert_eq!(let_statement.token_literal(), "let");
                assert_eq!(let_statement.name.value, name);
                // if let Expression::Identifier(ident) = &let_statement.value {
                //     assert_eq!(ident.token_literal(), "let");
                // } else {
                //     bail!("let_statement.value is not Identifier");
                // }
            }
            _ => bail!("error"),
        }

        Ok(())
    }

    #[test]
    fn let_statements() -> Result<()> {
        let input = r#"let x = 5;
        let y = 10;
        let foobar = 838383;"#;

        let lexer = Lexer::new(input.into());
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program()?;

        if program.statements.len() != 3 {
            bail!(
                "program.Statements does not contain 3 statements, got {}",
                program.statements.len()
            )
        }

        let expected_identifiers = vec!["x", "y", "foobar"];

        for (idx, ident) in expected_identifiers.iter().enumerate() {
            let_statement_components(&program.statements[idx], &ident)?;
        }

        Ok(())
    }
}
