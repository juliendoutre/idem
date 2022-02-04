use std::collections::HashMap;
use std::error::Error;

use opentelemetry::trace::Tracer;
use opentelemetry::{global, Context};

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
            self.interpretate(
                &function.body,
                &mut HashMap::new(),
                &functions,
                &Context::current(),
            )
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
        ctx: &Context,
    ) -> Result<Option<u32>, Box<dyn Error>> {
        match expression {
            Expression::Empty => Ok(None),
            Expression::Literal(literal) => match literal {
                Literal::Number(num) => Ok(Some(num.value)),
            },
            Expression::Variable(var) => Ok(Some(*variables.get(var.name.as_str()).unwrap())),
            Expression::Branch(branch) => {
                if self
                    .interpretate(
                        &*branch.condition,
                        variables,
                        functions,
                        &Context::current(),
                    )?
                    .unwrap()
                    == 1
                {
                    self.interpretate(&*branch.then, variables, functions, ctx)
                } else {
                    self.interpretate(&*branch.r#else, variables, functions, ctx)
                }
            }
            Expression::FunctionCall(call) => {
                let tracer = global::tracer("");
                let span = tracer.start_with_context(call.name.to_owned(), ctx);

                tracer.with_span(span, |ctx| match call.name.as_str() {
                    "or" => Ok(Some(or(
                        self.interpretate(&call.parameters[0], variables, functions, &ctx)
                            .unwrap()
                            .unwrap(),
                        self.interpretate(&call.parameters[1], variables, functions, &ctx)
                            .unwrap()
                            .unwrap(),
                    ))),
                    "and" => Ok(Some(and(
                        self.interpretate(&call.parameters[0], variables, functions, &ctx)
                            .unwrap()
                            .unwrap(),
                        self.interpretate(&call.parameters[1], variables, functions, &ctx)
                            .unwrap()
                            .unwrap(),
                    ))),
                    "xor" => Ok(Some(xor(
                        self.interpretate(&call.parameters[0], variables, functions, &ctx)
                            .unwrap()
                            .unwrap(),
                        self.interpretate(&call.parameters[0], variables, functions, &ctx)
                            .unwrap()
                            .unwrap(),
                    ))),

                    "not" => Ok(Some(not(self
                        .interpretate(&call.parameters[0], variables, functions, &ctx)
                        .unwrap()
                        .unwrap()))),
                    "equal" => Ok(Some(equal(
                        self.interpretate(&call.parameters[0], variables, functions, &ctx)
                            .unwrap()
                            .unwrap(),
                        self.interpretate(&call.parameters[1], variables, functions, &ctx)
                            .unwrap()
                            .unwrap(),
                    ))),
                    "add" => Ok(Some(add(
                        self.interpretate(&call.parameters[0], variables, functions, &ctx)
                            .unwrap()
                            .unwrap(),
                        self.interpretate(&call.parameters[1], variables, functions, &ctx)
                            .unwrap()
                            .unwrap(),
                    ))),
                    "sub" => Ok(Some(sub(
                        self.interpretate(&call.parameters[0], variables, functions, &ctx)
                            .unwrap()
                            .unwrap(),
                        self.interpretate(&call.parameters[1], variables, functions, &ctx)
                            .unwrap()
                            .unwrap(),
                    ))),
                    "multiply" => Ok(Some(multiply(
                        self.interpretate(&call.parameters[0], variables, functions, &ctx)
                            .unwrap()
                            .unwrap(),
                        self.interpretate(&call.parameters[1], variables, functions, &ctx)
                            .unwrap()
                            .unwrap(),
                    ))),
                    "print" => {
                        print(
                            self.interpretate(&call.parameters[0], variables, functions, &ctx)
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
                                self.interpretate(&call.parameters[i], variables, functions, &ctx)
                                    .unwrap()
                                    .unwrap(),
                            );
                        }

                        self.interpretate(&function.body, &mut local_variables, functions, &ctx)
                    }
                })
            }
        }
    }
}
