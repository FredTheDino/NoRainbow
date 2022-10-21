# No Rainbow
Based on "Exact Algorithms for No-Raindow Coloring and Phylogenetic Decisiveness" by Ghazaleh Parvini and David Ferna´ndez-Baca

Relevant links:
 - https://en.wikipedia.org/wiki/Stirling_numbers_of_the_second_kind

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
$n = \textrm{the number of nodes}$,
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
After trying to "visualize" the multi-dimensional state-space of the searching
algorithm I reached the conclusion that there might be states that are visited
multiple times. The coloring make up an equivalence class (there are states
which have the same structure, for example the colorings: (0, 1, 2) and (1, 2,
0) since they can be mapped to eachother with a bijective function). We can
define a function that maps these tuples to `the equivalence class
representative` by picking the equivalent coloring that is sorted first by all
colorings. Another way of saying this is: we can recolor the coloring so the
first color we hit is always color `0`, and the seconds color is `1` etc...
This can be done in linear time and is efficient and well defined. We now know
if we've seen this isomorphic state before and can stop the searching.

It remains to be shown that this mapping increases the speed of the algorithm.
But some back of the envelope calculations leads me to believe it should be
faster by a factor $~1/r$. Since we only search $S(n, r)$ instead of $(r
\choose n)^{(r - 1)n/r}$ which I believe is a speed up.
(Need proof here)

That is, we explore all the equivalence classes instead of all possible colorings.
Since a bijective function does not effect validity of the surrjective coloring.
(Might need proof here)

The randomized algorithm doesn't have the problem with searching the same area multiple
times. Mapping the space to it's equivalence classes won't really help there.

Can use same proof for correctness. But we have to show that we indeed visit all equivalence classes.
 - Note from future me, this is actually not true. We cannot use the same proof
since the correctness depends on that we search all nodes, if we map to
equivalence classes how do we know we don't hide anything?

## Things I need to prove
 - The search space from each candidate pair is convex even when mapped to equivalence classes, we don't hide anything when we do the mapping
 - There is an overlap in the search space, so we are visiting fewer nodes with this "heuristic like thing"
 - We still visit all equivalence classes

```
DetNRC(H):
  failedSearches := []
  foreach inital_candidate_pair(c, F) do
    if DetLocalSearch(H, c, F, (r - 1)n/r, failedSearches) then
      return 1

    failedSearches.add(ToRepresentativeColor(c))
  return 0

(O(n))
ToRepresentativeColor(c):
  mapping := empty
  q := 0
  for i in stableNodeOrder(c):
      if c(i) is not mapped in mapping:
        mapping(c(i)) := q
        q := q + 1
  return the recoloring of c using the mapping

DetLocalSearch(H, c, F, g, failedSearches):
  c := ToRepresentativeColor(c)
  foreach q in failedSearches do
    (O(n))
    (Should this be < or <= ? I'm pretty sure this is right)
    if HammingDistance(c, q) <= (r - 1)n/r then
      return 0

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
    if DetLocalSearch(H, c but c(v) = j, F union {v}, g - 1, failedSearches) then
      return 1

  return 0
```

# Other ideas releated to No-Rainbow
  - How many valid colorings are there?
    - n * num-mappings (this is somehow related to sterling numbers)
  - What are the odds of stumbling upon one?
    - If there is any solution, the odds are $>> S(n, r)/r^n$
  - What would a constraint satisfaction algorithm look like?
