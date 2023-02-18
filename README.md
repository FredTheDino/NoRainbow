# My Thoughts and Ideas About the No-Rainbow Problem
In fall 2022 I started work on my master thesis at Linköping University by tackling the No Rainbow problem. I quickly found a algorithm that runs faster but have not found a proof to show the algorithm is asymptotically faster - a fancy computer science word for "you can't get this kind of speedup with any number of computers running in parallel". This document summarizes my findings for anyone trying to tackle the same problem in the future, though I was unsuccessful someone else might find value in the ideas presented here. I am however writing this fairly hastily after finally deciding to give up on this problem to find another topic for my master thesis - so please excuse all the typos.

The current best algorithm in this field is done by Parvini et. al. - it's a good idea to [go read that to fully understand this](https://arxiv.org/pdf/2104.02103.pdf).

## The structure of this repository
 - [notes.md](notes.md) contains my internal notes while doing this, not the most clear notes but could potentially give insights.
 - [example/](example/) contains a reference implementation of both my algorithm and Parvini et al.'s algorithm which is the current best algorithm.
 - The running of the reference implementation resulted in graphs, and the raw-data from these runs generate these graphs is in the same folder. All graphs can be found [here](https://docs.google.com/spreadsheets/d/1JWxZERl_KZ6BqO5TmV2J7Wwt9rbWk4sNMNURX_5ATP8/edit?usp=sharing)
 - There are a few stray python scrips lying around, these are small explorations I made to further understand the problem and problem space of NoRainbow
 - Earlier versions of this repository has references to different

# My algorithm - with incomplete proofs
I started by investigating the current best algorithms. This brought me to the paper of Parvini et. al. and I scoured it, and I hope they fixed the typo in the running time analysis of the deterministic algorithm. The deterministic algorithm was slower and less worked on, so i figured it was the easiest way forward. I realized the only way to speed up the algorithm was to change the way the space was partitioned, and I manually searched some spaces with pen and paper to get a feel for the problem. I then realized there never was exactly 1 solution to each problem, there were always more. This made sense and I dug deeper getting me to define the equivalence class for validity of the Colorings. The idea is to only visit each equivalence class node once, this cuts down on the branching factor and gives a faster algorithm. (I later had another idea of adding more layers of equivalence classes, called categories, to filter out even more searching, but this made the algorithm even harder to reason about.)

Here is the algorithm I came up with:

```python
def local_search( g: Graph, c: Coloring, f: Set<X>) -> Option<Coloring>:
    if g.induces_rainbow_edge(c, f): return None
    if g.is_no_rainbow_coloring(&c): return c

    e' = pick_any(e where |e & f| != r - 1 forall e in edges(g))
    if not e': return c

    v = e - f
    for p in colors:
      if p == c(v): continue
      c' = c but c(v) = p
      # This is the only line that is different in my version
      if is_eq_representative(c'):
        local_search(g, c', f + { v })
```
We seed this algorithm with surjective (surjective means we use at least one of each color, if this is confusing you probably need to read more about the NoRainbow problem, but it is infact a big and scary word) colorings of the hypergraph in the same way the Parvini et. al. algorithm does. You can read the example code if you want to study this closer.

Both of the algorithms use the idea that we start with a surjective coloring and search in the space of all surjective colorings.

The first two if-statements are pretty straight forward. If the frozen nodes `f` force there to be a rainbow edge, there's no need to try any other colors for nodes around here.
If `is_no_rainbow_coloring` we're done, so we can exit. 
Otherwise we pick any of the most constrained edges, where all but one of the vertices in an edge are in the frozen set, we then pick that vertex and call it `v`.
We then try all other colors for that vertex to see what works, but my algorithm only tests it if the coloring is an eq-representative.


## The Equivalence-class Eepresentative of Colorings
This is a very simple concept. If you imagine we've ordered each vertex in the graph into a list, we've ascribed a natural number to each of them, this is so we know we always know a specific node is in the same place. We can then create a list of the same length with colors, this is how I represent colorings in the program I've written and a nice way to simplify away the graph when we only want to reason about colors. I often use letters to refer to colors, and we only care about if colors in the same slots are equal. 

Some examples always help. Consider these colorings `ABCC` and `ACBB`, both are surjective so could potentially be solutions to the no-rainbow problem. We also see that we could rename the colors in one to get the other. If we define the mapping `A => A, B => C, C => B` we can turn `ABCC` into `ACBB``. This means they share validity, so we never need to try both of them. The representative of this category is the first one lexicographically, and there are ofcourse `r!` members in each equivalence class if the coloring is surjective. (There's a really fun rabbit hole here with [sterling numbers of the second kind](https://en.wikipedia.org/wiki/Stirling_numbers_of_the_second_kind) which pop up in a lot of places when looking at this problem, this makes sense since we have a bunch of non-empty sets)

## Sketch for correctness
Just like the algorithm for Parvini et. al. we reach all equivalence classes when we search. Consider a surjective coloring C which is also a eq-representative, since the representative is surjective there has to be one starting node which shares `r` colors with `C`, and we can easily move to that starting node by changeing one color of each node at a time. Since all colorings have a lexicographically first equivalent coloring we can do this for all nodes.

Other than that, if the old algorithm is correct, my algorithm is also correct.

## Sketch for speedup
We get a speedup since we prune the search tree more aggressively than the original algorithm. I've tried multiple arguments here, but my most promising one is that we lower the branching factor by 1 for one branch in each tree. This gives us `lim n -> inf [ ((r - 2) + (n - 1)(r - 1))/n ] = (r - 1)`, giving us the same branching factor as the original algorithm. This instance of the algorithm is always faster, but this doesn't show up in the analysis of the algorithms runtime. One can generealize this argument to count the number of branches more thoroughly, but that path is fraught with devious traps of logic.

# Conclusion
The only problem with this algorithm is the runtime, either a faster algorithm needs to be found or someone better at reasoning about algorithms can prove that the runtime is asymtotically faster. I've conducted some experiments out of sheer frustration which have graphs in the [google sheet](https://docs.google.com/spreadsheets/d/1JWxZERl_KZ6BqO5TmV2J7Wwt9rbWk4sNMNURX_5ATP8/edit?usp=sharing), here we see an exponential speed up of around ~`O(p(n)^0.95)` for small `r`. The experiments are inconclusivve though since the amount of data is too small. This is where my journey on this problem end and I hope someone else can find this research useful.
