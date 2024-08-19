import sympy as sp
import networkx as nx
import queue

# import numpy as np


def partial_garbling_polys(*polys):
    graph = build_graph_from_polynomials(*polys)
    adj_matrix = graph_to_adjacency_matrix(graph)
    lx = build_lx_matrix(adj_matrix)
    dfx_coeffs = build_dfx_coeffs(lx, len(polys))
    lx_bar = lx[: -len(polys), :].transpose()
    out = {}
    out["dfx_coeffs"] = dfx_coeffs
    out["lx_bar"] = lx_bar
    return out


def build_graph_from_polynomials(*polys):
    # Initialize the graph
    graph = nx.DiGraph()
    start_node = 0
    graph.add_node(start_node, label="Start")

    # Initialize the node_of_mono map
    # end_nodes = []

    for i, poly in enumerate(polys):
        node_of_mono = {}
        # Create a unique end node for each polynomial
        end_node = 10000 + i
        # end_nodes.append(end_node)
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
            cur_node = start_node

            # Process edge_vals in reverse order
            for idx, edge_val in enumerate(edge_vals):
                if idx == len(edge_vals) - 1:
                    found = False
                    for edge in graph.out_edges(cur_node, data=True):
                        if edge[1] == end_node and edge[2]["weight"] == edge_val:
                            cur_node = end_node
                            found = True
                            break
                    if not found:
                        graph.add_edge(cur_node, end_node, weight=edge_val)
                        cur_node = end_node
                    break

                for edge in graph.out_edges(cur_node, data=True):
                    if edge[2]["weight"] == edge_val:
                        cur_node = edge[1]
                        break

                children_node_vars = tuple(edge_vals[idx + 1 :])
                next_node = None
                if children_node_vars in node_of_mono:
                    next_node = node_of_mono[children_node_vars]
                else:
                    next_node = len(graph.nodes)
                    graph.add_node(next_node, label="Middle")
                    node_of_mono[children_node_vars] = next_node
                graph.add_edge(cur_node, next_node, weight=edge_val)
                cur_node = next_node

                # next_node = None
                # children_node_vars = tuple(edge_vals[idx + 1 :])
                # if idx == len(edge_vals) - 1:
                #     next_node = end_node
                # elif children_node_vars in node_of_mono:
                #     next_node = node_of_mono[children_node_vars]
                # else:
                #     next_node = len(graph.nodes)
                #     graph.add_node(next_node, label="Middle")
                #     node_of_mono[children_node_vars] = next_node

                # if idx == len(edge_vals) - 1:
                #     found = False
                #     for edge in graph.in_edges(end_node, data=True):
                #         if edge[2]["weight"] == edge_val:
                #             cur_node = edge[1]
                #             found = True
                #             break
                #     if not found:
                #         graph.add_edge(start_node, cur_node, weight=edge_val)
                #         cur_node = start_node
                #     break

                # # Create parent_node_vars
                # parent_node_vars = tuple(edge_vals[0 : len(edge_vals) - 1 - idx])

                # if parent_node_vars in node_of_mono:
                #     parent_node = node_of_mono[parent_node_vars]
                #     graph.add_edge(parent_node, cur_node, weight=edge_val)
                #     cur_node = parent_node
                # else:
                #     parent_node = len(graph.nodes)  # Create a new node
                #     graph.add_node(parent_node, label="Middle")
                #     graph.add_edge(parent_node, cur_node, weight=edge_val)
                #     cur_node = parent_node
                #     node_of_mono[parent_node_vars] = parent_node

    return graph


def graph_to_adjacency_matrix(graph):
    num_nodes = len(graph.nodes)
    # Initialize the adjacency matrix with Zero
    adj_matrix = sp.Matrix(num_nodes, num_nodes, lambda i, j: sp.S.Zero)

    node_list = sorted(list(graph.nodes))

    # Iterate over the edges and populate the adjacency matrix
    for u, v, data in graph.edges(data=True):
        i = node_list.index(u)
        j = node_list.index(v)
        adj_matrix[i, j] = data["weight"]

    return adj_matrix


def build_lx_matrix(adj_matrix):
    idx_matrix = sp.Matrix(
        adj_matrix.shape[0],
        adj_matrix.shape[1],
        lambda i, j: sp.S.One if i == j else sp.S.Zero,
    )
    lx = adj_matrix - idx_matrix
    lx.col_del(0)
    return lx


def build_dfx_coeffs(lx, num_polys):
    num_rows = lx.rows
    last_column = lx.cols
    coeffs = []
    # sp.pprint(lx)
    for i in range(num_rows):
        sub_matrix = lx.copy()
        sub_matrix.row_del(i)
        # sp.pprint(sub_matrix)
        det = sub_matrix.det()
        sign = sp.Integer((-1) ** ((i + 1) + (last_column + 1)))
        coeffs.append(sign * det)

    return coeffs[-num_polys:] + coeffs[:-num_polys]
