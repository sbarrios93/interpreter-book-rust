use std::fmt;

// src/ast/ast.rs
use crate::lexer::Token;

#[derive(Debug)]
pub enum Node {
    Program(Program),
    Statement(Statement),
    Expression(Expression),
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Node::Program(p) => p.fmt(f),
            Node::Statement(p) => p.fmt(f),
            Node::Expression(p) => p.fmt(f),
        }
    }
}

#[derive(Debug)]
pub enum Statement {
    Let(LetStatement),
    Return(ReturnStatement),
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::Let(s) => s.fmt(f),
            Statement::Return(s) => s.fmt(f),
        }
    }
}

#[derive(Debug)]
pub enum Expression {
    Identifier(Identifier),
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Identifier(i) => i.fmt(f),
        }
    }
}

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for stmt in &self.statements {
            write!(f, "{}", stmt)?;
        }
        Ok(())
    }
}

impl Program {
    fn token_literal(&self) -> &str {
        if !self.statements.is_empty() {
            match &self.statements[0] {
                Statement::Let(let_statement) => let_statement.token_literal(),
                Statement::Return(return_statement) => return_statement.token_literal(),
            }
        } else {
            ""
        }
    }
}

#[derive(Debug)]
pub struct Identifier {
    pub token: Token,
    pub value: String,
}

impl Identifier {
    fn expression_node(&self) {}
    pub fn token_literal(&self) -> &str {
        self.token.token_literal()
    }
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
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
        self.token.token_literal()
    }
}

impl fmt::Display for LetStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "let {} = {};", self.name, self.value)
    }
}

#[derive(Debug)]
pub struct ReturnStatement {
    pub token: Token,
    pub return_value: Expression,
}

impl ReturnStatement {
    fn statement_node(&self) {}
    pub fn token_literal(&self) -> &str {
        self.token.token_literal()
    }
}

impl fmt::Display for ReturnStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "return {};", self.return_value)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use anyhow::*;

    #[test]
    fn format_return_statement() -> Result<()> {
        let expect = vec!["return 5;", "return 10;", "return 25;"];

        let statements = vec![
            ReturnStatement {
                token: Token::Return,
                return_value: Expression::Identifier(Identifier {
                    token: Token::Int("5".to_string()),
                    value: "5".to_string(),
                }),
            },
            ReturnStatement {
                token: Token::Return,
                return_value: Expression::Identifier(Identifier {
                    token: Token::Int("10".to_string()),
                    value: "10".to_string(),
                }),
            },
            ReturnStatement {
                token: Token::Return,
                return_value: Expression::Identifier(Identifier {
                    token: Token::Int("25".to_string()),
                    value: "25".to_string(),
                }),
            },
        ];

        for (stmt, expect) in statements.iter().zip(expect) {
            assert_eq!(stmt.to_string(), expect)
        }

        Ok(())
    }

    #[test]
    fn format_let_statement() -> Result<()> {
        let expect = vec!["let x = 5;", "let y = 10;", "let z = 25;"];

        let statements = vec![
            LetStatement {
                token: Token::Let,
                name: Identifier {
                    token: Token::Ident("x".to_string()),
                    value: "x".to_string(),
                },
                value: Expression::Identifier(Identifier {
                    token: Token::Int("5".to_string()),
                    value: "5".to_string(),
                }),
            },
            LetStatement {
                token: Token::Let,
                name: Identifier {
                    token: Token::Ident("y".to_string()),
                    value: "y".to_string(),
                },
                value: Expression::Identifier(Identifier {
                    token: Token::Int("10".to_string()),
                    value: "10".to_string(),
                }),
            },
            LetStatement {
                token: Token::Let,
                name: Identifier {
                    token: Token::Ident("z".to_string()),
                    value: "z".to_string(),
                },
                value: Expression::Identifier(Identifier {
                    token: Token::Int("25".to_string()),
                    value: "25".to_string(),
                }),
            },
        ];

        for (stmt, expect) in statements.iter().zip(expect) {
            assert_eq!(stmt.to_string(), *expect)
        }

        Ok(())
    }

    #[test]
    fn format_identifier() -> Result<()> {
        let expect = vec!["x", "y", "z"];
        let identifiers = vec![
            Identifier {
                token: Token::Ident("x".to_string()),
                value: "x".to_string(),
            },
            Identifier {
                token: Token::Ident("y".to_string()),
                value: "y".to_string(),
            },
            Identifier {
                token: Token::Ident("z".to_string()),
                value: "z".to_string(),
            },
        ];

        for (ident, expect) in identifiers.iter().zip(expect) {
            assert_eq!(ident.to_string(), *expect)
        }

        Ok(())
    }

    #[test]
    fn format_expression() -> Result<()> {
        let expect = "x";
        let expression = Expression::Identifier(Identifier {
            token: Token::Ident("x".to_string()),
            value: "x".to_string(),
        });

        assert_eq!(expression.to_string(), expect);
        Ok(())
    }

    #[test]
    fn format_program_single_statement() -> Result<()> {
        let expect = "let x = 5;";
        let program = Program {
            statements: vec![Statement::Let(LetStatement {
                token: Token::Let,
                name: Identifier {
                    token: Token::Ident("x".to_string()),
                    value: "x".to_string(),
                },
                value: Expression::Identifier(Identifier {
                    token: Token::Int("5".to_string()),
                    value: "5".to_string(),
                }),
            })],
        };

        assert_eq!(program.to_string(), expect);
        Ok(())
    }

    #[test]
    fn format_program_multiple_statements() -> Result<()> {
        let expect = "let x = 5;return 10;let y = 15;return 20;";
        let program = Program {
            statements: vec![
                Statement::Let(LetStatement {
                    token: Token::Let,
                    name: Identifier {
                        token: Token::Ident("x".to_string()),
                        value: "x".to_string(),
                    },
                    value: Expression::Identifier(Identifier {
                        token: Token::Int("5".to_string()),
                        value: "5".to_string(),
                    }),
                }),
                Statement::Return(ReturnStatement {
                    token: Token::Return,
                    return_value: Expression::Identifier(Identifier {
                        token: Token::Int("10".to_string()),
                        value: "10".to_string(),
                    }),
                }),
                Statement::Let(LetStatement {
                    token: Token::Let,
                    name: Identifier {
                        token: Token::Ident("y".to_string()),
                        value: "y".to_string(),
                    },
                    value: Expression::Identifier(Identifier {
                        token: Token::Int("15".to_string()),
                        value: "15".to_string(),
                    }),
                }),
                Statement::Return(ReturnStatement {
                    token: Token::Return,
                    return_value: Expression::Identifier(Identifier {
                        token: Token::Int("20".to_string()),
                        value: "20".to_string(),
                    }),
                }),
            ],
        };

        assert_eq!(program.to_string(), expect);
        Ok(())
    }

    #[test]
    fn format_program_no_statements() -> Result<()> {
        let expect = "";
        let program = Program { statements: vec![] };

        assert_eq!(program.to_string(), expect);
        Ok(())
    }
}
