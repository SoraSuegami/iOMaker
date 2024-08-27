import sympy as sp
import networkx as nx
from sympy.printing import pretty


def partial_garbling_polys(
    num_public_vars, num_private_vars1, num_private_vars2, *polys
):
    graph = build_graph_from_polynomials(*polys)
    adj_matrix = graph_to_adjacency_matrix(graph)
    lx = build_lx_matrix(adj_matrix)
    dfx_coeffs = build_dfx_coeffs(lx, len(polys))
    l0, l1 = build_l0_and_l1(lx, len(polys), num_public_vars)
    if num_private_vars1 * num_private_vars2 != len(polys):
        raise ValueError("Invalid number of polynomials")
    out = {}
    out["num_public_vars"] = num_public_vars
    out["num_private_vars1"] = num_private_vars1
    out["num_private_vars2"] = num_private_vars2
    out["polys"] = polys
    out["dfx_coeffs"] = dfx_coeffs
    out["l0"] = l0
    out["l1"] = l1
    return out


def pgb_to_json(pgb):
    pgb_json = {}
    pgb_json["num_public_vars"] = pgb["num_public_vars"]
    pgb_json["num_private_vars1"] = pgb["num_private_vars1"]
    pgb_json["num_private_vars2"] = pgb["num_private_vars2"]
    pgb_json["polys"] = [pretty(poly, use_unicode=False) for poly in pgb["polys"]]
    pgb_json["dfx_coeffs"] = [
        pretty(poly, use_unicode=False) for poly in pgb["dfx_coeffs"]
    ]
    pgb_json["l0"] = [
        [pretty(x, use_unicode=False) for x in row] for row in pgb["l0"].tolist()
    ]
    pgb_json["l1"] = [
        [pretty(x, use_unicode=False) for x in row] for row in pgb["l1"].tolist()
    ]
    return pgb_json


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


def build_l0_and_l1(lx, num_polys, num_pub_vars):
    lx_bar = lx[:-num_polys, :].transpose()
    l0 = sp.Matrix(lx_bar.rows, lx_bar.cols, lambda i, j: sp.S.Zero)
    l1 = sp.Matrix(lx_bar.rows, lx_bar.cols * num_pub_vars, lambda i, j: sp.S.Zero)
    for i in range(lx_bar.rows):
        for j in range(lx_bar.cols):
            val = lx_bar[i, j]
            coeff, *monomials = val.as_coeff_mul()
            if len(monomials[0]) == 0:
                l0[i, j] = coeff
                continue
            var = str(monomials[0][0])
            if var.startswith("x"):
                k = int(var[1:])
                l1[i, k * lx_bar.cols + j] = coeff
            else:
                raise ValueError("Invalid value in lx_bar")
    # print("l0", l0)
    # print("l1", l1)
    # print("lx_bar", lx_bar)
    return l0, l1
