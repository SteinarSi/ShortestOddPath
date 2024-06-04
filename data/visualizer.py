import networkx as nx                 # pip install networkx scipy
import matplotlib.pyplot as plt       # pip install matplotlib
import os

def visualize(directory, level=0):
    items = os.listdir(directory)
    for item in items:
        p = os.path.join(directory, item)
        if os.path.isdir(p):
            print(" " * level + item + ": ")
            visualize(p, level + 2)
        elif os.path.isfile(p) and (p.endswith('.in') or p.endswith('.mtx')):
            try:
                print(" " * level + item + ":")
                g, pos = read_graph(p)
                if g is None:
                    print(" " * (level+2) + "Graph too large, skipping.")
                else:
                    print(" " * (level+2) + "Plotting....")
                    plot_graph(g, pos, p[:-3] + '.png')
                    print(" " * (level+2) + "Done!")
            except IOError:
                print(f"{" " * (level+2)}Could not read '{p}' :-(")

def read_graph(path):
    with open(path, 'r') as f:
        lines = [[read(u) for u in line.split()] for line in f.readlines() if not line.startswith("%")]
        n = lines[0][0]
        if n >= 100:
            return None, None
        if 'planar' in path:
            pos = {}
            for i, x, y in lines[1:n+1]:
                pos[i] = (x, y)
            edges = lines[n+1:]
        else:
            edges = lines[1:]
            pos = None
        g = nx.Graph()
        for uv in edges:
            if len(uv) == 2:
                u, v = uv
                g.add_edge(u, v)
            elif len(uv) == 3:
                u, v, w = uv
                g.add_edge(u, v, weight=w)
            else:
                print(uv)
                raise IOError("Could not read the graph :-(")
        return g, pos

def read(x):
    a = float(x)
    return round(a) if round(a) == a else a

def plot_graph(g: nx.Graph, pos=None, filename=None):
    pos = pos if pos is not None else nx.spring_layout(g, seed=69)
    nx.draw(g, pos=pos, with_labels=True)
    nx.draw_networkx_edge_labels(g, pos=pos, edge_labels=nx.get_edge_attributes(g, 'weight'))
    if filename: plt.savefig(filename)
    else: plt.show()
    plt.clf()

visualize(".")