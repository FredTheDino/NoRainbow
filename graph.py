from random import sample
from itertools import chain, combinations

def isNoRainbowColoring(r, graph, coloring):
    return not any(isRainbowEdge(r, edge, coloring) for edge in edges(graph))

def isRainbowEdge(r, edge, coloring):
    return len(set(getColor(v, coloring) for v in edge)) == r

def findRainbowEdges(r, graph, coloring):
    yield from (edge for edge in edges(graph) if isRainbowEdge(r, edge, coloring))

def getR(graph):
    r = len(next(edges(graph)))
    assert all(len(edge) == r for edge in edges(graph)), f"The graph isn't {r}-regular"
    return r

def getN(graph):
    return len(nodes(graph))

def getColor(node, coloring):
    # Everything is color 0 if it doesn't exist
    return coloring.get(node) or 0

def nodes(graph):
    return set(chain(*edges(graph)))

def edges(graph):
    return iter(graph)

def randomGraph(r, numEdges, nodes):
    return set(frozenset(sample(nodes, r)) for n in range(0, numEdges))

def completeGraph(r, nodes):
    return set(frozenset(e) for e in combinations(nodes, r))

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

