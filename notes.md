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
 - There is an overlap in the search space, so we are visiting fewer nodes with these equivalence classes
    - Did some samples, in commit `01cbec8`, this shows it's probably true
```
r: 6 k: 9 normalized: 126 unique states: 504
r: 6 k: 10 normalized: 252 unique states: 1260
r: 6 k: 11 normalized: 462 unique states: 2772
r: 7 k: 9 normalized: 84 unique states: 252
r: 7 k: 10 normalized: 210 unique states: 840
r: 7 k: 11 normalized: 462 unique states: 2310
```
 - We visit every equivalence class that could possibly be a solution

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

# The better algoritm (this one is provable - not apparently, it's super finicky! It was easier to change the algoritm. Also it's not faster.)
  - Filtering out beacons won't work with the frozen set, consider `aaba` and `abaa` edit distance is 2 but we cannot move to them with the fozen set as it is.
  - To prove:
    - Prove: All `top_node` has as many nodes in one color as possible (aaaab, aaaba...)
    - Prove: All `bottom_node` has as many nodes not colored 0 as possible (ababab, abbaab)

    - Prove: All `top_node` has an edit distance of 2 to the closest `top_node` (for cases c != n) (Do I need to prove this?)
    - Prove: All `bottom_node` has an edit distance of 2 to closest `bottom_node` (for cases c != n) (Do I need to prove this?)

    - ~~NOT CORRECT: A `bottom_node` has an edit distance of `r` to a `top_node` (for cases c != n) (WRONG! Consider r = 3, n = 1000000)~~
    - Calculate: Shortest edit distance from `bottom_node` to furthest away node. (Trivial $n - r$, better: We need to do $(r - 1)(n/r - 1)$ swaps to get to the category furthest away  )

## Correctness (a bit hand-wavy at the moment)
We know the algorithm is correct because we visit all categories of equivalence classes since the depth is sufficient.
We need to re-color $(r - 1)(n/r - 1)$ nodes to get to the category with as even a distribution as possible from the
category with the least even distribution. ($(1, 1, 1, 7) \rightarrow (2, 2, 2, 2, 3)$).

> **NOTE**: This isn't super clear. The argument rests on that we have to do (r - 1)(n/r - 1)
> recolorings to end up in e.g. (2, 2, 2, 2, 3) starting in (1, 1, 1, 7). Where (1, 1, 1, 7)
> is where we actually start, all ways to (n - 1 choose r - 1) and from that generate a coloring.
> Or put another way, to go from minimum entropy to maximum entropy.

We also know we visit all instances of the equivalence class since the graph is
fully connected. There cannot exist a coloring which is too far from a start
node. Since that node would have to exist in the category which is furthest
from the node, but we know we always reach that category. All nodes also have to

> **NOTE**: Still a bit unclear, but we visit all instances of equivalence
> classes nodes. Otherwise there would be an equivalence class node that is
> only reachable from inside one of these classes, which cannot happen. This
> has to be true.

And since the previous algorithm suggested by `Ghazaleh Parvini and David Ferna´ndez-Baca` is correct, this algorithm should also be correct.
We still visit all equivalence classes. 

> **NOTE**: We Don't care if we visit all nodes, we only care if we visit all
> equivalence classes. But since we right now don't have limitations, this has
> to be correct. But there's a huge speed up to be gained by writing the proof
> that we don't cut out anything when we only visit images of the equivalence
> classes.

## Speed


## Algorithm
```
DetNRC(H):
  foreach top_node(c, F) do
    if DetLocalSearch(H, c, F, (r - 1)((n/r) - 1)) then
      return 1
  return 0

ToRepresentativeColoring(c):
  mapping := empty
  q := 0
  for i in stableNodeOrder(c):
      if c(i) is not mapped in mapping then
        mapping(c(i)) := q
        q := q + 1
  return the recoloring of c using the mapping

DetLocalSearch(H, c, F, g):
  if ToRepresentativeColoring(c) != c then
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
    (There might be a bigger speed improvement if we filter all neighbors that aren't the image of the equivalence class, we cut off more branches then which is speedy, but I want to make my life easy)
    if DetLocalSearch(H, c but c(v) = j, F union {v}) then
      return 1

  return 0
```

# A potentially even betterone (without proof)
This algoritm is based on the idea that there are orbits in the equivalence classes for the coloring.
So if we could find a node in each orbit, we could exhaust each orbit and thus have visited all nodes without overlap.
For this we need two group actions:
  - Move inside an orbit
  - Move between orbits, has to visit all orbits of the first kind

