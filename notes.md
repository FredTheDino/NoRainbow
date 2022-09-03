# No Rainbow
Based on "Exact Algorithms for No-Raindow Coloring and Phylogenetic Decisiveness" by Ghazaleh Parvini and David Ferna´ndez-Baca

## What is "Phylogenetic Decisiveness"?
Has the special case `r=4` and No-Rainbow is the complement of this problem.
I have no idea what this is.

## What is the No-Rainbow Problemm?
We have a hypergraph (fancy word for saying the edges connect more than 2 nodes).
The hypergraph is `r`-regular (all edges have `r` elements in them, note `r > 2`).
We want to color this graph with `r`-distinct colors.
We try to color the graph (assign a color to each vertex) such that no edge contains all `r` colors.
The coloring should be surrjective (we also want to use  all `r` colors).

This problem is NP-complete and can be mapped to SAT. (When `r >= 3`)

## Some ideas for algorithms
 - If we can split the graph into two components such that no edge carries between the components, we can "bucket-fill" these halves. In my head it makes sense.
 - Use the invariant `each color must be used at least once in the graph` (The paper uses this approach both for the deterministic and the randomized algorithm)
 - Reason about the different mappings between colors and maybe divide and conquer the search space somehow?
 - Checking if two colorings are isomorphic can be done in linear time, so it might be a really good speed up
 - Pruning the search space better

### Invariants of the problem, no matter how helpful they are
$e = \textrm{the number of edges}$,
$r = \textrm{the number of colors and nodes per edge}$
 - There has to be at least 1 node of each color
 - There has to be at least $r$ nodes
 - There cannot be more than $e(r-1)$ nodes of a single color
 - There has to be at least 2 nodes of the same color in each edge
 - Maybe go the other way and loosen constraints?

(I feel like there's something more that can be said for each edge)

## CSP in general
No-Rainbow is a form of CSP problem.
The main problem with designing an algorithm here is local optimums.
When searching around an area (which we are going to have to do since we cannot generate), *we will need to backtrack* so we never get stuck in a local optimum.
Some ways to do this quickly is:
  - Don't do duplicate work, remember where we've been
  - Reason about the symetries in the problem and simplify it (Ken Thompson chess endgames)
  - Better state-representation so isomorphic states map to the same state

## Visual representation
Either a table with edges and nodes then markings to show what each node is connected to.
```
a: *   (0)
b: **  (-)
c:  ** (-)
d: *** (2)
e:   * (1)
```
All vertical slices are edges. All horizontal slices are nodes.

Here the `-` means "deafult-color", which is equivalent to 0. But the algorithm from the paper only assigns as few colors as possible.

Or nodes on a 2d plane with areas marking out the edges.
(Not as forgiving when making ASCII art :sad:)

## The idea of the algoritm

The main point of the algoritm is that we can partition the solution space and search each "block" individually.
The algoritm does some clever things to limit searches that are guaranteed to be unfruitful. But the idea is really solid.

# The deterministic algoritm by Ghazaleh Parvini and David Ferna´ndez-Baca for reference
```
DetNRC(H):
  foreach inital_candidate_pair(c, F) do
    if DetLocalSearch(H, c, F, (r - 1)n/r) then return 1
  return 0

DetLocalSearch(H, c, F, g):
  if g = 0 and induces_raindbow_edge(c, H) then
    return 0
  if induces_raindbow_edge(c, H).issubset(F) then
    return 0
  if is_no_rainbow_coloring(c, H) then
    return 1
  if |e intersect F| != r - 1 forall e in E then
    return 1
  e' = pick_any(e where |e intersect F| == r - 1)
  v = pick_any(v in (e' - F))
  foreach j in [r] - {c(v)} do
    if DetLocalSearch(H, c but c(v) = j, F union {v}, g - 1) then
      return 1
  return 0
```

# No Rainbow Algoritm Suggestion 1
After trying to "visualize" the multi-dimensional state-space of the searching algorithm I reached the conclusion
that there might be states that are visited multiple times. The coloring make up an equivalence class (there are states which have the same structure, for example the colorings: (0, 1, 2) and (1, 2, 0) since they can be mapped to eachother with a bijective function).
We can define a function that maps these tuples to `the equivalence class representative` by picking the equivalent coloring that is sorted first by all colorings.
Another way of saying this is: we can recolor the coloring so the first color we hit is always color `0`, and the seconds color is `1` etc... This can be done in linear time and is efficient and well defined.
We now know if we've seen this isomorphic state before and can stop the searching.

It remains to be shown that this mapping increases the speed of the algorithm.

```
DetNRC(H):
  foreach inital_candidate_pair(c, F) do
    if DetLocalSearch(H, c, F, (r - 1)n/r) then return 1
  return 0

DetLocalSearch(H, c, F, g):
  if g = 0 and induces_raindbow_edge(c, H) then
    return 0
  if induces_raindbow_edge(c, H).issubset(F) then
    return 0
  if is_no_rainbow_coloring(c, H) then
    return 1
  if |e intersect F| != r - 1 forall e in E then
    return 1
  e' = pick_any(e where |e intersect F| == r - 1)
  v = pick_any(v in (e' - F))
  foreach j in [r] - {c(v)} do
    if DetLocalSearch(H, c but c(v) = j, F union {v}, g - 1) then
      return 1
  return 0
```
