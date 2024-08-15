import sympy as sp
import networkx as nx
import numpy as np


def create_graph_from_polynomials(*polynomials):
    # Initialize the graph
    graph = nx.DiGraph()
    start_node = "Start"
    graph.add_node(start_node, label="Start")

    # Initialize the node_of_mono map
    node_of_mono = {}
    end_nodes = []

    for i, poly in enumerate(polynomials):
        # Create a unique end node for each polynomial
        end_node = f"End_{i+1}"
        end_nodes.append(end_node)
        graph.add_node(end_node, label="End")

        for term in poly.as_ordered_terms():
            # Decompose the term into coefficient and monomials, create edge_vals
            coeff, *monomials = term.as_coeff_mul()
            edge_vals = [coeff]
            if len(monomials[0]) > 0:
                edge_vals += sorted(monomials[0], key=lambda x: int(str(x)[1:]))
            # If the term is a constant term
            if len(edge_vals) == 1:
                graph.add_edge(start_node, end_node, weight=edge_vals[0])
                continue
            cur_node = end_node

            # Process edge_vals in reverse order
            print(edge_vals)
            for idx, edge_val in enumerate(reversed(edge_vals)):
                print(idx)
                if idx == len(edge_vals) - 1:
                    found = False
                    for edge in graph.out_edges(start_node, data=True):
                        if edge[2]["weight"] == edge_val:
                            cur_node = edge[1]
                            found = True
                            break
                    if not found:
                        graph.add_edge(start_node, cur_node, weight=edge_val)
                        cur_node = start_node
                    break

                # Create parent_node_vars
                parent_node_vars = tuple(edge_vals[0 : len(edge_vals) - 1 - idx])

                if parent_node_vars in node_of_mono:
                    parent_node = node_of_mono[parent_node_vars]
                    graph.add_edge(parent_node, cur_node, weight=edge_val)
                    cur_node = parent_node
                else:
                    parent_node = len(graph.nodes)  # Create a new node
                    graph.add_node(parent_node, label="Middle")
                    graph.add_edge(parent_node, cur_node, weight=edge_val)
                    cur_node = parent_node
                    node_of_mono[parent_node_vars] = parent_node

    return graph, end_nodes


def graph_to_adjacency_matrix(graph, end_nodes):
    # Create a mapping from nodes to indices
    node_list = list(graph.nodes)
    num_nodes = len(node_list)
    adj_matrix = np.full(
        (num_nodes, num_nodes), sp.S.Zero
    )  # Initialize the adjacency matrix with Zero

    # Iterate over the edges and populate the adjacency matrix
    for u, v, data in graph.edges(data=True):
        i = node_list.index(u)
        j = node_list.index(v)
        print(data["weight"])
        adj_matrix[i, j] = data["weight"]

    # Adjust end nodes indices
    for i, end_node in enumerate(end_nodes):
        end_idx = node_list.index(end_node)
        target_idx = num_nodes - len(end_nodes) + i
        if end_idx != target_idx:
            adj_matrix[[end_idx, target_idx]] = adj_matrix[[target_idx, end_idx]]
            adj_matrix[:, [end_idx, target_idx]] = adj_matrix[:, [target_idx, end_idx]]
            node_list[end_idx], node_list[target_idx] = (
                node_list[target_idx],
                node_list[end_idx],
            )

    return adj_matrix
