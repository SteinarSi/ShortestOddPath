'''
Reads a graph, and formats it nicely.
All vertex labels are converted into id's from 0 to n-1, the integer n is found, and duplicate edges are removed.
'''

prefix = "data/real_graphs/"
path = prefix + input(f"Filename: {prefix}")

id = {}
graph = {}
start = []
def lookup(u):
    if u in id:
        return id[u]
    else:
        id[u] = len(id)
        return id[u]


with open(path, "r") as f:
    for line in f.readlines():
        line = line.strip()
        if line.strip().startswith('%') or line.startswith('#'):
            start.append(line)
        else:
            line = line.replace(',', ' ').split(' ')
            if len(line) <= 1: continue
            line = list(map(lookup, line))
            match line:
                case [u, v]:
                    graph.setdefault(u, set()).add((v, None))
                    graph.setdefault(v, set()).add((u, None))
                case [u, v, w]:
                    graph.setdefault(u, set()).add((v, w))
                    graph.setdefault(v, set()).add((u, w))
                case _:
                    raise Exception(f"Could not parse this line: '{line}'")

with open(path, "w") as f:
    for s in start:
        f.write(s + "\n")
    f.write(f"{len(id)}\n")
    for u in sorted(graph.keys()):
        for v, w in graph[u]:
            if v <= u: continue
            if w is None: f.write(f"{u} {v}\n")
            else: f.write(f"{u} {v} {w}\n")
