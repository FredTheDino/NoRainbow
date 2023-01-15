from itertools import product, permutations
from collections import defaultdict
import sys


def is_eq_rep(cs):
    return all(cs <= x for x in get_eqs(cs))

def get_eqs(cs):
    global alpha
    for per in permutations(alpha, len(alpha)):
        mapping = { a: b for a, b in zip(per, alpha) }
        yield tuple(map(lambda c: mapping[c], cs))

def get_cat(cs):
    seen = set()
    cat = ""
    for c in min(get_eqs(cs)):
        if c in seen:
            cat += 'a'
        else:
            cat += c
            seen.add(c)
    return tuple(cat)

def is_cat(cs):
    return get_cat(cs) == cs

def pretty(x):
    return "".join(list(x))

def is_surr(x):
    global alpha
    return set(x) == set(alpha)

def debug(c):
    print(f"{pretty(c)} - eq: {is_eq_rep(c)}, eqs: {sorted(list(map(pretty, get_eqs(c))))}, cat: {pretty(get_cat(c))} - {is_surr(c)}")


# debug(tuple("abcabc"))
# debug(tuple("aaabbc"))
# debug(tuple("bbbaac"))
# debug(tuple("aaaaa"))

alpha = list("abc")
n = 5
        
def print_tree_graph():
    global alpha, n

    cats = defaultdict(set)
    for c in product(alpha, repeat=n):
        if not is_eq_rep(c): continue
        cc = get_cat(c)
        if cc == c: continue
        cats[(not is_surr(c), cc)].add(c)

    for ((_, rep), subs) in sorted(cats.items()):
        print("\\begin{forest}")
        if is_surr(rep):
            print(f"  [\\underline{{ {pretty(rep)} }},circle,draw")
        else:
            print(f"  [{pretty(rep)},circle,draw")
        for sub in subs:
            print(f"    [{pretty(sub)}]")
        print("  ]")
        print("\\end{forest}")

print_tree_graph()

def print_dot_graph():
    global alpha, n
    print("graph {")

    def check(x):
        # return is_eq_rep(x)
        return is_surr(x) and is_eq_rep(x)

    for c in product(alpha, repeat=n):
        if not check(c): continue
        label = "*"
        for eq in sorted(get_eqs(c)):
            label += pretty(eq) + "\\n"


        shape = "rect" if is_cat(c) else "ellipse"
        style = "filled" if not is_surr(c) else ""
        color = "red" if not is_surr(c) else ""

        print(f"  {pretty(c)} [color=\"{color}\", style=\"{style}\", shape=\"{shape}\", label=\"{label}\"];")

    for c in product(alpha, repeat=n):
        if not check(c): continue
        # for eq in get_eqs(c):
        #     if eq <= c: continue
        #     print(f"  {pretty(c)} -- {pretty(eq)} [color=green];")
        
        cat = get_cat(c)
        if cat == c or not is_eq_rep(c):
            pass
        else:
            print(f"  {pretty(c)} -- {pretty(cat)} [color=blue];")

    print("} ")


