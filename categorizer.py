from itertools import product 
from collections import defaultdict
from string import (ascii_lowercase)


def one_change(a, b):
    changes = 0
    for (ai, bi) in zip(a, b):
        if ai != bi:
            changes += 1
            if changes > 1:
                return False
    return changes != 0

def distance(a, b):
    return sum(abs(ai - bi) for (ai, bi) in zip(a, b))

try:
    n, alpha_size = map(int, input().split())
    alpha = list(ascii_lowercase)[:alpha_size]
except:
    n = 3
    alpha = ["a", "b", "c"]

def to_eqv(a):
    global alpha
    mapping = dict()
    q = 0
    for c in a:
        if c not in mapping:
            mapping[c] = alpha[q]
            q += 1
    return tuple(mapping[c] for c in a)

def is_surr(a):
    return len(set(alpha)) == len(set(a))

seen = set(to_eqv(a) for a in product(alpha, repeat=n) if is_surr(a))

partition = defaultdict(int)
for s in seen:
    cat = defaultdict(int)
    for c in s:
        cat[c] += 1
    partition[tuple(sorted(cat.values()))] += 1

print("strict graph {")
for (k, v) in sorted(partition.items()):
    print(f"    \"{k}\" [label=\"{k} - {v}\"]")
for a in sorted(partition.keys()):
    for b in sorted(partition.keys()):
        if distance(a, b) == 2:
            print(f"    \"{a}\" -- \"{b}\"")
print("}")
