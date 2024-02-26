import networkx as nx                 # pip install networkx scipy
import matplotlib.pyplot as plt       # pip install matplotlib
import os

def visualize(directory):
    items = os.listdir(directory)
    for item in items:
        p = os.path.join(directory, item)
        if os.path.isdir(p):
            visualize(p)
        elif os.path.isfile(p) and p.endswith('.in'):
            try:
                g = read_graph(p)
                plot_graph(g, p[:-3] + '.png')
            except IOError:
                print(f"Could not read '{p}' :-(")

def read_graph(path):
    with open(path, 'r') as f:
        _ = int(f.readline())
        g = nx.Graph()
        for uv in ([int(u) for u in uv.split()] for uv in f.readlines() if not uv.startswith('%')):
            if len(uv) == 2:
                u, v = uv
                g.add_edge(u, v)
            elif len(uv) == 3:
                u, v, w = uv
                g.add_edge(u, v, weight=w)
            else:
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
