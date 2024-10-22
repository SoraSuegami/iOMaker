import sympy as sp
import networkx as nx
import numpy as np
import unittest
import random
import json
from src.phfe import (
    build_graph_from_polynomials,
    graph_to_adjacency_matrix,
    partial_garbling_polys,
    pgb_to_json,
)
from sympy.physics.quantum import TensorProduct
from sympy.printing import pretty


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

    def test_partial_garbling1(self):
        # Define the symbols
        x0, x1, x2 = sp.symbols("x0 x1 x2")

        # Example polynomials for testing
        p0 = x1 + x2 + 1
        p1 = x0 * x1 + x1 * x2
        p2 = x0 * x2 + x1 + x2 + 1
        p3 = x0 * x1 + x1 * x2
        p4 = x0 * x2 + x1 + x0
        p5 = x0 * x1 + x1 * x2 + 1

        pgb = partial_garbling_polys(3, 2, 3, p0, p1, p2, p3, p4, p5)
        # print(pgc)
        # print(pretty(pgc, use_unicode=False))
        # with open("test_phfe_output.json", "w") as f:
        #     json.dump(pgb_to_json(pgb), f, ensure_ascii=False, indent=4)
        l0 = pgb["l0"]
        l1 = pgb["l1"]
        # lx_bar  =
        x0_val = sp.Integer(random.randint(0, 1))
        x1_val = sp.Integer(random.randint(0, 1))
        x2_val = sp.Integer(random.randint(0, 1))
        z0_val = sp.Integer(random.randint(0, 1))
        z1_val = sp.Integer(random.randint(0, 1))
        z2_val = sp.Integer(random.randint(0, 1))
        z3_val = sp.Integer(random.randint(0, 1))
        z4_val = sp.Integer(random.randint(0, 1))
        xs = sp.Matrix([x0_val, x1_val, x2_val])
        # print(xs.shape)
        # print(sp.eye(l0.cols).shape)
        lx_bar = l1 * TensorProduct(xs, sp.eye(l0.cols)) + l0
        t = sp.Matrix([[random.randint(0, 100) for _ in range(l0.rows)]])
        t_bar = t[:, -2 * 3 :]
        # print(t)
        # print(t_bar)

        z1s = sp.Matrix([[z0_val, z1_val]])
        z2s = sp.Matrix([[z2_val, z3_val, z4_val]])
        zs = TensorProduct(z1s, z2s)
        print(zs)
        # lx_bar_substituted = lx_bar.subs({x1: x1_val, x2: x2_val, x3: x3_val})
        # print("zs-t_bar", (zs - t_bar).shape)
        pfx = (zs - t_bar).row_join(t * lx_bar)
        # print(pfx.shape)
        dfx_coeffs = sp.Matrix(pgb["dfx_coeffs"])
        # print(dfx_coeffs.shape)
        dfx = dfx_coeffs.subs({x0: x0_val, x1: x1_val, x2: x2_val})
        # print(dfx)
        out = pfx * dfx
        expected_out = (
            p0.subs({x0: x0_val, x1: x1_val, x2: x2_val}) * zs[0]
            + p1.subs({x0: x0_val, x1: x1_val, x2: x2_val}) * zs[1]
            + p2.subs({x0: x0_val, x1: x1_val, x2: x2_val}) * zs[2]
            + p3.subs({x0: x0_val, x1: x1_val, x2: x2_val}) * zs[3]
            + p4.subs({x0: x0_val, x1: x1_val, x2: x2_val}) * zs[4]
            + p5.subs({x0: x0_val, x1: x1_val, x2: x2_val}) * zs[5]
        )
        self.assertEqual(out[0, 0], expected_out)

    def test_partial_garbling2(self):
        # Define the symbols
        x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12, x13 = sp.symbols(
            "x0 x1 x2 x3 x4 x5 x6 x7 x8 x9 x10 x11 x12 x13"
        )

        # Example polynomials for testing
        # "x0*x1*x2*x3*x4 + x5 + x6",
        # "x1*x2*x3*x4*x5 + x6 + x7",
        # "x0 + x1 + x2*x3*x4*x5*x6",
        # "x3*x4*x5*x6*x7 + x8 + x9",
        # "x0 + x2 + x4*x5*x6*x7*x8",
        # "x1 + x3 + x5*x6*x7*x8*x9",
        # "x10*x6*x7*x8*x9 + x2 + x4",
        # "x10*x11*x7*x8*x9 + x3 + x5",
        # "x0 + x10*x11*x12*x8*x9 + x6",
        # "x1 + x10*x11*x12*x13*x9 + x7"
        p0 = x0 * x1 * x2 * x3 * x4 + x5 + x6
        p1 = x1 * x2 * x3 * x4 * x5 + x6 + x7
        p2 = x0 + x1 + x2 * x3 * x4 * x5 * x6
        p3 = x3 * x4 * x5 * x6 * x7 + x8 + x9
        p4 = x0 + x2 + x4 * x5 * x6 * x7 * x8
        p5 = x1 + x3 + x5 * x6 * x7 * x8 * x9
        p6 = x10 * x6 * x7 * x8 * x9 + x2 + x4
        p7 = x10 * x11 * x7 * x8 * x9 + x3 + x5
        p8 = x0 + x10 * x11 * x12 * x8 * x9 + x6
        p9 = x1 + x10 * x11 * x12 * x13 * x9 + x7

        pgb = partial_garbling_polys(14, 2, 5, p0, p1, p2, p3, p4, p5, p6, p7, p8, p9)
        # print(pgc)
        # print(pretty(pgc, use_unicode=False))
        # with open("test_phfe_output.json", "w") as f:
        #     json.dump(pgb_to_json(pgb), f, ensure_ascii=False, indent=4)
        l0 = pgb["l0"]
        l1 = pgb["l1"]
        # lx_bar  =
        x0_val = sp.Integer(random.randint(0, 1))
        x1_val = sp.Integer(random.randint(0, 1))
        x2_val = sp.Integer(random.randint(0, 1))
        x3_val = sp.Integer(random.randint(0, 1))
        x4_val = sp.Integer(random.randint(0, 1))
        x5_val = sp.Integer(random.randint(0, 1))
        x6_val = sp.Integer(random.randint(0, 1))
        x7_val = sp.Integer(random.randint(0, 1))
        x8_val = sp.Integer(random.randint(0, 1))
        x9_val = sp.Integer(random.randint(0, 1))
        x10_val = sp.Integer(random.randint(0, 1))
        x11_val = sp.Integer(random.randint(0, 1))
        x12_val = sp.Integer(random.randint(0, 1))
        x13_val = sp.Integer(random.randint(0, 1))
        z0_val = sp.Integer(random.randint(0, 1))
        z1_val = sp.Integer(random.randint(0, 1))
        z2_val = sp.Integer(random.randint(0, 1))
        z3_val = sp.Integer(random.randint(0, 1))
        z4_val = sp.Integer(random.randint(0, 1))
        z5_val = sp.Integer(random.randint(0, 1))
        z6_val = sp.Integer(random.randint(0, 1))
        xs = sp.Matrix(
            [
                x0_val,
                x1_val,
                x2_val,
                x3_val,
                x4_val,
                x5_val,
                x6_val,
                x7_val,
                x8_val,
                x9_val,
                x10_val,
                x11_val,
                x12_val,
                x13_val,
            ]
        )
        # print(xs.shape)
        # print(sp.eye(l0.cols).shape)
        lx_bar = l1 * TensorProduct(xs, sp.eye(l0.cols)) + l0
        t = sp.Matrix([[random.randint(0, 100) for _ in range(l0.rows)]])
        t_bar = t[:, -2 * 5 :]
        # print(t)
        # print(t_bar)

        z1s = sp.Matrix([[z0_val, z1_val]])
        z2s = sp.Matrix([[z2_val, z3_val, z4_val, z5_val, z6_val]])
        zs = TensorProduct(z1s, z2s)
        print(zs)
        # lx_bar_substituted = lx_bar.subs({x1: x1_val, x2: x2_val, x3: x3_val})
        # print("zs-t_bar", (zs - t_bar).shape)
        pfx = (zs - t_bar).row_join(t * lx_bar)
        # print(pfx.shape)
        dfx_coeffs = sp.Matrix(pgb["dfx_coeffs"])
        # print(dfx_coeffs.shape)
        dfx = dfx_coeffs.subs(
            {
                x0: x0_val,
                x1: x1_val,
                x2: x2_val,
                x3: x3_val,
                x4: x4_val,
                x5: x5_val,
                x6: x6_val,
                x7: x7_val,
                x8: x8_val,
                x9: x9_val,
                x10: x10_val,
                x11: x11_val,
                x12: x12_val,
                x13: x13_val,
            }
        )
        # print(dfx)
        out = pfx * dfx
        expected_out = (
            p0.subs(
                {
                    x0: x0_val,
                    x1: x1_val,
                    x2: x2_val,
                    x3: x3_val,
                    x4: x4_val,
                    x5: x5_val,
                    x6: x6_val,
                    x7: x7_val,
                    x8: x8_val,
                    x9: x9_val,
                    x10: x10_val,
                    x11: x11_val,
                    x12: x12_val,
                    x13: x13_val,
                }
            )
            * zs[0]
            + p1.subs(
                {
                    x0: x0_val,
                    x1: x1_val,
                    x2: x2_val,
                    x3: x3_val,
                    x4: x4_val,
                    x5: x5_val,
                    x6: x6_val,
                    x7: x7_val,
                    x8: x8_val,
                    x9: x9_val,
                    x10: x10_val,
                    x11: x11_val,
                    x12: x12_val,
                    x13: x13_val,
                }
            )
            * zs[1]
            + p2.subs(
                {
                    x0: x0_val,
                    x1: x1_val,
                    x2: x2_val,
                    x3: x3_val,
                    x4: x4_val,
                    x5: x5_val,
                    x6: x6_val,
                    x7: x7_val,
                    x8: x8_val,
                    x9: x9_val,
                    x10: x10_val,
                    x11: x11_val,
                    x12: x12_val,
                    x13: x13_val,
                }
            )
            * zs[2]
            + p3.subs(
                {
                    x0: x0_val,
                    x1: x1_val,
                    x2: x2_val,
                    x3: x3_val,
                    x4: x4_val,
                    x5: x5_val,
                    x6: x6_val,
                    x7: x7_val,
                    x8: x8_val,
                    x9: x9_val,
                    x10: x10_val,
                    x11: x11_val,
                    x12: x12_val,
                    x13: x13_val,
                }
            )
            * zs[3]
            + p4.subs(
                {
                    x0: x0_val,
                    x1: x1_val,
                    x2: x2_val,
                    x3: x3_val,
                    x4: x4_val,
                    x5: x5_val,
                    x6: x6_val,
                    x7: x7_val,
                    x8: x8_val,
                    x9: x9_val,
                    x10: x10_val,
                    x11: x11_val,
                    x12: x12_val,
                    x13: x13_val,
                }
            )
            * zs[4]
            + p5.subs(
                {
                    x0: x0_val,
                    x1: x1_val,
                    x2: x2_val,
                    x3: x3_val,
                    x4: x4_val,
                    x5: x5_val,
                    x6: x6_val,
                    x7: x7_val,
                    x8: x8_val,
                    x9: x9_val,
                    x10: x10_val,
                    x11: x11_val,
                    x12: x12_val,
                    x13: x13_val,
                }
            )
            * zs[5]
            + p6.subs(
                {
                    x0: x0_val,
                    x1: x1_val,
                    x2: x2_val,
                    x3: x3_val,
                    x4: x4_val,
                    x5: x5_val,
                    x6: x6_val,
                    x7: x7_val,
                    x8: x8_val,
                    x9: x9_val,
                    x10: x10_val,
                    x11: x11_val,
                    x12: x12_val,
                    x13: x13_val,
                }
            )
            * zs[6]
            + p7.subs(
                {
                    x0: x0_val,
                    x1: x1_val,
                    x2: x2_val,
                    x3: x3_val,
                    x4: x4_val,
                    x5: x5_val,
                    x6: x6_val,
                    x7: x7_val,
                    x8: x8_val,
                    x9: x9_val,
                    x10: x10_val,
                    x11: x11_val,
                    x12: x12_val,
                    x13: x13_val,
                }
            )
            * zs[7]
            + p8.subs(
                {
                    x0: x0_val,
                    x1: x1_val,
                    x2: x2_val,
                    x3: x3_val,
                    x4: x4_val,
                    x5: x5_val,
                    x6: x6_val,
                    x7: x7_val,
                    x8: x8_val,
                    x9: x9_val,
                    x10: x10_val,
                    x11: x11_val,
                    x12: x12_val,
                    x13: x13_val,
                }
            )
            * zs[8]
            + p9.subs(
                {
                    x0: x0_val,
                    x1: x1_val,
                    x2: x2_val,
                    x3: x3_val,
                    x4: x4_val,
                    x5: x5_val,
                    x6: x6_val,
                    x7: x7_val,
                    x8: x8_val,
                    x9: x9_val,
                    x10: x10_val,
                    x11: x11_val,
                    x12: x12_val,
                    x13: x13_val,
                }
            )
            * zs[9]
        )
        self.assertEqual(out[0, 0], expected_out)


if __name__ == "__main__":
    unittest.main()
