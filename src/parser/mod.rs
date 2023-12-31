use std::fmt;

// src/parser/parser.rs
use crate::{
    ast::{
        Expression, ExpressionStatement, Identifier, LetStatement, Program, ReturnStatement,
        Statement,
    },
    lexer::{Lexer, Token},
};
use anyhow::*;

#[derive(Debug)]
pub enum ParserError {
    UnexpectedToken { want: String, got: String },
    MissingIdentifier(Token),
    PrefixExpressionNotImplemented(Token),
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
                write!(f, "Was expecting identifier, got {}", token.token_literal())
            }
            ParserError::PrefixExpressionNotImplemented(token) => {
                write!(
                    f,
                    "Expression for token {} not implemented on prefix",
                    token.token_literal()
                )
            }
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
pub enum OperatorPrecedence {
    Lowest,      // Lowest precedence
    Equals,      // ==
    LessGreater, // > or <
    Sum,         // +
    Product,     // *
    Prefix,      // -X or !X
    Call,        // myFunction(X)
}

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

        parser
    }

    pub fn next_token(&mut self) -> Result<()> {
        self.current_token = std::mem::replace(&mut self.peek_token, self.lexer.next_token()?);
        Ok(())
    }

    pub fn parse_program(&mut self) -> Result<Program> {
        let mut program = Program { statements: vec![] };

        while self.current_token != Token::EOF {
            let statement = self.parse_statement()?;
            program.statements.push(statement);
            self.next_token()
                .context("Error occurred when moving to the next token")?
        }

        Ok(program)
    }

    fn parse_statement(&mut self) -> Result<Statement> {
        match self.current_token {
            Token::Let => Ok(Statement::Let(self.parse_let_statement()?)),
            Token::Return => Ok(Statement::Return(self.parse_return_statement()?)),
            _ => Ok(Statement::Expression(self.parse_expression_statement()?)),
        }
    }

    fn parse_let_statement(&mut self) -> Result<LetStatement> {
        self.next_token()?;

        let identifier = self.read_identifier()?.clone();

        self.expect_peek(Token::Assign)?;
        self.next_token()?;

        while !self.current_token_is(Token::Semicolon) {
            self.next_token()?;
        }

        Ok(LetStatement {
            token: Token::Let,
            name: Identifier {
                token: Token::Ident(identifier.clone()),
                value: identifier,
            },
            // TODO: We are skipping the expression for now
            value: Expression::Identifier(Identifier {
                token: Token::Int("5".into()),
                value: "5".into(),
            }),
        })
    }

    fn parse_return_statement(&mut self) -> Result<ReturnStatement> {
        while !self.current_token_is(Token::Semicolon) {
            self.next_token()?;
        }

        Ok(ReturnStatement {
            token: Token::Return,
            return_value: Expression::Identifier(Identifier {
                token: Token::Int("5".into()),
                value: "5".into(),
            }),
        })
    }

    fn parse_expression_statement(&mut self) -> Result<ExpressionStatement> {
        let expression = self.parse_expression(OperatorPrecedence::Lowest)?;

        if self.peek_token_is(&Token::Semicolon) {
            self.next_token()?;
        }
        Ok(ExpressionStatement {
            token: self.current_token.clone(),
            expression,
        })
    }

    fn parse_expression(&self, precedence: OperatorPrecedence) -> Result<Expression> {
        let left_expression = self.parse_prefix()?;

        Ok(left_expression)
    }

    fn parse_prefix(&self) -> Result<Expression> {
        match self.current_token {
            Token::Ident(_) => Ok(self.parse_identifier()),
            _ => bail!(ParserError::PrefixExpressionNotImplemented(
                self.current_token.clone()
            )),
        }
    }

    fn parse_identifier(&self) -> Expression {
        Expression::Identifier(Identifier {
            token: self.current_token.clone(),
            value: self.current_token.token_literal().to_string(),
        })
    }

    fn read_identifier(&mut self) -> Result<&String> {
        match self.current_token {
            Token::Ident(ref identifier) => Ok(identifier),
            _ => bail!(ParserError::MissingIdentifier(self.current_token.clone())),
        }
    }

    fn current_token_is(&self, token: Token) -> bool {
        self.current_token == token
    }

    fn peek_token_is(&self, token: &Token) -> bool {
        self.peek_token == *token
    }

    fn expect_peek(&mut self, token: Token) -> Result<()> {
        if self.peek_token_is(&token) {
            self.next_token()?;
            Ok(())
        } else {
            bail!(ParserError::UnexpectedToken {
                want: token.token_literal().to_string(),
                got: self.peek_token.token_literal().to_string()
            })
        }
    }
}
#[cfg(test)]
mod test {

    use super::*;

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
            _ => bail!("statement not LetStatement"),
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
            let_statement_components(&program.statements[idx], ident)?;
        }

        Ok(())
    }

    #[test]
    fn return_statements() -> Result<()> {
        let input = r#"return 5;
        return 10;
        return 993322; "#;

        let lexer = Lexer::new(input.into());
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program()?;

        if program.statements.len() != 3 {
            bail!(
                "program.Statements does not contain 3 statements, got {}",
                program.statements.len()
            )
        }

        for statement in program.statements {
            match statement {
                Statement::Return(return_statement) => {
                    assert_eq!(return_statement.token_literal(), "return");
                }
                _ => bail!("statement not ReturnStatement"),
            }
        }

        Ok(())
    }

    #[test]
    fn identifier_expression() -> Result<()> {
        let input = "foobar";

        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program()?;

        if program.statements.len() != 1 {
            bail!(
                "program.Statements does not contain 3 statements, got {}",
                program.statements.len()
            )
        }
        for statement in program.statements {
            match statement {
                Statement::Expression(expression_statement) => {
                    assert_eq!(expression_statement.token_literal(), "foobar");
                    println!("{}", expression_statement.token_literal());
                    assert_eq!(expression_statement.expression.to_string(), "foobar")
                }
                _ => bail!("Statement not ExpressionStatement"),
            }
        }
        Ok(())
    }
}
