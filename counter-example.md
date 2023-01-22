# Why the No-Rainbow problem is P - actually

The No-Rainbow problem is formulated as:

> A) Given a $r$-regular hyper-graph, find a coloring that uses all $r$ colors and without any edges having all colors.

I first propose that this formulation is equivalent to:

> B) Given a $r$-regular hyper-graph, find $r - 1$ nodes which do not share an edge.

I will however not show this, I will limit my case to $r=3$ to make this as understandable as possible. Once this proof is understood it is trivial to generalize for any $r$.

$A => B$)
Given a No-Rainbow coloring, we can easily find 2 nodes that differ in color and don't share an edge. We could do this by finding a set of 3 nodes all with different color, which has to exist otherwise the coloring isn't surjective, and all 3 of these colors cannot share an edge since then we have found a No-Rainbow edge, now simply pick 2 colors which don't share an edge of these 3 and color the rest of the nodes the color that wasn't picked.

$B => A$)
Let us say that all pairs of nodes share an edge, but you still have a valid No-Rainbow coloring. Since you have a valid No-Rainbow coloring I can find all 3 unique colors in your solution, but since all pairs nodes share an edge I know these 3 unique colors all share an edge. This contradicts that you have a valid No-Rainbow coloring.

This shows that A and B are equivalent formulations.

\clearpage
# The algorithm

```python
def findNoRainbowColoringWith3Colors(G):
  (V, E) = G

  pairs = Set()
  for e in E:
    for (x, y) in allPairs(e):
      # Iterate through each pair:
      # ab, ba, ac, ca, cb, bc
      # given: a, b, c
      pairs.add((x, y))

  for x in V:
    for y in V - {x}:
      if (x, y) not in pairs:
        return { x: 1, y: 2, otherwise: 0 }
  return None
```
The `otherwise` in the first return statement is to signal that all other nodes have the color 0.

The algorithm starts by putting all pairs of nodes which share an edge into a set (this can be made more efficient by using a table and marking which nodes share an edge, then this marking can be done in constant time). Now we simply loop through all nodes which aren't equal and if we find a pair not present in our pre-computed set we are done (this can be made more efficient by defining a total ordering for all the nodes, then we only have to visit each node which is larger than $x$ and also looking up nodes in constant time using a table.).

## Correctness
It has already been shown that if we can find two nodes which do not share an edge, we have found a No-Rainbow coloring. This algorithm does this in polynomial time for $r=3$.

## Asymptotic Time Complexity
Each step is polynomial in time in the algorithm. The running time is $O(n^2log(n))$ (for the generalized optimized algorithm one gets $O(n^{r-1})$) since each for-loop can maximally be run a polynomial number of times it is trivial to see that the algorithm is limited to a polynomial running time.

## Asymptotic Space Complexity
If the suggested optimizations are used the memory usage is limited to $O(n^2)$ (for the generalized algorithm one gets $O(n^{r-1})$).