This would be even faster since there's no overlap. But what is the group action?

```
def find_no_rainbow_coloring(H):
  start := outer := random_surrjective_coloring(H)
  do:
    outer := inner := to_eqv(magic_next_node_in_outer_cycle(outer))
    do:
      inner := to_eqv(magic_next_node_in_cycle(inner))
      if is_no_rainbow_coloring(inner):
        return inner
    while outer != inner
  while curr != start
  return Failed
```

# Another potential one
This algorithm is based on the idea of generating the sets of colorings.
It might be possible to pick out the sets of each coloring and map them to equivalence classes automatically.

The idea is to continuously pick out set-elements and then back-track to pick out more until all equivalence classes of colorings are found.

Each coloring can be put into a category, based on how many of each color there are, sort this tuple and you have a unique mapping.
If we can generate all these sorted tuples and then partition the set of nodes into all possible `r`-partition-sets without repeating we would be golden.

A super tiny sketch
```
Search(Graph):
  foreach X := partition_scheem(r, n):
    if NoRainbowColoring(Partition(X, Nodes(Graph)), Graph):
      return 1
```

# Another jabb - 2022-10-02

*top-regex*: `top_coloring` is a coloring which follows the regular expressions: `aa*ba*c*d*...`, that is, it starts with an `a`, and has all the other colors placed out with `a`-s in between. The frozen nodes are the first node and all nodes which do not have the color `a`.

*eq-regex*: Note that the `ToRepresentativeColoring` makes everything match: `aa*b[ab]*c[abc]*d[abcd]*...` which is a subset of the starting regex, we know we are in a valid coloring when we start.

> **NOTE**: The DFA for this coloring looks pretty funky. I should draw it.

## Algorithm
```
DetNRC(H):
  foreach top_coloring(c, F) do
    if DetLocalSearch(H, c, F, n - r) then
      return 1
  return 0

(Can this be done faster?) 
ToRepresentativeColoring(c):
  mapping := empty
  q := 0
  for i in stableNodeOrder(c):
      if c(i) is not mapped in mapping then
        mapping(c(i)) := q
        q := q + 1
  return the recoloring of c using the mapping

DetLocalSearch(H, c, F, g):
  -- This is the secret sauce
  if ToRepresentativeColoring(c) != c then
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
    if DetLocalSearch(H, c but c(v) = j, F union {v}) then
      return 1

  return 0
```

## Correctness (a bit hand-wavy at the moment)
Some sub points that need proving:

### All equivalence classes of coloring can be reachable from a top coloring
Given a representative coloring $X$.
Since $X$ is a representative we know it matches the *eq-regex*.
We can now write a stricter regex: *a-regex* `a [a]^m1 b [ab]^m2 c [abc]^m3` where $m_i$ is picked to match $X$.
Note specially that *a-regex* doesn't allow moving of the appearance of a color.
(Note: I think this step is obvious, but maybe I need to clarify it more?)
It's now easy to see that:
  1. *eq-regex* > *a-regex*
  2. *eq-regex* > *top-regex*
  3. *top-regex* & *a-regex* != 0 (This is questionable, and there might be a better way to express it)

Since *top-regex* & *a-regex* is non-empty we can pick an element $Y$ from it.
Since we cannot move the first instance of the colors in anything that is in *a-regex*, we can stay in *a-regex* and still reach $Y$ from $X$ replacing a maximum of $n - r$ colors.
Thus $X$ is reachable from $Y$ and $Y$ is a top coloring -- thus all colorings in *eq-regex* are reachable from a top coloring.

--------------
*Intuition*:
We can think of the *a-regex* as describing a vertical slice through the space of all colorings. And *top-regex* as describing a horizontal slice.
And this shows that everthing in the vertical slice is connected. And we know that we completely partition the space.
So now we have reached all of the nodes.

### The depth of $n - r$ is sufficient and
 - See previous proof

## Speed (A bit hand wavy, sorry)
Similar arguments to Parvini.
  - Work per node is polynomial
  - The number of children per node is harder to define exactly.
    In the range $(r - 1)$ and $0$ depending on the node we pick.
    I'm pretty sure there are en equal number of $0$s and $(r-1)$, so maybe I can get away with $(r-1)/2$ or even $r/2$ as an over approximation?

We get closer to, I'm guessing  $O*((\frac{r-1}{2])^{n - r})$. This is however a *substantial* speed improvement.

