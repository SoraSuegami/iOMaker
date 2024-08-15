import sympy as sp
import networkx as nx
import numpy as np
import unittest
from src.phfe import create_graph_from_polynomials, graph_to_adjacency_matrix


class TestPolynomialGraph(unittest.TestCase):
    def test_graph_creation_and_adjacency_matrix(self):
        # Define the symbols
        x1, x2, x3 = sp.symbols("x1 x2 x3")

        # Example polynomials for testing
        p1 = 3 * x1 * x2 + 5 * x2 * x3 + 7
        p2 = 2 * x1 * x3 + 4 * x2 + 6

        # Create the graph from the polynomials
        graph, end_nodes = create_graph_from_polynomials(p1, p2)

        # Convert the graph to an adjacency matrix
        adj_matrix = graph_to_adjacency_matrix(graph, end_nodes)

        # Define the expected adjacency matrix (this is an example, adjust based on actual output)
        x1 = sp.symbols("x1")
        x2 = sp.symbols("x2")
        x3 = sp.symbols("x3")
        expected_matrix = np.array(
            [
                [0, 2, 0, 3, 0, 5, 4, 0, 7, 6],
                [0, 0, 0, 0, 0, 0, 0, x1, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, x2, 0],
                [0, 0, x1, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, x3, 0],
                [0, 0, 0, 0, x2, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, x2],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, x3],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            ]
        )

        # Test the shape of the adjacency matrix
        self.assertEqual(adj_matrix.shape, (len(graph.nodes), len(graph.nodes)))

        # Test the content of the adjacency matrix
        self.assertTrue(np.array_equal(adj_matrix, expected_matrix))


if __name__ == "__main__":
    unittest.main()
