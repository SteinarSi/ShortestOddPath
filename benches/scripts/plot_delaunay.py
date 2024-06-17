from math import log2
import matplotlib.pyplot as plt
import os.path
import numpy as np

RUNNING_MEAN_WINDOW = 15

def running_mean(x, N):
    cumsum = np.cumsum(np.insert(x, 0, 0))
    return (cumsum[N:] - cumsum[:-N]) / float(N)

def plot_delaunay_stats(alg_name, sizes, times, outer_constant, inner_constant):
    def mlogn(n):
        m = 3 * n
        return outer_constant * m * log2(inner_constant * n)

    times = [t / 1_000_000 for t in times]
    big_o = [mlogn(n) / 1_000_000 for n in sizes]
    roll_avg = running_mean(times, RUNNING_MEAN_WINDOW)

    folder = f'../figures/delaunay'
    if not os.path.exists(folder):
        os.makedirs(folder)

    plt.plot(sizes, times, '.', label=alg_name.title())
    plt.plot(sizes, big_o, label=f'm • log2(n)')
    plt.plot(sizes[RUNNING_MEAN_WINDOW//2:-RUNNING_MEAN_WINDOW//2+1], roll_avg, label=f'Running mean, window of {RUNNING_MEAN_WINDOW}')
    plt.legend()
    plt.xlabel('Vertices in the input graph')
    plt.ylabel('Median time over 10 runs (ms)')
    plt.title(f'{alg_name.title()} on Delaunay graphs')
    plt.savefig(f'{folder}/{alg_name}.svg')
    plt.show()
