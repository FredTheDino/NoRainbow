use itertools::Itertools;
use std::collections::BTreeSet;
use std::fmt::Debug;
use std::marker::Copy;

use crate::multigraph::{Coloring, MultiGraph};

pub fn no_rainbow_coloring<X, C, const R: usize>(
    g: &MultiGraph<X, R>,
    colors: [C; R],
) -> Option<Coloring<C>>
where
    X: Ord + Copy + Debug,
    C: Ord + Copy,
{
    assert_eq!(colors.iter().collect::<BTreeSet<&C>>().len(), R);

    let n = g.node_to_i.len();
    for picked in g.node_to_i.iter().combinations(R) {
        let f = picked.iter().map(|(x, _)| **x).collect::<BTreeSet<X>>();
        let mut c = vec![colors[0]; R];
        for (t, (_, i)) in colors.iter().zip(picked.iter()) {
            c[**i] = *t;
        }

        match det_local_search(colors, g, Coloring(c), &f, (R - 1) * n / R) {
            Some(out) => return Some(out),
            None => continue,
        }
    }
    None
}

fn det_local_search<X, C, const R: usize>(
    colors: [C; R],
    g: &MultiGraph<X, R>,
    c: Coloring<C>,
    f: &BTreeSet<X>,
    d: usize,
) -> Option<Coloring<C>>
where
    X: Ord + Copy + Debug,
    C: Ord + Copy,
{
    if d == 0 && g.contains_rainbow_edge(&c) {
        return None;
    }
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
                if t == &first_color { continue; }
                match det_local_search(colors, g, c.mutate(i, *t), &f, d - 1) {
                    Some(c) => return Some(c),
                    None => {},
                }
            }
            return None;
        }
    }
}
