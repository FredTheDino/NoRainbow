from itertools import product
from collections import defaultdict

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

def avg(xs):
    return prod(xs) ** (1/len(xs))

def count(r, n):
    c = 0
    a = 0
    for thing in product(range(1, r + 1), repeat=n - r):
        if is_sorted(thing) and thing:
            c += 1
            a += avg(thing)

    return a / c

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

for r in [7]:
    for n in range(r + 1, 50):
        print(f"r={r}, n={n}: {count(r, n)}")


# |states| = S(n, r) -- This is the maximum number of states visited
