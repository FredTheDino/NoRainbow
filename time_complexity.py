from collections import defaultdict
from itertools import chain, product

def is_increasing_and_positive(l):
    return all(a < b and a >= 0 and b >= 0 for (a, b) in zip(l, l[1:]))

def to_category(alphabet, c):
    return [c.find(k) for k in alphabet]

def is_eq(alphabet, c):
    return is_increasing_and_positive(to_category(alphabet, c))

alphabet = ["a", "b", "c", "d"]
for length in [5, 6, 7, 8, 9, 10, 11, 12]:
    all_of_them = list(map(lambda x: "".join(x), product(alphabet, repeat=length)))
    the_eq_ones = list(filter(lambda x: is_eq(alphabet, x), all_of_them))

    print(f"{length}: ALL: {len(all_of_them)}, EQ: {len(the_eq_ones)}, EQ/ALL: {len(the_eq_ones)/len(all_of_them)}")

    # assert unique == visited, f"The sets differ: extra = {visited - unique}, not visited = {unique - visited}"

    # assert len(seen) == sum(seen.values()), f"Didn't visit each node exactly once! length={length}"

# print(seen)
# print(f"Visited {len(seen)} nodes")
# print(f"Max: {max(seen.values())}, Total visits: {sum(seen.values())}")
# for (s, i) in seen.items():
#     if i != 1:
#         print(s, i)
