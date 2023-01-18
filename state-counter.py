from itertools import product, combinations_with_replacement
from collections import defaultdict
from math import factorial

def spy(x):
    print(x)
    return x

def prod(xs):
    i = 1
    for x in xs:
        i *= x
    return i

def is_sorted(xs):
    p = 0
    for x in xs:
        if x < p:
            return False
        p = x
    return True

def tree_size(xs):
    return prod(xs)

def count(r, n):
    return sum(tree_size(thing) for thing in combinations_with_replacement(range(1, r + 1), n - r))

def other(r, n):
    return sum((r - 1) ** ((r - 1) * n/ r) for thing in combinations_with_replacement(range(1, r + 1), n - r))

def count_occ(xs):
    cs = defaultdict(int)
    for x in xs:
        cs[x] += 1
    return cs


#  aaabc: 1 * 1 = 1
#  aabac: 2 * 1 = 2
#  aabca: 3 * 1 = 3
#  abaac: 2 * 2 = 4
#  abaca: 2 * 3 = 6
#  abcaa: 3 * 3 = 9
# 25
# print(count(3, 5))

for r in [20]:
    for n in range(r + 1, 30):
        x = count(r, n)
        y = factorial(n)
        print(f"r={r}, n={n}: {x}/{y} = {x/y:0.3} | {int(y/x)}")


# |states| = S(n, r) -- This is the maximum number of states visited
