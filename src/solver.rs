use std::io::{Error, ErrorKind};

use crate::facts::Fact;
use crate::graph::{Graph, NodeIndex};
use crate::print;
use crate::rules::{rule::token::{Operand, Token}, Rules};
use crate::tree_builder;

fn get_plain_solved_queries(queries: Vec<&Fact>) -> Vec<Fact> {
    let mut solved_queries = Vec::new();
    for fact in queries.iter() {
        solved_queries.push(fact.copy());
    }
    solved_queries
}

fn compute(operand: Operand, lhs: bool, rhs: bool) -> bool {
    return match operand {
        Operand::Not    => !lhs,
        Operand::And    => lhs & rhs,
        Operand::Or     => lhs | rhs,
        Operand::Xor    => lhs ^ rhs,
        _               => panic!("Error: () in tree_solver()."),
    }
}

fn tree_solver(graph: &Graph<Token>, cur: NodeIndex) -> Result<bool, Error> {
    match graph.get(cur) {
        Some(node) => {
            let token = node.content;
            if let Some(fact) = token.fact {
                if let Some(node_index) = node.lhs {
                    return tree_solver(graph, node_index)
                }
                return Ok(fact.state.get())
            } else if let Some(op) = token.operand {
                if let Some(lhs) = node.lhs {
                    if op == Operand::Not {
                        return Ok(!tree_solver(graph, lhs)?)
                    } else if let Some(rhs) = node.rhs {
                        return Ok(compute(op, tree_solver(graph, lhs)?, tree_solver(graph, rhs)?));
                    }
                }
            }
            return Err(Error::new(ErrorKind::InvalidData, "Tree solver: empty token"))
        },
        None => panic!("Error: print_tree_rec() out of bounds.")
    }
}

pub fn solve(queries: Vec<&Fact>, rules: Rules) -> Result<Vec<Fact>, Error> {
    for fact in queries.iter() {
        let mut graph: Graph<Token> = Graph::new();
        let root: NodeIndex = graph.add_query(Token::new_fact(&fact));
        graph = tree_builder::generate(graph, fact, root, &rules)?;
        fact.set_solved(tree_solver(&graph, 0)?);
        print::tree_to_file(&graph);
    }
    // checker::solved_queries(&facts)?;
    Ok(get_plain_solved_queries(queries))
}
