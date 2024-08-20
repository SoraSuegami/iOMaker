// use core::num;
// use std::collections::{BTreeMap, HashMap};
// use std::marker::PhantomData;

// use crate::poly::{Monomial, Polynomial, Variable};
// use ark_ff::PrimeField;
// use petgraph::visit::{Bfs, EdgeRef, NodeRef};
// use petgraph::{graph::DiGraph, graph::NodeIndex, Graph};

// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
// pub enum AbpEdge<F: PrimeField> {
//     Coeff(F),
//     Variable(Variable),
// }

// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
// pub enum AbpNodeWeight {
//     Start,
//     End,
//     Middle,
// }

// #[derive(Debug, Clone)]
// pub struct Abp<F: PrimeField> {
//     pub graph: DiGraph<AbpNodeWeight, AbpEdge<F>>,
// }

// impl<F: PrimeField> Abp<F> {
//     pub fn new(poly: &Polynomial<F>) -> Self {
//         let mut graph = DiGraph::<AbpNodeWeight, AbpEdge<F>, _>::new();
//         let start = graph.add_node(AbpNodeWeight::Start);
//         let end = graph.add_node(AbpNodeWeight::End);
//         let mut node_of_vars = HashMap::<Vec<AbpEdge<F>>, NodeIndex>::new();
//         for mono in poly.monomials.iter() {
//             if mono.degree() == 0 {
//                 graph.add_edge(start, end, AbpEdge::Coeff(mono.coefficient));
//                 continue;
//             }
//             let mut cur_node = end.clone();
//             let mut edge_vals = vec![];
//             if mono.degree() == 0 || mono.coefficient != F::one() {
//                 edge_vals.push(AbpEdge::Coeff(mono.coefficient));
//             }
//             for var in mono.variables.iter() {
//                 edge_vals.push(AbpEdge::Variable(*var));
//             }
//             for (idx, edge_val) in edge_vals.iter().enumerate().rev() {
//                 if idx == 0 {
//                     let mut found = false;
//                     for edge in graph.edges_directed(start, petgraph::Direction::Outgoing) {
//                         if edge.weight() == edge_val {
//                             cur_node = edge.target();
//                             found = true;
//                             break;
//                         }
//                     }
//                     if !found {
//                         graph.add_edge(start, cur_node, edge_val.clone());
//                         cur_node = start;
//                     }
//                     break;
//                 }
//                 let parent_node_vars = edge_vals[0..idx].to_vec();
//                 if let Some(node) = node_of_vars.get(&parent_node_vars.clone()) {
//                     graph.add_edge(*node, cur_node, edge_val.clone());
//                     cur_node = *node;
//                     break;
//                 } else {
//                     let parent_node = graph.add_node(AbpNodeWeight::Middle);
//                     graph.add_edge(parent_node, cur_node, edge_val.clone());
//                     cur_node = parent_node;
//                     node_of_vars.insert(parent_node_vars, parent_node);
//                 }
//             }
//         }

//         let mut renamed_graph = DiGraph::new();
//         let mut rename_map = HashMap::<NodeIndex, NodeIndex>::new();
//         let mut bfs = Bfs::new(&graph, start);
//         while let Some(node) = bfs.next(&graph) {
//             if graph[node] == AbpNodeWeight::End {
//                 continue;
//             }
//             let new_node = renamed_graph.add_node(graph[node].clone());
//             rename_map.insert(node, new_node);
//         }
//         let new_last_node = renamed_graph.add_node(AbpNodeWeight::End);
//         rename_map.insert(end, new_last_node);
//         for edge in graph.raw_edges() {
//             let source = edge.source();
//             let target = edge.target();
//             let new_source = *rename_map.get(&source).unwrap();
//             let new_target = *rename_map.get(&target).unwrap();
//             renamed_graph.add_edge(new_source, new_target, edge.weight.clone());
//         }
//         Self {
//             graph: renamed_graph,
//         }
//     }
// }

// #[cfg(test)]
// mod test {
//     use super::*;
//     use ark_bn254::Fr;

//     #[test]
//     fn test_abp_valid_case1() {
//         // f(x_0, x_1, x_2, x_3, x_4, x_5) = x_0x_1x_2x_3 + x_0x_4 + x_0x_1x_6
//         let poly = Polynomial::new(vec![
//             Monomial::new(
//                 vec![Variable(0), Variable(1), Variable(2), Variable(3)],
//                 Fr::from(1),
//                 0,
//             ),
//             Monomial::new(vec![Variable(0), Variable(4)], Fr::from(1), 1),
//             Monomial::new(vec![Variable(0), Variable(1), Variable(5)], Fr::from(1), 2),
//         ]);
//         let abp = Abp::new(&poly);
//         let mut expected_graph = DiGraph::new();
//         let start = expected_graph.add_node(AbpNodeWeight::Start);
//         let mid0 = expected_graph.add_node(AbpNodeWeight::Middle);
//         let mid1 = expected_graph.add_node(AbpNodeWeight::Middle);
//         let mid2 = expected_graph.add_node(AbpNodeWeight::Middle);
//         let end = expected_graph.add_node(AbpNodeWeight::End);
//         expected_graph.add_edge(mid2, end, AbpEdge::Variable(Variable(3)));
//         expected_graph.add_edge(mid1, mid2, AbpEdge::Variable(Variable(2)));
//         expected_graph.add_edge(mid0, mid1, AbpEdge::Variable(Variable(1)));
//         expected_graph.add_edge(start, mid0, AbpEdge::<Fr>::Variable(Variable(0)));
//         expected_graph.add_edge(mid0, end, AbpEdge::Variable(Variable(4)));
//         expected_graph.add_edge(mid1, end, AbpEdge::Variable(Variable(5)));

//         for (node_a, node_b) in abp
//             .graph
//             .raw_nodes()
//             .iter()
//             .zip(expected_graph.raw_nodes().iter())
//         {
//             assert_eq!(node_a.weight, node_b.weight);
//         }
//         for (edge_a, edge_b) in abp
//             .graph
//             .raw_edges()
//             .iter()
//             .zip(expected_graph.raw_edges().iter())
//         {
//             assert_eq!(edge_a.weight, edge_b.weight);
//         }
//     }
// }
