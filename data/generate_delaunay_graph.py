import os.path

from scipy.spatial import Delaunay
import numpy as np
import random as r

SCALE = 1_000_000
def generate_delaunay(filename, n, weighted=False):
    normal_folder = "delaunay_graphs/normal_delaunay_graphs/" + filename
    planar_folder = "delaunay_graphs/planar_delaunay_graphs/" + filename
    if not os.path.exists(normal_folder):
        os.makedirs(normal_folder)
    if not os.path.exists(planar_folder):
        os.makedirs(planar_folder)
    with open(planar_folder + "/" + filename + ".in", "w") as planar_file:
        with open(normal_folder + "/" + filename + ".in", "w") as normal_file:
            used_edges = [[] for _ in range(n)]
            edges = []

            def save_edge(fro, to):
                if to not in used_edges[fro]:
                    used_edges[fro].append(to)
                    used_edges[to].append(fro)
                    edges.append(f"{fro} {to} {r.random() * SCALE if weighted else ""}\n")

            points = SCALE * np.random.rand(n, 2)
            triangles = Delaunay(points).simplices
            for a, b, c in triangles:
                save_edge(a, b)
                save_edge(a, c)
                save_edge(b, c)

            m = len(edges)
            header = f"{n} {m}\n"
            planar_file.write(header)
            normal_file.write(header)

            for i, (x, y) in enumerate(points):
                planar_file.write(f"{i} {x} {y}\n")
            for edge in edges:
                planar_file.write(edge)
                normal_file.write(edge)

# generate_delaunay("delaunay35", 35, False)

# for n in range(1000, 200_001, 1000):
#     generate_delaunay("delaunay" + str(n), n, True)
#     if n % 10_000 == 0:
#         print(f"Done with #{n}.")
