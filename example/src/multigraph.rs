use itertools::Itertools;
use rand::prelude::SliceRandom;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Debug;
use std::marker::Copy;

#[derive(Debug)]
pub struct MultiGraph<X, const R: usize>
where
    X: Ord + Copy + Debug,
{
    pub edges: BTreeSet<BTreeSet<X>>,
    pub node_to_i: BTreeMap<X, usize>,
}

impl<X, const R: usize> MultiGraph<X, R>
where
    X: Ord + Copy + Debug,
{
    pub fn new(edges: Vec<[X; R]>) -> Self {
        let edges = edges
            .into_iter()
            .map(|xs| xs.into())
            .collect::<BTreeSet<BTreeSet<X>>>();
        Self::new_from_set(edges)
    }

    fn new_from_set(edges: BTreeSet<BTreeSet<X>>) -> Self {
        for edge in edges.iter() {
            assert!(
                edge.len() == R,
                "One edge has the same node twice {:?}",
                edge
            );
        }
        let nodes = edges.iter().flatten().cloned().collect::<BTreeSet<X>>();
        let node_to_i = nodes
            .iter()
            .enumerate()
            .map(|(i, n)| (*n, i))
            .collect::<BTreeMap<X, usize>>();
        Self { edges, node_to_i }
    }

    pub fn find_unfrozen_node_in_maximally_constrained_edge(
        &self,
        frozen: &BTreeSet<X>,
    ) -> Option<X> {
        for edge in self.edges.iter() {
            let edge = edge.iter().cloned().collect::<BTreeSet<X>>();
            let unfrozen = edge.difference(&frozen).collect::<Vec<_>>();
            if unfrozen.len() == 1 {
                return Some(*unfrozen[0]);
            }
        }
        return None;
    }

    pub fn induces_rainbow_edge<C: Ord + Copy + Debug>(
        &self,
        coloring: &Coloring<C>,
        frozen: &BTreeSet<X>,
    ) -> bool {
        'outer: for edge in self.edges.iter() {
            let mut seen = BTreeSet::new();
            for n in edge {
                if !frozen.contains(n) {
                    continue 'outer;
                }
                let c = coloring.0[self.node_to_i[&n]];
                if seen.contains(&c) {
                    continue 'outer;
                }
                seen.insert(c);
            }
            return true;
        }
        return false;
    }

    pub fn contains_rainbow_edge<C: Ord + Copy + Debug>(&self, coloring: &Coloring<C>) -> bool {
        'outer: for edge in self.edges.iter() {
            let mut seen = BTreeSet::new();
            for n in edge {
                let c = coloring.0[self.node_to_i[&n]];
                if seen.contains(&c) {
                    continue 'outer;
                }
                seen.insert(c);
            }
            return true;
        }
        return false;
    }

    pub fn is_no_rainbow_coloring<C: Ord + Copy + Debug>(&self, coloring: &Coloring<C>) -> bool {
        coloring.used_colors() == R && !self.contains_rainbow_edge(coloring)
    }

    pub fn random(to_keep: f64, nodes: &[X]) -> MultiGraph<X, R> {
        let all_edges = nodes
            .into_iter()
            .combinations(R)
            .map(|x| x.into_iter().cloned().collect::<BTreeSet<X>>())
            .collect::<Vec<BTreeSet<X>>>();
        let mut rng = &mut rand::thread_rng();
        Self::new_from_set(
            all_edges
                .choose_multiple(&mut rng, (all_edges.len() as f64 * to_keep) as usize)
                .cloned()
                .collect(),
        )
    }

    pub fn complete(nodes: &[X]) -> MultiGraph<X, R> {
        Self::new_from_set(
            nodes
                .into_iter()
                .combinations(R)
                .map(|x| x.into_iter().cloned().collect::<BTreeSet<X>>())
                .collect::<BTreeSet<BTreeSet<X>>>(),
        )
    }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone)]
pub struct Coloring<C: Ord + Copy + Debug>(pub Vec<C>);

impl<C: Ord + Copy + Debug> Coloring<C> {
    pub fn is_same_cat(ax: &Coloring<C>, bx: &Coloring<C>) -> bool {
        ax.to_cat() == bx.to_cat()
    }

    pub fn used_colors(&self) -> usize {
        self.0.iter().collect::<BTreeSet<&C>>().len()
    }

    pub fn mutate(&self, i: usize, c: C) -> Coloring<C> {
        let mut out = self.0.clone();
        out[i] = c;
        Coloring(out)
    }

    pub fn to_cat(&self) -> Vec<(C, usize)> {
        let mut n = vec![(self.0[0], 0)];
        let mut c = self.0[0];
        for (i, x) in self.0.iter().enumerate().skip(1) {
            if x > &c {
                n.push((*x, i));
                c = *x;
            }
        }
        n
    }

