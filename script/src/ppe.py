import sympy as sp
import networkx as nx
from sympy.printing import pretty
import argparse
import json
import concurrent.futures
import random
import math


def ppe_to_phfe_polys(
    n,
    k,
    deg,
    delta,
    monos,
    linear_coeffs,
):
    if deg == 0 or deg % 2 != 0:
        raise ValueError("Degree must be even")
    if len(linear_coeffs) != m * k:
        raise ValueError("linear_coeffs must have length m*k")
    m = monos.length
    t_1 = math.ceil(k ** (1 - delta))
    T = math.ceil(k ** (delta / 2))
    t_2 = math.ceil(k ** (delta / 10))

    # public_vars = sp.symbols(f"x0:{n*k}")
    # print(public_vars)
    s = sp.Matrix([sp.symbols(f"s{j}") for j in range(k)])
    a_s = []
    b_s = []
    for i in range(n):
        for j in range(k):
            a_s.append(sp.Matrix([sp.symbols(f"a{j}_{i}_{l}") for l in range(k)]))
            b_s.append(sp.symbols(f"b{j}_{i}"))
    flag = sp.symbols("flag")
    us = []
    vs = []
    for r in range(m):
        for gamma in range(t_1):
            us.append(
                sp.Matrix(T, t_2, lambda i, j: sp.symbols(f"u{r}_{gamma}_{i}_{j}"))
            )
            vs.append(
                sp.Matrix(t_2, T, lambda i, j: sp.symbols(f"v{r}_{gamma}_{i}_{j}"))
            )
    uv_muled = []
    for r in range(m):
        for gamma in range(t_1):
            uv_muled.append(us[t_1 * r + gamma] * vs[t_1 * r + gamma])

    sum = sp.S.Zero
    for r in range(m):
        q = monos[r]
        for j in range(k):
            q_val = sp.S.One
            for q_idx in q:
                q_val *= b_s[j * n + q_idx] - a_s[j * n + q_idx].dot(s)
            j1, j2, j3 = map_j_to_j123(t_1, T, j)
            w = flag * (q_val + uv_muled[t_1 * r + j1][j2, j3])
            sum += linear_coeffs[k * r + j] * w
    return sum


def map_j_to_j123(t_1, T, j):
    j1 = j % t_1
    q = j // t_1
    j2 = q // T
    j3 = q % T
    return j1, j2, j3
