from itertools import (combinations, chain)
from random import sample

def detNRC(graph):
    r = getR(graph)
    n = getN(graph)
    g = (r - 1) * n / r
    for pair in candidatePair(graph, r):
        if coloring := detLocalSearch(r, graph, pair, g):
            return coloring
    return None

def candidatePair(graph, r):
    for picked in combinations(nodes(graph), r):
        frozen = set(picked)
        for n in range(0, r):
            coloring = dict(map(lambda t: (t[1], (t[0] + n) % r), enumerate(list(picked))))
            yield (coloring, frozen)

def detLocalSearch(r, graph, x, g):
    (coloring, frozen) = x
    # There's something I don't understand here?
    rainbowEdges = findRainbowEdges(r, graph, coloring)
    if g == 0 and len(rainbowEdges) != 0: return None
    if any(edge.isSubSet(frozen) for edge in rainbowEdges): return None
    if isNoRainbowColoring(r, graph, coloring): return coloring

    for edge in edges(graph):
        if len(edge.intersection(frozen)) != r - 1: continue
        # Everything is color 0 if it doesn't exist
        v = next(edge - frozen)
        newColoring = coloring.copy()
        newFrozen = frozen.add(v)
        for c in range(1, r + 1):
            newColoring[v] = c
            if coloring := detLocalSearch(r, graph, (newColoring, newFrozen), g - 1):
                return coloring
        return None
    return None

def isNoRainbowColoring(r, graph, coloring):
    return all(not isRainbowEdge(r, edge, coloring) for edge in edges(graph))

def isRainbowEdge(r, edge, coloring):
    return set(getColor(v, coloring) for v in edge) == r

def findRainbowEdges(r, graph, coloring):
    yield from (edge for edge in edges(graph) if isRainbowEdge(r, edge, coloring))

def getR(graph):
    r = len(next(edges(graph)))
    assert all(len(edge) == r for edge in edges(graph)), f"The graph isn't {r}-regular"
    return r

def getN(graph):
    return len(nodes(graph))

def getColor(node, coloring):
    return coloring.get(node) or 0

def nodes(graph):
    return set(chain(*edges(graph)))

def edges(graph):
    return iter(graph)

def randomGraph(r, numEdges, nodes):
    return set(frozenset(sample(nodes, r)) for n in range(0, numEdges))

def visualizeColoring(graph, coloring):
    sortedNodes = sorted(nodes(graph))
    sortedEdges = sorted(edges(graph), key=lambda e: tuple(sorted(sortedNodes.index(v) for v in e)))
    for n in sortedNodes:
        print(f"{n} ", end="")
        for edge in sortedEdges:
            if n in edge:
                print(f"*", end="")
            else:
                print(f" ", end="")
        if n in coloring:
            print(f" {getColor(n, coloring)}", end="")
        print("")

# graph = set([ frozenset(["a", "b", "c"])
#         , frozenset(["b", "d", "e"])
#         , frozenset(["a", "c", "f"])
#         , frozenset(["a", "c", "f"])
#         ])
g = randomGraph(3, 1000, ["a", "b", "c", "d"])

coloring = detNRC(g)
visualizeColoring(g, coloring)

