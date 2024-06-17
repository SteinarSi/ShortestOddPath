import networkx as nx                 # pip install networkx scipy
import matplotlib.pyplot as plt       # pip install matplotlib

def read(x):
    a = float(x)
    return round(a) if round(a) == a else a

def plot_graph(g: nx.Graph, pos, node_sizes, edge_colors, widths, filename=None):
    nx.draw(g,
        pos=pos,
        node_size=node_sizes,
        edge_color=edge_colors,
        width=widths,
        with_labels=False
    )
    if filename: plt.savefig(filename)
    plt.show()
    plt.clf()

def visualize_diversion(folder, file):
    with open(f"{folder}/{file}/{file}.in", "r") as f:
        lines = [[read(u) for u in line.split()] for line in f.readlines()]
        pos = {}
        n = lines[0][0]
        for i, x, y in lines[1:n+1]:
            pos[i] = (x, y)
        all_edges = lines[n+1:]

    with open(f"{folder}/{file}/{file}.diverted", "r") as f:
        diversion_set = set()
        for u, v in ([read(u) for u in line.split()] for line in f.readlines()):
            diversion_set.add((u, v))
            diversion_set.add((v, u))

    with open(f"{folder}/{file}/{file}.diversion", "r") as f:
        s, t, d1, d2 = [int(i) for i in f.read().split()][:4]
        cut = [(d1, d2), (d2, d1)]

    full_graph = nx.Graph()
    diverted_graph = nx.Graph()
    for edge in all_edges:
        u = edge[0]
        v = edge[1]
        full_graph.add_edge(u, v)
        if (u, v) not in diversion_set:
            diverted_graph.add_edge(u, v)
    node_sizes = [ 100 if u == s or u == t else 0 for u in full_graph.nodes ]
    just_diversion_edge_colors = [ "red" if e in cut else "black" for e in full_graph.edges ]
    widths = [ 4 if e in cut else 1 for e in full_graph.edges ]

    plot_graph(full_graph, pos, node_sizes, just_diversion_edge_colors, widths, f"benches/figures/{file}-Full.svg")

    diversion_set_edge_colors = [ "red" if e in cut else "orange" if e in diversion_set else "black" for e in full_graph.edges]
    widths = [ 4 if e in cut or e in diversion_set else 1 for e in full_graph.edges ]

    plot_graph(full_graph, pos, node_sizes, diversion_set_edge_colors, widths, f"benches/figures/{file}-Partial.svg")

    widths = [ 4 if e in cut else 1 for e in diverted_graph.edges ]
    diverted_edge_colors = [ "red" if e in cut else "black" for e in diverted_graph.edges ]
    plot_graph(diverted_graph, pos, node_sizes, diverted_edge_colors, widths, f"benches/figures/{file}-Diverted.svg")

visualize_diversion("data/delaunay_graphs/planar_delaunay_graphs", "delaunay35")
