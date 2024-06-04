import os.path

from scipy.spatial import Delaunay
import numpy as np
import random as r

SCALE = 1000
def generate_delaunay(filename, n, weighted=False):
    normal_folder = "delaunay_graphs/normal_delaunay_graphs/" + filename
    planar_folder = "delaunay_graphs/planar_delaunay_graphs/" + filename
    if not os.path.exists(normal_folder):
        os.makedirs(normal_folder)
    if not os.path.exists(planar_folder):
        os.makedirs(planar_folder)
    with open(planar_folder + "/" + filename + ".in", "w") as planar_file:
        with open(normal_folder + "/" + filename + ".in", "w") as normal_file:
            def write_edge(fro, to):
                row = f"{fro} {to} {r.random() * SCALE if weighted else ""}\n"
                normal_file.write(row)
                planar_file.write(row)

            points = SCALE * np.random.rand(n, 2)
            triangles = Delaunay(points).simplices
            header = f"{n} {len(triangles) * 3}\n"
            planar_file.write(header)
            normal_file.write(header)

            for i, (x, y) in enumerate(points):
                planar_file.write(f"{i} {x} {y}\n")

            for a, b, c in triangles:
                write_edge(a, b)
                write_edge(a, c)
                write_edge(b, c)

for n in range(1000, 100_001, 1000):
    generate_delaunay("delaunay" + str(n), n, True)
