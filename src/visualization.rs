use super::ast::{Expression, Statement, AST};
use super::graph::{Digraph, Edge, Node};

pub fn function_calls_graph(path: &str, ast: &AST) -> Digraph {
    let mut graph = Digraph::new(path);

    for statement in &ast.statements {
        let Statement::FunctionDefinition(function) = statement;
        graph.nodes.push(Node {
            id: function.prototype.name.to_owned(),
            label: function.prototype.name.to_owned(),
        });

        function_calls_extract_from_expression(
            &function.body,
            &function.prototype.name,
            &mut graph,
        );
    }

    graph
}

fn function_calls_extract_from_expression(
    expression: &Expression,
    parent_function: &str,
    graph: &mut Digraph,
) {
    match expression {
        Expression::FunctionCall(call) => {
            graph.edges.push(Edge {
                source: parent_function.to_owned(),
                destination: call.name.to_owned(),
            });
            for parameter in &call.parameters {
                function_calls_extract_from_expression(parameter, parent_function, graph);
            }
        }
        Expression::Branch(branch) => {
            function_calls_extract_from_expression(&*branch.condition, parent_function, graph);
            function_calls_extract_from_expression(&*branch.then, parent_function, graph);
            function_calls_extract_from_expression(&*branch.r#else, parent_function, graph);
        }
        _ => {}
    }
}
