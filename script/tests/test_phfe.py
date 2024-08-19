import sympy as sp
import networkx as nx
import numpy as np
import unittest
import random
from src.phfe import (
    build_graph_from_polynomials,
    graph_to_adjacency_matrix,
    partial_garbling_polys,
)


class TestPHFE(unittest.TestCase):
    # def test_graph_creation_and_adjacency_matrix(self):
    #     # Define the symbols
    #     x1, x2, x3 = sp.symbols("x1 x2 x3")

    #     # Example polynomials for testing
    #     p1 = 3 * x1 * x2 + 5 * x2 * x3 + 7
    #     p2 = 2 * x1 * x3 + 4 * x2 + 6

    #     # Create the graph from the polynomials
    #     graph, end_nodes = build_graph_from_polynomials(p1, p2)

    #     # Convert the graph to an adjacency matrix
    #     adj_matrix = graph_to_adjacency_matrix(graph, end_nodes)

    #     # Define the expected adjacency matrix (this is an example, adjust based on actual output)
    #     x1 = sp.symbols("x1")
    #     x2 = sp.symbols("x2")
    #     x3 = sp.symbols("x3")
    #     expected_matrix = np.array(
    #         [
    #             [0, 2, 0, 3, 0, 5, 4, 0, 7, 6],
    #             [0, 0, 0, 0, 0, 0, 0, x1, 0, 0],
    #             [0, 0, 0, 0, 0, 0, 0, 0, x2, 0],
    #             [0, 0, x1, 0, 0, 0, 0, 0, 0, 0],
    #             [0, 0, 0, 0, 0, 0, 0, 0, x3, 0],
    #             [0, 0, 0, 0, x2, 0, 0, 0, 0, 0],
    #             [0, 0, 0, 0, 0, 0, 0, 0, 0, x2],
    #             [0, 0, 0, 0, 0, 0, 0, 0, 0, x3],
    #             [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    #             [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    #         ]
    #     )

    #     # Test the shape of the adjacency matrix
    #     self.assertEqual(adj_matrix.shape, (len(graph.nodes), len(graph.nodes)))

    #     # Test the content of the adjacency matrix
    #     self.assertTrue(np.array_equal(adj_matrix, expected_matrix))

    def test_partial_garbling(self):
        # Define the symbols
        x1, x2, x3, z1, z2 = sp.symbols("x1 x2 x3 z1 z2")

        # Example polynomials for testing
        p1 = 3 * x1 * x2 + 5 * x2 * x3 + 7
        p2 = 2 * x1 * x3 + 4 * x2 + 6

        pgc = partial_garbling_polys(p1, p2)
        # print(pgc)
        lx_bar = pgc["lx_bar"]
        print(lx_bar)
        num_rows = lx_bar.rows
        t = sp.Matrix([[random.randint(0, 100) for _ in range(num_rows)]])
        t_bar = t[:, -2:]
        # print(t)
        # print(t_bar)
        x1_val = sp.Integer(random.randint(0, 1))
        x2_val = sp.Integer(random.randint(0, 1))
        x3_val = sp.Integer(random.randint(0, 1))
        z1_val = sp.Integer(random.randint(0, 1))
        z2_val = sp.Integer(random.randint(0, 1))
        z_vec = sp.Matrix([[z1_val, z2_val]])
        lx_bar_substituted = lx_bar.subs({x1: x1_val, x2: x2_val, x3: x3_val})
        pfx = (z_vec - t_bar).row_join(t * lx_bar_substituted)
        # print(pfx.shape)
        dfx_coeffs = sp.Matrix(pgc["dfx_coeffs"])
        # print(dfx_coeffs.shape)
        dfx = dfx_coeffs.subs({x1: x1_val, x2: x2_val, x3: x3_val})
        # print(dfx)
        out = pfx * dfx
        expected_out = (
            p1.subs({x1: x1_val, x2: x2_val, x3: x3_val}) * z1_val
            + p2.subs({x1: x1_val, x2: x2_val, x3: x3_val}) * z2_val
        )
        print(out)
        print(expected_out)


if __name__ == "__main__":
    unittest.main()
