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
    for c in colors.iter().combinations(n) {
        let c = Coloring(c.iter().map(|x| **x).collect());
        if g.is_no_rainbow_coloring(&c) {
            return Some(c);
        }
    }
    None
}
