import json
from subprocess import run, PIPE

output = run("cargo criterion --bench bench_odd_path --message-format=json", shell=True, stdout=PIPE).stdout.decode('utf-8').splitlines()

sizes = []
times = []
units = set()
for out in output:
    if out.startswith('{'):
        dct = json.loads(out)
        if dct['reason'] == 'benchmark-complete' and 'delaunay' in dct['id']:
            i = dct['id'].index('/delaunay') + len('/delaunay')
            values = dct['measured_values']
            time = dct['mean']['estimate']
            sizes.append(int(dct['id'][i:]))
            times.append(time)
            units.add(dct['unit'])

print(f"sizes = {sizes}")
print(f"times = {times}")
print(f"units = {units}")
