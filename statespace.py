from graph import *
from itertools import product

def isSurrjective(r, coloring):
    return r == len(set(coloring.values()))

def isSolution(r, graph, coloring):
    return isSurrjective(r, coloring) and isNoRainbowColoring(r, graph, coloring)

def toSimplestColoring(colorTuple):
    mapping = dict()
    for c in enumerate(colorTuple):
        if c in mapping:
            continue
        mapping[c] = len(mapping)
    return tuple(mapping[c] for c in colorTuple)

def overviewStateSpace(graph):
    n = getN(graph)
    r = getR(graph)
    numStates = r ** n

    nodesAsList = list(nodes(graph))
    states = 0
    surrjectiveStates = 0
    solutions = 0
    seen = set()
    for c in product(range(r), repeat=n):
        c = toSimplestColoring(c)
        if c in seen:
            continue
        seen.add(c)

        coloring = dict(map(lambda x: (nodesAsList[x[0]], x[1]), enumerate(c)))
        states += 1

        if not isSurrjective(r, coloring):
            continue
        surrjectiveStates += 1

        if not isSolution(r, graph, coloring):
            continue
        print(f"#{solutions}")
        visualizeColoring(graph, coloring)
        solutions += 1
    print("states", states, "surr", surrjectiveStates, "sol", solutions)


# {frozenset({'f', 'b', 'g'}), frozenset({'d', 'b', 'c'}), frozenset({'b', 'c', 'a'}), frozenset({'c', 'a', 'g'})} - states 28 surr 10 sol 10
# g = randomGraph(3, 4, ["a", "b", "c", "d", "e", "f", "g"])
g = {frozenset({'f', 'b', 'g'}), frozenset({'d', 'b', 'c'}), frozenset({'b', 'c', 'a'}), frozenset({'c', 'a', 'g'})}
print(g)
overviewStateSpace(g)

