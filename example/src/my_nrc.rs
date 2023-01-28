use itertools::Itertools;
use std::collections::{BTreeSet, BTreeMap};
use std::fmt::Debug;
use std::marker::Copy;

use crate::multigraph::{Coloring, MultiGraph};

pub fn no_rainbow_coloring<X, C, const R: usize>(
    g: &MultiGraph<X, R>,
    colors: [C; R],
) -> (usize, Option<Coloring<C>>)
where
    X: Ord + Copy + Debug,
    C: Ord + Copy + Debug,
{
    assert_eq!(colors.iter().collect::<BTreeSet<&C>>().len(), R);
    // ASSSUMES THIS:
    // let colors = colors.is_sorted();

    let mut states = 0;
    let n = g.node_to_i.len();
    let i_to_node = g.node_to_i.iter().map(|(a, b)| (*b, *a)).collect::<BTreeMap<usize, X>>();
    for picked in g.node_to_i.iter().combinations(R) {
        let mut f = picked.iter().map(|(x, _)| **x).collect::<BTreeSet<X>>();
        let mut c = vec![colors[0]; n];
        let mut m = colors[0];

        for (s, (_, i)) in picked.iter().enumerate() {
            c[**i] = colors[s];
        }

        for x in c.iter_mut() {
            m = m.max(*x);
            *x = m;
        }

        for (i, t) in c.iter().enumerate() {
            if *t == colors[0] {
                f.insert(i_to_node[&i]);
            }
        }

        assert_eq!(Coloring(c.clone()).used_colors(), R);
        match det_local_search(&mut states, colors, g, Coloring(c), &f) {
            Some(out) => return (states, Some(out)),
            None => continue,
        }
    }
    (states, None)
}

fn det_local_search<X, C, const R: usize>(
    states: &mut usize,
    colors: [C; R],
    g: &MultiGraph<X, R>,
    c: Coloring<C>,
    f: &BTreeSet<X>,
) -> Option<Coloring<C>>
where
    X: Ord + Copy + Debug,
    C: Ord + Copy,
{
    *states += 1;
    if g.induces_rainbow_edge(&c, &f) {
        return None;
    }
    if g.is_no_rainbow_coloring(&c) {
        return Some(c);
    }
    match g.find_unfrozen_node_in_maximally_constrained_edge(&f) {
        None => return Some(c),
        Some(v) => {
            let mut f = f.clone();
            f.insert(v);
            let f = f;

            let i = g.node_to_i[&v];
            let first_color = c.0[i];
            for t in colors.iter() {
                if !(t < &first_color) {
                    continue;
                }
                let new_c = c.mutate(i, *t);
                match det_local_search(states, colors, g, new_c, &f) {
                    Some(c) => return Some(c),
                    None => {}
                }
            }
            return None;
        }
    }
}
