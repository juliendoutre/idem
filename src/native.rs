use std::collections::HashMap;

use super::ast::VariableDefinition;
use super::lexing::Location;

pub fn or(a: u32, b: u32) -> u32 {
    a | b
}

pub fn and(a: u32, b: u32) -> u32 {
    a & b
}

pub fn xor(a: u32, b: u32) -> u32 {
    a ^ b
}

pub fn not(a: u32) -> u32 {
    !a
}

pub fn equal(a: u32, b: u32) -> u32 {
    (a == b) as u32
}

pub fn add(a: u32, b: u32) -> u32 {
    a + b
}

pub fn sub(a: u32, b: u32) -> u32 {
    a - b
}

pub fn multiply(a: u32, b: u32) -> u32 {
    a * b
}

pub fn print(a: u32) {
    println!("{}", a)
}

pub fn native_functions_map<'a>() -> HashMap<&'a str, Vec<VariableDefinition>> {
    let mut map = HashMap::<&'a str, Vec<VariableDefinition>>::new();

    map.insert(
        "or",
        vec![
            VariableDefinition {
                name: String::from("a"),
                location: Location {
                    path: String::new(),
                    line: 0,
                    column: 0,
                },
            },
            VariableDefinition {
                name: String::from("b"),
                location: Location {
                    path: String::new(),
                    line: 0,
                    column: 0,
                },
            },
        ],
    );
    map.insert(
        "and",
        vec![
            VariableDefinition {
                name: String::from("a"),
                location: Location {
                    path: String::new(),
                    line: 0,
                    column: 0,
                },
            },
            VariableDefinition {
                name: String::from("b"),
                location: Location {
                    path: String::new(),
                    line: 0,
                    column: 0,
                },
            },
        ],
    );
    map.insert(
        "xor",
        vec![
            VariableDefinition {
                name: String::from("a"),
                location: Location {
                    path: String::new(),
                    line: 0,
                    column: 0,
                },
            },
            VariableDefinition {
                name: String::from("b"),
                location: Location {
                    path: String::new(),
                    line: 0,
                    column: 0,
                },
            },
        ],
    );
    map.insert(
        "not",
        vec![VariableDefinition {
            name: String::from("a"),
            location: Location {
                path: String::new(),
                line: 0,
                column: 0,
            },
        }],
    );
    map.insert(
        "equal",
        vec![
            VariableDefinition {
                name: String::from("a"),
                location: Location {
                    path: String::new(),
                    line: 0,
                    column: 0,
                },
            },
            VariableDefinition {
                name: String::from("b"),
                location: Location {
                    path: String::new(),
                    line: 0,
                    column: 0,
                },
            },
        ],
    );
    map.insert(
        "add",
        vec![
            VariableDefinition {
                name: String::from("a"),
                location: Location {
                    path: String::new(),
                    line: 0,
                    column: 0,
                },
            },
            VariableDefinition {
                name: String::from("b"),
                location: Location {
                    path: String::new(),
                    line: 0,
                    column: 0,
                },
            },
        ],
    );
    map.insert(
        "sub",
        vec![
            VariableDefinition {
                name: String::from("a"),
                location: Location {
                    path: String::new(),
                    line: 0,
                    column: 0,
                },
            },
            VariableDefinition {
                name: String::from("b"),
                location: Location {
                    path: String::new(),
                    line: 0,
                    column: 0,
                },
            },
        ],
    );
    map.insert(
        "multiply",
        vec![
            VariableDefinition {
                name: String::from("a"),
                location: Location {
                    path: String::new(),
                    line: 0,
                    column: 0,
                },
            },
            VariableDefinition {
                name: String::from("b"),
                location: Location {
                    path: String::new(),
                    line: 0,
                    column: 0,
                },
            },
        ],
    );
    map.insert(
        "print",
        vec![VariableDefinition {
            name: String::from("a"),
            location: Location {
                path: String::new(),
                line: 0,
                column: 0,
            },
        }],
    );

    map
}
