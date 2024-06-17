import json
from subprocess import run, PIPE
from collections import defaultdict

output = (run("cargo criterion --bench bench_network_diversion --message-format=json", shell=True, stdout=PIPE)
          .stdout
          .decode('utf-8')
          .splitlines())

groups = defaultdict(list)
units = set()

for out in output:
    if out.startswith('{'):
        dct = json.loads(out)
        if dct['reason'] == 'benchmark-complete' and 'delaunay' in dct['id']:
            i = dct['id'].index('delaunay') + len('delaunay')
            j = dct['id'].index(',')
            size = int(dct['id'][i:j])
            time = dct['mean']['estimate']
            groups[size].append(time)
            units.add(dct['unit'])

sizes = []
times = []
for size, time in sorted([(size, max(times)) for size, times in groups.items()]):
    sizes.append(size)
    times.append(time)

print(f"sizes = {sizes}")
print(f"times = {times}")
print(f"units = {units}")