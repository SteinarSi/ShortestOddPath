def reformat_normal(edges, output):
    """Reformat graphs. If the edges are on the form (id, from, to [, weight]),
        it rewrites them to (from, to [, weight]), with n and m at the top."""
    n = 0
    lines = []
    with open(edges, "r") as e:
        for edge in e.readlines():
            match edge.split():
                case [_, u, v, w]:
                    lines.append(f'{u} {v} {w}\n')
                    n = max(n, int(u), int(v))
                case [_, u, v]:
                    lines.append(f'{u} {v}\n')
                    n = max(n, int(u), int(v))
                case [u, v]:
                    lines.append(f'{u} {v}\n')
                    n = max(n, int(u), int(v))
                case x:
                    print(f"Error parsing this row: {x}")
    with open(output, "w") as out:
        out.write(f'{n+1} {len(lines)}\n')
        out.writelines(lines)

def reformat_planar(vertices, edges, output):
    with open(output, "w") as out:
        with open(vertices, "r") as v:
            vs = v.readlines()
        with open(edges, "r") as e:
            es = e.readlines()
        out.write(f"{len(vs)} {len(es)}\n")
        for line in vs:
            out.write(line)
        for _, u, v, w in map(lambda l: l.split(), es):
            out.write(f'{u} {v} {w}\n')

# name = "soc-pokec-relationships"
# reformat_normal(
#     f"real_graphs/{name}.edges",
#     f"real_graphs/{name}.in"
# )

# name = "CaliforniaRoadNetwork"
# reformat_planar(
#     f"planar_graphs/real_planar_graphs/{name}/nodes",
#     f"planar_graphs/real_planar_graphs/{name}/edges",
#     f"planar_graphs/real_planar_graphs/{name}/{name}.in"
# )
