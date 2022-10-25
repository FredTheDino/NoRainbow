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

adj = dict()
for a in product(alpha, repeat=n):
    a = to_eqv(a)
    if is_surr(a) and a not in adj:
        adj[a] = set(to_eqv(p) for p in product(alpha, repeat=len(a)) if one_change(a, p) and is_surr(p))

# print("strict graph {")
# print("    rankdir=LR")

byNeighborCount = defaultdict(set)
lines = set()
for (k, vs) in adj.items():
    k = to_eqv(k)
    if k in lines:
        continue;
    lines.add(k)
    byNeighborCount[len(vs)].add(k)
    # print("    ", "".join(k), "[color = ", ["black", "red", "green", "blue", "yellow", "cyan", "magenta"][len(vs) % 6], ", label=\"", "".join(k), len(vs),"\"]")
    for v in vs:
        if (k, v) in lines or (v, k) in lines :
            continue
        lines.add((k, v))
        # print("    ", "".join(k), "--", "".join(v))

for i, v in enumerate(byNeighborCount.values()):
    print(i, len(v))

# for v in byNeighborCount.values():
#     print("    {rank = same;", "; ".join(map(lambda x: "".join(x), v)), "}")
# print("}")


