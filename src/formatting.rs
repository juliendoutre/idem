use super::ast::{
    Branch, Expression, FunctionCall, FunctionDefinition, FunctionPrototype, Literal, Statement,
    Variable, VariableDefinition, AST,
};

pub trait Formattable {
    fn format(&self) -> String;
}

impl Formattable for AST {
    fn format(&self) -> String {
        self.statements
            .iter()
            .map(|statement| statement.format())
            .collect::<Vec<String>>()
            .join("\n\n")
    }
}

impl Formattable for Statement {
    fn format(&self) -> String {
        match self {
            Statement::FunctionDefinition(func) => func.format(),
        }
    }
}

impl Formattable for FunctionDefinition {
    fn format(&self) -> String {
        format!(
            "{} {{\n{}\n}}",
            self.prototype.format(),
            self.body
                .format()
                .lines()
                .map(|line| format!("\t{}", line))
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

impl Formattable for FunctionPrototype {
    fn format(&self) -> String {
        format!(
            "{}({})",
            self.name,
            self.arguments
                .iter()
                .map(|arg| arg.format())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

impl Formattable for VariableDefinition {
    fn format(&self) -> String {
        self.name.clone()
    }
}

impl Formattable for Expression {
    fn format(&self) -> String {
        match self {
            Expression::Literal(literal) => literal.format(),
            Expression::Variable(variable) => variable.format(),
            Expression::Branch(branch) => branch.format(),
            Expression::FunctionCall(call) => call.format(),
            Expression::Empty => String::new(),
        }
    }
}

impl Formattable for Literal {
    fn format(&self) -> String {
        match self {
            Literal::Number(num) => format!("{}", num.value),
        }
    }
}

impl Formattable for Variable {
    fn format(&self) -> String {
        self.name.clone()
    }
}

impl Formattable for FunctionCall {
    fn format(&self) -> String {
        format!(
            "{}({})",
            self.name,
            self.parameters
                .iter()
                .map(|arg| arg.format())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

impl Formattable for Branch {
    fn format(&self) -> String {
        let mut string = format!(
            "if {} {{\n{}\n}}",
            (*self.condition).format(),
            (*self.then)
                .format()
                .lines()
                .map(|line| format!("\t{}", line))
                .collect::<Vec<String>>()
                .join("\n"),
        );

        match *self.r#else {
            Expression::Empty => {}
            _ => {
                string.push_str(&format!(
                    " else {{\n{}\n}}",
                    (*self.r#else)
                        .format()
                        .lines()
                        .map(|line| format!("\t{}", line))
                        .collect::<Vec<String>>()
                        .join("\n")
                ));
            }
        }

        string
    }
}
