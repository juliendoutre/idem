use std::error::Error;

use super::ast::{
    Branch, Expression, FunctionCall, FunctionDefinition, FunctionPrototype, Literal, Number,
    Statement, Variable, VariableDefinition, AST,
};
use super::lexing::{LocatedToken, Location, Symbol, Token};

pub struct Parser<'a, I: Iterator<Item = &'a LocatedToken>> {
    tokens: &'a mut I,
    current_token: Option<&'a LocatedToken>,
}

#[derive(Debug)]
pub struct SyntaxError {
    location: Location,
    issue: String,
}

impl std::fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}:{}:{} {}",
            self.location.path, self.location.line, self.location.column, self.issue
        )
    }
}

impl Error for SyntaxError {}

impl<'a, I: Iterator<Item = &'a LocatedToken>> Parser<'a, I> {
    pub fn new(tokens: &'a mut I) -> Self {
        Self {
            current_token: tokens.next(),
            tokens: tokens,
        }
    }

    pub fn parse(&mut self) -> Result<AST, Box<dyn Error>> {
        let mut ast = AST {
            statements: Vec::new(),
        };

        while self.current_token.is_some() {
            ast.statements.push(self.parse_statement()?);
        }

        Ok(ast)
    }

    fn parse_statement(&mut self) -> Result<Statement, Box<dyn Error>> {
        Ok(Statement::FunctionDefinition(
            self.parse_function_definition()?,
        ))
    }

    fn parse_function_definition(&mut self) -> Result<FunctionDefinition, Box<dyn Error>> {
        Ok(FunctionDefinition {
            location: self.current_token.unwrap().location.clone(),
            prototype: self.parse_function_prototype()?,
            body: self.parse_function_body()?,
        })
    }

    fn parse_function_prototype(&mut self) -> Result<FunctionPrototype, Box<dyn Error>> {
        if let Token::Word(name) = &self.current_token.unwrap().token {
            self.next_token();

            if let Token::Symbol(Symbol::OpeningParenthesis) = self.current_token.unwrap().token {
                self.next_token();

                let mut arguments = Vec::<VariableDefinition>::new();

                loop {
                    match self.current_token.unwrap().token {
                        Token::Symbol(Symbol::ClosingParenthesis) => {
                            self.next_token();
                            break;
                        }
                        Token::Word(_) => {
                            arguments.push(self.parse_variable_definition()?);
                            if let Token::Symbol(Symbol::Comma) = self.current_token.unwrap().token
                            {
                                self.next_token();
                            }
                        }
                        _ => {
                            return Err(Box::new(SyntaxError {
                                issue: "expected a closing parenthesis or a word".to_owned(),
                                location: self.current_token.unwrap().location.clone(),
                            }))
                        }
                    }
                }

                Ok(FunctionPrototype {
                    name: name.clone(),
                    arguments,
                    location: self.current_token.unwrap().location.clone(),
                })
            } else {
                Err(Box::new(SyntaxError {
                    issue: "expected an opening parenthesis".to_owned(),
                    location: self.current_token.unwrap().location.clone(),
                }))
            }
        } else {
            Err(Box::new(SyntaxError {
                issue: "expected a word".to_owned(),
                location: self.current_token.unwrap().location.clone(),
            }))
        }
    }

    fn parse_variable_definition(&mut self) -> Result<VariableDefinition, Box<dyn Error>> {
        if let Token::Word(name) = &self.current_token.unwrap().token {
            self.next_token();
            Ok(VariableDefinition {
                name: name.to_owned(),
                location: self.current_token.unwrap().location.clone(),
            })
        } else {
            Err(Box::new(SyntaxError {
                issue: "expected a word".to_owned(),
                location: self.current_token.unwrap().location.clone(),
            }))
        }
    }

    fn parse_function_body(&mut self) -> Result<Expression, Box<dyn Error>> {
        if let Token::Symbol(Symbol::OpeningBrace) = self.current_token.unwrap().token {
            self.next_token();

            if let Token::Symbol(Symbol::ClosingBrace) = self.current_token.unwrap().token {
                self.next_token();
                Ok(Expression::Empty)
            } else {
                let expression = self.parse_expression();

                if let Token::Symbol(Symbol::ClosingBrace) = self.current_token.unwrap().token {
                    self.next_token();
                    expression
                } else {
                    Err(Box::new(SyntaxError {
                        issue: "expected a closing brace".to_owned(),
                        location: self.current_token.unwrap().location.clone(),
                    }))
                }
            }
        } else {
            Err(Box::new(SyntaxError {
                issue: "expected an opening brace".to_owned(),
                location: self.current_token.unwrap().location.clone(),
            }))
        }
    }

    fn parse_expression(&mut self) -> Result<Expression, Box<dyn Error>> {
        let expression_location = self.current_token.unwrap().location.clone();
        if let Token::Word(word) = &self.current_token.unwrap().token {
            if word == "if" {
                let condition_location = self.current_token.unwrap().location.clone();
                self.next_token();

                let condition = self.parse_expression()?;

                if let Token::Symbol(Symbol::OpeningBrace) = self.current_token.unwrap().token {
                    self.next_token();

                    let then = self.parse_expression()?;

                    if let Token::Symbol(Symbol::ClosingBrace) = self.current_token.unwrap().token {
                        self.next_token();

                        if let Token::Word(word) = &self.current_token.unwrap().token {
                            if word == "else" {
                                self.next_token();

                                if let Token::Symbol(Symbol::OpeningBrace) =
                                    self.current_token.unwrap().token
                                {
                                    self.next_token();

                                    let r#else = self.parse_expression()?;

                                    if let Token::Symbol(Symbol::ClosingBrace) =
                                        self.current_token.unwrap().token
                                    {
                                        self.next_token();

                                        Ok(Expression::Branch(Branch {
                                            condition: Box::new(condition),
                                            then: Box::new(then),
                                            r#else: Box::new(r#else),
                                            location: condition_location,
                                        }))
                                    } else {
                                        Err(Box::new(SyntaxError {
                                            issue: "expected a closing brace".to_owned(),
                                            location: self.current_token.unwrap().location.clone(),
                                        }))
                                    }
                                } else {
                                    Err(Box::new(SyntaxError {
                                        issue: "expected an opening brace".to_owned(),
                                        location: self.current_token.unwrap().location.clone(),
                                    }))
                                }
                            } else {
                                Ok(Expression::Branch(Branch {
                                    condition: Box::new(condition),
                                    then: Box::new(then),
                                    r#else: Box::new(Expression::Empty),
                                    location: condition_location,
                                }))
                            }
                        } else {
                            Ok(Expression::Branch(Branch {
                                condition: Box::new(condition),
                                then: Box::new(then),
                                r#else: Box::new(Expression::Empty),
                                location: condition_location,
                            }))
                        }
                    } else {
                        Err(Box::new(SyntaxError {
                            issue: "expected a closing brace".to_owned(),
                            location: self.current_token.unwrap().location.clone(),
                        }))
                    }
                } else {
                    Err(Box::new(SyntaxError {
                        issue: "expected an opening brace".to_owned(),
                        location: self.current_token.unwrap().location.clone(),
                    }))
                }
            } else {
                self.next_token();
                if let Token::Symbol(Symbol::OpeningParenthesis) = self.current_token.unwrap().token
                {
                    let mut parameters = Vec::<Expression>::new();

                    self.next_token();

                    loop {
                        match self.current_token.unwrap().token {
                            Token::Symbol(Symbol::ClosingParenthesis) => {
                                self.next_token();
                                break;
                            }
                            _ => {
                                parameters.push(self.parse_expression()?);
                                if let Token::Symbol(Symbol::Comma) =
                                    self.current_token.unwrap().token
                                {
                                    self.next_token();
                                }
                            }
                        }
                    }

                    Ok(Expression::FunctionCall(FunctionCall {
                        name: word.clone(),
                        parameters: parameters,
                        location: expression_location,
                    }))
                } else {
                    if let Ok(number) = word.parse::<u32>() {
                        Ok(Expression::Literal(Literal::Number(Number {
                            value: number,
                            location: self.current_token.unwrap().location.clone(),
                        })))
                    } else {
                        Ok(Expression::Variable(Variable {
                            name: word.clone(),
                            location: self.current_token.unwrap().location.clone(),
                        }))
                    }
                }
            }
        } else {
            Err(Box::new(SyntaxError {
                issue: "expected a word".to_owned(),
                location: self.current_token.unwrap().location.clone(),
            }))
        }
    }

    fn next_token(&mut self) {
        self.current_token = self.tokens.next();
    }
}
