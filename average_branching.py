import random
from itertools import product, permutations

def spy(x):
    print(x)
    return x

def get_eqs(cs, alpha):
    for per in permutations(alpha, len(alpha)):
        mapping = { a: b for a, b in zip(per, alpha) }
        yield tuple(map(lambda c: mapping[c], cs))

def to_eq(cs, alpha):
    return min(get_eqs(cs, alpha))

def is_eq(cs, alpha):
    return to_eq(cs, alpha) == cs

def get_cat(cs):
    seen = set()
    cat = ""
    for c in min(get_eqs(cs, alpha)):
        if c in seen:
            cat += 'a'
        else:
            cat += c
            seen.add(c)
    return tuple(cat)

def is_same_cat(a, b):
    return get_cat(a) == get_cat(b)

# def is_same_cat(a, b):
#     global alpha
#     seen = 0
#     cat = ""
#     for x, y in zip(a, b):
#         if (x == alpha[seen]) == (y == alpha[seen]):
#             seen += (x == alpha[seen])
#             if seen == len(alpha):
#                 break
#         else:
#             return False
#     return True



def random_branching_factor(alpha, n=10):
    node = to_eq(tuple(random.choice(alpha) for _ in range(n)), alpha)
    options = [i for i, c in enumerate(node) if c == 'a'][1:]

    new = list(node)
    try:
        i = random.choice(options)
    except:
        return random_branching_factor(alpha, n)

    children = 0
    for c in alpha[1:]:
        new[i] = c
        children += is_same_cat(node, new)
    return children

def avg(xs):
    return sum(xs) / len(xs)

alpha = list("abc")
num = 2000

for n in range(5, 500):
    print(f"r={len(alpha)} n={n} est: {avg([random_branching_factor(alpha, n) for _ in range(1, num)])}")




