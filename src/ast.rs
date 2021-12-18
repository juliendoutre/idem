use super::lexing::Location;

#[derive(Debug)]
pub struct AST {
    pub statements: Vec<Statement>,
}

#[derive(Debug)]
pub enum Statement {
    FunctionDefinition(FunctionDefinition),
}

#[derive(Debug)]
pub struct FunctionDefinition {
    pub prototype: FunctionPrototype,
    pub body: Expression,
    pub location: Location,
}

#[derive(Debug)]
pub struct FunctionPrototype {
    pub name: String,
    pub arguments: Vec<VariableDefinition>,
    pub location: Location,
}

#[derive(Debug)]
pub struct VariableDefinition {
    pub name: String,
    pub location: Location,
}

#[derive(Debug)]
pub enum Expression {
    FunctionCall(FunctionCall),
    Branch(Branch),
    Variable(Variable),
    Literal(Literal),
    Empty,
}

#[derive(Debug)]
pub struct FunctionCall {
    pub name: String,
    pub parameters: Vec<Expression>,
    pub location: Location,
}

#[derive(Debug)]
pub struct Branch {
    pub condition: Box<Expression>,
    pub then: Box<Expression>,
    pub r#else: Box<Expression>,
    pub location: Location,
}

#[derive(Debug)]
pub struct Variable {
    pub name: String,
    pub location: Location,
}

#[derive(Debug)]
pub enum Literal {
    Number(Number),
}

#[derive(Debug)]
pub struct Number {
    pub value: u32,
    pub location: Location,
}
