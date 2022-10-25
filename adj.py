from itertools import product 
from collections import defaultdict

def count_neighbors(alpha_size, n):
    alpha = list(range(alpha_size))
    assert len(alpha) == alpha_size

    def one_change(a, b):
        changes = 0
        for (ai, bi) in zip(a, b):
            if ai != bi:
                changes += 1
                if changes > 1:
                    return False
        return changes != 0

    def to_eqv(a):
        mapping = dict()
        q = 0
        for c in a:
            if c not in mapping:
                mapping[c] = alpha[q]
                q += 1
        return tuple(mapping[c] for c in a)

    def is_surr(a):
        return len(set(alpha)) == len(set(a))

    minDegree = float("inf")
    maxDegree = 0
    seen = set()
    for a in product(alpha, repeat=n):
        if not is_surr(a) or to_eqv(a) in seen:
            continue
        seen.add(to_eqv(a))

        neighbors = set(to_eqv(p) for p in product(alpha, repeat=len(a)) if one_change(a, p) == 1 and is_surr(p))
        degree = len(neighbors)
        minDegree = min(degree, minDegree)
        maxDegree = max(degree, maxDegree)
    return minDegree, maxDegree

# alpha_size = 4
# n = 5

for alpha_size in range(2, 9):
    for n in range(alpha_size + 1, alpha_size + 4):
        minDegree, maxDegree = count_neighbors(alpha_size, n)
        print(f"|alpha|: {alpha_size}, n: {n:3}, <{minDegree:4}, {maxDegree:4}>")

