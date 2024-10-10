import sympy as sp
import networkx as nx
import numpy as np
import unittest
import random
import json
from src.ppe import ppe_to_phfe_polys
from sympy.physics.quantum import TensorProduct
from sympy.printing import pretty


class TestPPE(unittest.TestCase):
    def test_ppe1(self):
        n = 3
        k = 2
        d = 2
        # Define the symbols
        x0, x1, x2, x3, x4, x5 = sp.symbols("x0 x1 x2 x3 x4 x5")
        q_1 = [0, 2]
        q_2 = [1, 2]


if __name__ == "__main__":
    unittest.main()
