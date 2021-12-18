use std::collections::HashMap;

use super::ast::{Expression, Statement, VariableDefinition, AST};
use super::lexing::Location;
use super::native::native_functions_map;

#[derive(Debug)]
pub struct Report {
    pub issue: String,
    pub location: Location,
}

impl std::fmt::Display for Report {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}:{}:{} {}",
            self.location.path, self.location.line, self.location.column, self.issue
        )
    }
}

pub fn validate(ast: &AST) -> Vec<Report> {
    let mut prototypes: HashMap<&String, (Location, &[VariableDefinition], usize)> = HashMap::new();
    let native_functions: HashMap<&str, Vec<VariableDefinition>> = native_functions_map();

    for statement in &ast.statements {
        let Statement::FunctionDefinition(definition) = statement;
        prototypes.insert(
            &definition.prototype.name,
            (
                definition.location.clone(),
                &definition.prototype.arguments,
                0,
            ),
        );
    }

    let mut reports = Vec::new();

    for statement in &ast.statements {
        let Statement::FunctionDefinition(definition) = statement;
        let mut variables: HashMap<&String, (Location, usize)> = HashMap::new();

        for arg in &definition.prototype.arguments {
            if variables.contains_key(&arg.name) {
                reports.push(Report {
                    location: arg.location.clone(),
                    issue: format!(
                        "several \"{}\"'s arguments have the name \"{}\"",
                        definition.prototype.name, arg.name
                    ),
                });
            } else {
                variables.insert(&arg.name, (arg.location.clone(), 0));
            }
        }

        reports.append(&mut validate_expression(
            &mut prototypes,
            &native_functions,
            &mut variables,
            &definition.body,
        ));

        for (variable, (location, count)) in variables {
            if count == 0 {
                reports.push(Report {
                    location: location.clone(),
                    issue: format!("variable \"{}\" is never used", variable),
                });
            }
        }
    }

    for (function, (location, _, count)) in &prototypes {
        if *function != "main" && *count == 0 {
            reports.push(Report {
                issue: format!("function \"{}\" is never used", function),
                location: location.clone(),
            });
        }
    }

    reports
}

fn validate_expression(
    prototypes: &mut HashMap<&String, (Location, &[VariableDefinition], usize)>,
    native_functions: &HashMap<&str, Vec<VariableDefinition>>,
    variables: &mut HashMap<&String, (Location, usize)>,
    expression: &Expression,
) -> Vec<Report> {
    match expression {
        Expression::FunctionCall(call) => {
            let mut reports = Vec::<Report>::new();

            if let Some((_, arguments, count)) = prototypes.get_mut(&call.name) {
                *count += 1;
                if arguments.len() != call.parameters.len() {
                    reports.push(Report {
                        issue: format!(
                            "function \"{}\" accepts {} arguments but received {} parameters",
                            call.name,
                            arguments.len(),
                            call.parameters.len()
                        ),
                        location: call.location.clone(),
                    });
                }
            } else {
                if let Some(arguments) = native_functions.get(call.name.as_str()) {
                    if arguments.len() != call.parameters.len() {
                        reports.push(Report {
                            issue: format!(
                                "function \"{}\" accepts {} arguments but received {} parameters",
                                call.name,
                                arguments.len(),
                                call.parameters.len()
                            ),
                            location: call.location.clone(),
                        });
                    }
                } else {
                    reports.push(Report {
                        issue: format!("unknwon function \"{}\"", call.name),
                        location: call.location.clone(),
                    })
                }
            }

            for parameter in &call.parameters {
                reports.append(&mut validate_expression(
                    prototypes,
                    native_functions,
                    variables,
                    &parameter,
                ));
            }

            reports
        }
        Expression::Branch(branch) => {
            let mut reports = Vec::new();

            if let Expression::Empty = *branch.condition {
                reports.push(Report {
                    issue: format!("empty branch condition"),
                    location: branch.location.clone(),
                });
            } else {
                reports.append(&mut validate_expression(
                    prototypes,
                    native_functions,
                    variables,
                    &branch.condition,
                ));
            }

            reports.append(&mut validate_expression(
                prototypes,
                native_functions,
                variables,
                &branch.then,
            ));

            reports.append(&mut validate_expression(
                prototypes,
                native_functions,
                variables,
                &branch.r#else,
            ));

            reports
        }
        Expression::Variable(var) => {
            if let Some((_, count)) = variables.get_mut(&var.name) {
                *count += 1;
                vec![]
            } else {
                vec![Report {
                    location: var.location.clone(),
                    issue: format!("unknown variable \"{}\"", var.name),
                }]
            }
        }
        _ => Vec::new(),
    }
}
