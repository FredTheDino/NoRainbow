from itertools import combinations
from graph import *

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
    rainbowEdges = set(findRainbowEdges(r, graph, coloring))
    if g == 0 and len(rainbowEdges) != 0: return None
    if any(edge.issubset(frozen) for edge in rainbowEdges): return None
    if isNoRainbowColoring(r, graph, coloring): return coloring

    for edge in edges(graph):
        if len(edge.intersection(frozen)) != r - 1: continue
        v = next(iter(edge - frozen))
        newColoring = coloring.copy()
        newFrozen = frozen.copy()
        newFrozen.add(v)
        for c in range(1, r + 1):
            newColoring[v] = c
            if coloring := detLocalSearch(r, graph, (newColoring, newFrozen), g - 1):
                return coloring
        return None
    return None

if __name__ == "__main__":
    # graph = set([ frozenset(["a", "b", "c"])
    #         , frozenset(["b", "d", "e"])
    #         , frozenset(["a", "c", "f"])
    #         , frozenset(["a", "c", "f"])
    #         ])
    g = completeGraph(4, list(range(6)))
    stateCounter = 0
    seen = set()
    coloring = detNRC(g)
    # visualizeColoring(g, coloring)
    print(stateCounter)

