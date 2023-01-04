from collections import defaultdict
from itertools import chain

def visit_all_in_category(non_zero_colors, c, i, is_new):
    """
        Visits all nodes in the same category as the `c` exactly once.

        `non_zero_colors` contains the set of colors that are required to be
                          used at least once for the coloring to be surjective
                          except the zero color.
        `c` is the coloring (here passed in as a string)
        `i` is the current index we modify
        `is_new` signals if the current coloring has not been outputted yey
    """
    if is_new: yield c

    # Search down one step but don't output it twice
    yield from visit_all_in_category(non_zero_colors, c, i - 1, False)

    # If this node is frozen don't change it
    if c[i] in non_zero_colors: return

    for other in non_zero_colors:
        d = c[:i] + other + c[i+1:]

        # Different category, we have to check for each color
        if any(d.find(k) != c.find(k) for k in non_zero_colors): continue
            
        yield from visit_all_in_category(non_zero_colors, d, i - 1, True)


def find_top(colors, length, zero_color):
    """
        Generates all top-colorings, all colorings with the colors (i.e. ["a",
        "b", "c"]) of length `length` that are surjective. Runs in polynomial
        time if the colors is fixed.

        `colors` contains the set of colors that are required to be used at
                 least once for the coloring to be surjective.
        `length` is the number of nodes, or the length of the final string
        `zero_color` is the default color to pad with
    """
    if len(colors) == 1:
        yield colors[0] + zero_color * length
    else:
        for i in range(0, length):
            prefix = colors[0] + zero_color * i
            yield from [prefix + suffix
                        for suffix
                        in find_top(colors[1:], length - i, zero_color)
                       ]


def deterministic_no_rainbow_coloring(graph):
  """
      Looks through all unique colorings and finds a no-rainbow coloring if it
      exists.
  
      `graph` the graph we try to color
  """
  assert is_r_uniform(colors)

  r = len(get_random_edge(colors))
  n = count_nodes(graph)

  for top_coloring in find_top(colors, n, colors[0]):
    for coloring in visit_all_in_category(colors[1:], top_coloring, n - 1):
      if is_no_rainbow_coloring(c, graph):
        return c
    
