use itertools::Itertools;
use rand::Rng;
use std::collections::BTreeMap;
use std::fmt::Debug;
use std::marker::Copy;
use std::time::{Duration, Instant};

mod complete_nrc;
mod det_nrc;
mod multigraph;
mod my_nrc;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
enum Color {
    CA,
    CB,
    CC,
    CD,
    CE,
    CF,
}
use Color::*;

const C: [Color; 3] = [CA, CB, CC];

fn print_dot<X, C, const R: usize>(
    g: &multigraph::MultiGraph<X, R>,
    coloring: &multigraph::Coloring<C>,
) where
    X: Ord + Copy + Debug,
    C: Ord + Copy + Debug,
{
    let mut c_to_int = BTreeMap::new();
    for c in coloring.0.iter() {
        let n = c_to_int.len() % 12 + 1;
        match c_to_int.entry(*c) {
            std::collections::btree_map::Entry::Vacant(e) => {
                e.insert(n);
            }
            std::collections::btree_map::Entry::Occupied(_) => {}
        }
    }

    println!("=== DOT BEGIN ===");
    println!(
        r#"
graph G {{
     node [colorscheme="set312"];
     edge [colorscheme="set312"];
"#
    );

    for (x, i) in g.node_to_i.iter() {
        println!(
            "{:?} [fillcolor={}, style=filled];",
            x, c_to_int[&coloring.0[*i]]
        );
    }

    for (i, e) in g.edges.iter().enumerate() {
        let n = i % 12 + 1;
        for v in e.iter().combinations(2) {
            print!("{:?} -- {:?} [color={}]; ", v[0], v[1], n);
        }
        println!("");
    }

    println!("}}");
    println!("=== DOT END ===");
}

fn run_for_all<X, C, const R: usize>(
    g: &multigraph::MultiGraph<X, R>,
    colors: [C; R],
) -> (Duration, usize, Duration, usize)
where
    X: Ord + Copy + Debug,
    C: Ord + Copy + Debug,
{
    let det_start = Instant::now();
    let (det_states, det) = det_nrc::no_rainbow_coloring(g, colors);
    let det_took = det_start.elapsed();

    let me_start = Instant::now();
    let (me_states, me) = my_nrc::no_rainbow_coloring(g, colors);
    let me_took = me_start.elapsed();

    let corr = me.clone(); // complete_nrc::no_rainbow_coloring(g, colors);

    let det_correct = det.as_ref().map(|c| g.is_no_rainbow_coloring(&c));
    let me_corrrect = me.as_ref().map(|c| g.is_no_rainbow_coloring(&c));
    let corr_correct = corr.as_ref().map(|c| g.is_no_rainbow_coloring(&c));

    if det.is_some() != me.is_some() || det.is_some() != corr.is_some() {
        print_dot(&g, &det.clone().or(me.clone()).unwrap());

        panic!(
            "\n\n det: {:?}  {:?}\n me: {:?}  {:?}\n corr: {:?}  {:?}\n\n for \n\n{:?}",
            det, det_correct, me, me_corrrect, corr, corr_correct, g,
        );
    }
    (det_took, det_states, me_took, me_states)
}

fn main() {
    let start = Instant::now();
    for i in 4..50 {
        let n = (0..i).collect::<Vec<_>>();

        let mut a_max = 0;
        let mut b_max = 0;
        for _i in 0..100 {
            let rng = &mut rand::thread_rng();
            let density = rng.gen::<f64>() * 0.5 + 0.4; // 0.4 - 0.9
            let c: multigraph::MultiGraph<u32, 3> = multigraph::MultiGraph::random(density, &n);
            let (_, an, _, bn) = run_for_all(&c, C);
            a_max = a_max.max(an);
            b_max = b_max.max(bn);
        }
        println!("{} | {} states | {} states ~ {}", i, a_max, b_max, (a_max * 10) / (b_max * 10));
    }
    println!(" TOOK: {}ms", start.elapsed().as_millis());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_det() {
        let c = [1, 2, 3];
        {
            let g = multigraph::MultiGraph::new(vec![
                [0, 1, 3],
                [0, 2, 4],
                [1, 2, 3],
                [1, 3, 4],
            ]);
            run_for_all(&g, c);
        }
        return;

        {
            let g = multigraph::MultiGraph::new(vec![
                [0, 1, 2],
                [0, 1, 3],
                [0, 1, 4],
                [0, 2, 4],
                [0, 2, 5],
                [0, 4, 5],
                [1, 2, 4],
                [1, 2, 5],
                [1, 4, 5],
                [2, 3, 4],
                [2, 3, 5],
                [3, 4, 5],
            ]);
            run_for_all(&g, c);
        }

        {
            let g2 = multigraph::MultiGraph::new(vec![
                [0, 1, 2],
                [0, 1, 3],
                [0, 1, 5],
                [0, 2, 3],
                [0, 2, 5],
                [0, 3, 5],
                [1, 2, 5],
                [1, 3, 5],
                [1, 4, 5],
                [2, 3, 4],
                [2, 3, 5],
                [2, 4, 5],
            ]);
            assert!(det_nrc::no_rainbow_coloring(&g2, c).1.is_some());
            assert!(my_nrc::no_rainbow_coloring(&g2, c).1.is_some());
            assert!(complete_nrc::no_rainbow_coloring(&g2, c).is_some());
        }

        {
            let g = multigraph::MultiGraph::new(vec![
                [0, 1, 3],
                [0, 2, 3],
                [0, 2, 4],
                [0, 2, 5],
                [0, 3, 4],
                [0, 3, 5],
                [0, 4, 5],
                [1, 2, 4],
                [1, 3, 5],
                [2, 3, 4],
                [2, 3, 5],
                [2, 4, 5],
            ]);
            run_for_all(&g, c);
        }
    }
}