    pub fn is_eq(&self) -> bool {
        let asc = self
            .0
            .clone()
            .into_iter()
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        let mut i = 0;
        for c in self.0.iter() {
            if c <= &asc[i] {
                continue;
            }

            i += 1;
            if c != &asc[i] {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {
    use super::*;

    fn is_same_cat<const N: usize>(a: [i64; N], b: [i64; N]) {
        let a = Coloring(a.to_vec());
        let b = Coloring(b.to_vec());
        assert!( Coloring::is_same_cat(&a, &b), "Expected same category a={:?}, b={:?}", a, b);
    }

    fn is_different_cat<const N: usize>(a: [i64; N], b: [i64; N]) {
        let a = Coloring(a.to_vec());
        let b = Coloring(b.to_vec());
        assert!( !Coloring::is_same_cat(&a, &b), "Expected not same category a={:?}, b={:?}", a, b);
    }

    #[test]
    fn same_cat_1() { is_same_cat([0, 0, 1, 1, 2, 2], [0, 0, 1, 0, 2, 0]); }
    #[test]
    fn same_cat_2() { is_same_cat([0, 0, 1, 0, 2, 0], [0, 0, 1, 0, 2, 0]); }
    #[test]
    fn same_cat_3() { is_same_cat([1, 1, 1, 1, 1, 1], [1, 1, 1, 1, 1, 1]); }
    #[test]
    fn same_cat_4() { is_same_cat([0, 1, 2, 0, 1, 2], [0, 1, 2, 0, 1, 2]); }
    // #[test]
    // fn same_cat_4() { is_same_cat([], []); }
    #[test]
    fn same_cat_5() { is_same_cat([1], [1]); }
    #[test]
    fn same_cat_6() { is_same_cat([3, 2, 1], [3, 1, 2]); }
    #[test]
    fn diff_cat_1() { is_different_cat([1, 3, 3], [2, 2, 2]); }
    #[test]
    fn diff_cat_2() { is_different_cat([1, 2, 3], [1, 2, 2]); }
    #[test]
    fn diff_cat_3() { is_different_cat([1, 1, 1, 1], [1, 1, 1, 2]); }
    #[test]
    fn diff_cat_4() { is_different_cat([1, 1, 2, 2, 3], [1, 1, 2, 3, 3]); }

    fn surjectivity<const R: usize, const N: usize>(a: [i64; N]) {
        let got = Coloring(a.to_vec()).used_colors();
        assert!(got == R, "a={:?}, {:?}!={:?} where I expected {:?}", a, got, R, R);
    }

    #[test]
    fn surr_1() { surjectivity::<2, 4>([1, 1, 1, 2]) }
    #[test]
    fn surr_2() { surjectivity::<3, 4>([1, 3, 2, 2]) }
    #[test]
    fn surr_3() { surjectivity::<4, 4>([1, 2, 3, 4]) }
    #[test]
    fn surr_4() { surjectivity::<4, 4>([4, 2, 3, 5]) }
    #[test]
    fn surr_5() { surjectivity::<1, 4>([1, 1, 1, 1]) }
    #[test]
    fn surr_6() { surjectivity::<1, 5>([1, 1, 1, 1, 1]) }

    #[test]
    fn test_eq() {
        assert!(Coloring(vec![1, 2, 3, 2, 1]).is_eq())
    }

    #[test]
    fn test_multigraph() {
        let edges = vec![[0, 1, 2], [1, 2, 3], [3, 0, 1], [0, 3, 2]];
        let g = MultiGraph::new(edges.clone());

        let c1 = Coloring([0, 0, 1, 2].into());
        assert!(g.contains_rainbow_edge(&c1), "Should contain a rainbow edge {:?} in {:?}", c1, edges);
        let c2 = Coloring([0, 1, 1, 0].into());
        assert!(!g.contains_rainbow_edge(&c2), "Should not contain a rainbow edge {:?} in {:?}", c2, edges);

        let f1 = [1, 2].into();
        assert_eq!(g.find_unfrozen_node_in_maximally_constrained_edge(&f1), Some(0));
        let f2 = [1, 2, 3, 0].into();
        assert_eq!(g.find_unfrozen_node_in_maximally_constrained_edge(&f2), None);
        let f3 = [].into();
        assert_eq!(g.find_unfrozen_node_in_maximally_constrained_edge(&f3), None);

        assert!(!g.induces_rainbow_edge::<usize>(&c1, &f1));
        assert!(g.induces_rainbow_edge::<usize>(&c1, &f2));
    }
}
