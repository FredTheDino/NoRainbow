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

fn run_for_all<X, C, const R: usize>(g: &multigraph::MultiGraph<X, R>, colors: [C; R])
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

    //let corr = complete_nrc::no_rainbow_coloring(g, colors);

    let det_correct = det.as_ref().map(|c| g.is_no_rainbow_coloring(&c));
    let me_corrrect = me.as_ref().map(|c| g.is_no_rainbow_coloring(&c));
    // let corr_correct = corr.as_ref().map(|c| g.is_no_rainbow_coloring(&c));

    assert!(matches!(det_correct, Some(true) | None));
    assert!(matches!(me_corrrect, Some(true) | None));

    if det.is_some() != me.is_some() {
        print_dot(&g, &det.clone().or(me.clone()).unwrap());

        panic!(
            "\n\n det: {:?}  {:?}\n me: {:?}  {:?}\n\n for \n\n{:?}",
            det, det_correct, me, me_corrrect, g,
        );
    }
    let n = g.node_to_i.len();
    println!(
        "{{ r: {}, n: {}, det_states: {}, det_time: {}, me_states: {}, me_time: {}  }}",
        R,
        n,
        det_states,
        det_took.as_millis(),
        me_states,
        me_took.as_millis()
    );
}

fn run_X_and_block<const R: usize>(n: u32, c: [Color; R]) {
    let mut threads = Vec::new();
    for _i in 0..7 {
        let n = n.clone();
        let c = c.clone();
        threads.push(std::thread::spawn(move || {
            let xs = (0..n).collect::<Vec<_>>();
            let rng = &mut rand::thread_rng();
            let density = rng.gen::<f64>(); // 0.0 - 1.0
            let g: multigraph::MultiGraph<u32, R> = multigraph::MultiGraph::random(density, &xs);
            run_for_all(&g, c);
        }));
    }

    for t in threads.into_iter() {
        t.join().unwrap();
    }
}

fn main() {
        for i in 4..30 {
            run_X_and_block(i, [CA, CB, CC]);
            run_X_and_block(i, [CA, CB, CC, CD]);
            run_X_and_block(i, [CA, CB, CC, CD, CE]);
        }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_det() {
        let c = [1, 2, 3];
        {
            let g = multigraph::MultiGraph::new(vec![[0, 1, 3], [0, 2, 4], [1, 2, 3], [1, 3, 4]]);
            run_for_all(&g, c);
        }

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
