import networkx as nx                 # pip install networkx scipy
import matplotlib.pyplot as plt       # pip install matplotlib
import os

def visualize(directory, level=0):
    items = os.listdir(directory)
    for item in items:
        p = os.path.join(directory, item)
        if 'planar' in p:
            continue
        elif os.path.isdir(p):
            print(" " * level + item + ": ")
            visualize(p, level + 2)
        elif os.path.isfile(p) and (p.endswith('.in') or p.endswith('.mtx')):
            try:
                print(" " * level + item + ":")
                g = read_graph(p)
                if g is None:
                    print(" " * (level+2) + "Graph to large, skipping.")
                else:
                    print(" " * (level+2) + "Plotting....")
                    plot_graph(g, p[:-3] + '.png')
                    print(" " * (level+2) + "Done!")
            except IOError:
                print(f"Could not read '{p}' :-(")

def read_graph(path):
    with open(path, 'r') as f:
        lines = [line for line in f.readlines() if not line.startswith("%")]
        n = int(lines[0])
        if n >= 100:
            return None
        g = nx.Graph()
        for uv in ([int(u) for u in uv.split()] for uv in lines[1:]):
            if len(uv) == 2:
                u, v = uv
                g.add_edge(u, v)
            elif len(uv) == 3:
                u, v, w = uv
                g.add_edge(u, v, weight=w)
            else:
                print(uv)
                raise IOError("Could not read the graph :-(")
        return g

def plot_graph(g: nx.Graph, filename=None):
    pos = nx.spring_layout(g, seed=69)
    nx.draw(g, pos=pos, with_labels=True)
    nx.draw_networkx_edge_labels(g, pos=pos, edge_labels=nx.get_edge_attributes(g, 'weight'))
    if filename: plt.savefig(filename)
    else: plt.show()
    plt.clf()

visualize(".")
