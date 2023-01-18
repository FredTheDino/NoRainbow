from collections import defaultdict
from itertools import chain, product

def visit_all_in_category(non_zero_colors, c, seen, i, is_new=True):
    """
        Visits all nodes in the same category as the `c` exactly once. When a node is visited it is recorded in `seen`.

        `non_zero_colors` contains the set of colors that are required to be used at least once for the coloring to be surjective.
        `c` is the coloring (here passed in as a string)
        `seen` is the seen colorings as well as how often they've been seen
    """
    global length
    assert len(c) == length, f"Got: {c} expected {length} long string but this is {len(c)}"
    assert is_eq(non_zero_colors, c), f"This node isn't EQ, {c}"

    if is_new:
        seen[c] += 1

    if i == 1: return

    visit_all_in_category(non_zero_colors, c, seen, i - 1, False)

    if c[i] in non_zero_colors: return

    for other in non_zero_colors:
        d = c[:i] + other + c[i+1:]

        # Different category check
        if not is_same_category(d, c, non_zero_colors): continue

        visit_all_in_category(non_zero_colors, d, seen, i - 1, True)

def find_top(alphabet, length, zero_color="a"):
    """
        Generates all top-colorings, all colorings with the alphabet (i.e. ["a", "b", "c"]) of length `length` that are surjective. Runs in polynomial time if the alphabet is fixed.

        `alphabet` contains the set of colors that are required to be used at least once for the coloring to be surjective.
        `length` is the number of nodes, or the length of the final string
        `zero_color` is the default color to pad with
    """
    if len(alphabet) == 1:
        yield alphabet[0] + zero_color * length
    else:
        for i in range(0, length + 1):
            prefix = alphabet[0] + zero_color * i
            yield from [prefix + suffix for suffix in find_top(alphabet[1:], length - i)]

def is_same_category(a, b, non_zero_colors):
    return all(d.find(k) == c.find(k) for k in non_zero_colors)

def is_increasing_and_positive(l):
    return all(a < b and a >= 0 and b >= 0 for (a, b) in zip(l, l[1:]))

def to_category(alphabet, c):
    return [c.find(k) for k in alphabet]

def is_eq(alphabet, c):
    return is_increasing_and_positive(to_category(alphabet, c))

alphabet = ["a", "b", "c", "d", "e"]
for length in [6]:
    seen = defaultdict(int)
    top_nodes = 0
    for top in find_top(alphabet, length - len(alphabet)):
        assert is_eq(alphabet, top)
        top_nodes += 1
        visit_all_in_category(alphabet[1:], top, seen, length - 1)

    print(f"{length}: {len(seen)} nodes, {sum(seen.values())} visits, {top_nodes} top nodes, {sum(seen.values()) / len(seen)}")

    assert all(map(lambda x: is_eq(alphabet, x), seen.keys()))

    visited = set(seen.keys())
    assert len(list(product(alphabet, repeat=length))) == len(alphabet) ** length

    unique = set(filter(lambda c: is_eq(alphabet, c), map(lambda x: "".join(x), product(alphabet, repeat=length))))
    assert all(map(lambda x: is_eq(alphabet, x), unique))

    assert unique == visited, f"The sets differ: extra = {visited - unique}, not visited = {unique - visited}"
    print(unique)

    # assert len(seen) == sum(seen.values()), f"Didn't visit each node exactly once! length={length}"

# print(seen)
# print(f"Visited {len(seen)} nodes")
# print(f"Max: {max(seen.values())}, Total visits: {sum(seen.values())}")
# for (s, i) in seen.items():
#     if i != 1:
#         print(s, i)
