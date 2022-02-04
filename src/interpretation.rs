use std::collections::HashMap;
use std::error::Error;

use opentelemetry::global;
use opentelemetry::trace::{Span, Tracer};

use super::ast::{Expression, FunctionDefinition, Literal, Statement, AST};
use super::native::{add, and, equal, multiply, not, or, print, sub, xor};

pub struct Interpreter {}

impl Interpreter {
    pub fn run(&mut self, ast: &AST) -> Result<(), Box<dyn Error>> {
        let mut functions = HashMap::new();
        for statement in &ast.statements {
            let Statement::FunctionDefinition(function) = statement;
            functions.insert(function.prototype.name.as_str(), function);
        }

        if let Some(main) = ast.statements.iter().find(|statement| {
            let Statement::FunctionDefinition(function) = statement;
            function.prototype.name == "main"
        }) {
            let Statement::FunctionDefinition(function) = main;
            self.interpretate(&function.body, &mut HashMap::new(), &functions)
                .unwrap();
            Ok(())
        } else {
            panic!("no main function")
        }
    }

    fn interpretate(
        &mut self,
        expression: &Expression,
        variables: &mut HashMap<&str, u32>,
        functions: &HashMap<&str, &FunctionDefinition>,
    ) -> Result<Option<u32>, Box<dyn Error>> {
        match expression {
            Expression::Empty => Ok(None),
            Expression::Literal(literal) => match literal {
                Literal::Number(num) => Ok(Some(num.value)),
            },
            Expression::Variable(var) => Ok(Some(*variables.get(var.name.as_str()).unwrap())),
            Expression::Branch(branch) => {
                if self
                    .interpretate(&*branch.condition, variables, functions)?
                    .unwrap()
                    == 1
                {
                    self.interpretate(&*branch.then, variables, functions)
                } else {
                    self.interpretate(&*branch.r#else, variables, functions)
                }
            }
            Expression::FunctionCall(call) => {
                let tracer = global::tracer("");
                let mut span = tracer.start(call.name.to_owned());

                let result = match call.name.as_str() {
                    "or" => Ok(Some(or(
                        self.interpretate(&call.parameters[0], variables, functions)
                            .unwrap()
                            .unwrap(),
                        self.interpretate(&call.parameters[1], variables, functions)
                            .unwrap()
                            .unwrap(),
                    ))),
                    "and" => Ok(Some(and(
                        self.interpretate(&call.parameters[0], variables, functions)
                            .unwrap()
                            .unwrap(),
                        self.interpretate(&call.parameters[1], variables, functions)
                            .unwrap()
                            .unwrap(),
                    ))),
                    "xor" => Ok(Some(xor(
                        self.interpretate(&call.parameters[0], variables, functions)
                            .unwrap()
                            .unwrap(),
                        self.interpretate(&call.parameters[0], variables, functions)
                            .unwrap()
                            .unwrap(),
                    ))),

                    "not" => Ok(Some(not(self
                        .interpretate(&call.parameters[0], variables, functions)
                        .unwrap()
                        .unwrap()))),
                    "equal" => Ok(Some(equal(
                        self.interpretate(&call.parameters[0], variables, functions)
                            .unwrap()
                            .unwrap(),
                        self.interpretate(&call.parameters[1], variables, functions)
                            .unwrap()
                            .unwrap(),
                    ))),
                    "add" => Ok(Some(add(
                        self.interpretate(&call.parameters[0], variables, functions)
                            .unwrap()
                            .unwrap(),
                        self.interpretate(&call.parameters[1], variables, functions)
                            .unwrap()
                            .unwrap(),
                    ))),
                    "sub" => Ok(Some(sub(
                        self.interpretate(&call.parameters[0], variables, functions)
                            .unwrap()
                            .unwrap(),
                        self.interpretate(&call.parameters[1], variables, functions)
                            .unwrap()
                            .unwrap(),
                    ))),
                    "multiply" => Ok(Some(multiply(
                        self.interpretate(&call.parameters[0], variables, functions)
                            .unwrap()
                            .unwrap(),
                        self.interpretate(&call.parameters[1], variables, functions)
                            .unwrap()
                            .unwrap(),
                    ))),
                    "print" => {
                        print(
                            self.interpretate(&call.parameters[0], variables, functions)
                                .unwrap()
                                .unwrap(),
                        );
                        Ok(None)
                    }
                    _ => {
                        let function = functions.get(call.name.as_str()).unwrap();
                        let mut local_variables = HashMap::new();

                        for i in 0..call.parameters.len() {
                            local_variables.insert(
                                function.prototype.arguments[i].name.as_str(),
                                self.interpretate(&call.parameters[i], variables, functions)
                                    .unwrap()
                                    .unwrap(),
                            );
                        }

                        self.interpretate(&function.body, &mut local_variables, functions)
                    }
                };

                span.end();

                result
            }
        }
    }
}
