def reformat(vertices, edges, output):
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

name = "CaliforniaRoadNetwork"
reformat(
    f"planar_graphs/real_planar_graphs/{name}/nodes",
    f"planar_graphs/real_planar_graphs/{name}/edges",
    f"planar_graphs/real_planar_graphs/{name}/{name}.in"
)
