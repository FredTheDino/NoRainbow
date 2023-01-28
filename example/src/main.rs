use rand::Rng;
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

    let det_correct = det.as_ref().map(|c| g.is_no_rainbow_coloring(&c));
    let me_corrrect = me.as_ref().map(|c| g.is_no_rainbow_coloring(&c));

    assert!(
        det.is_some() == me.is_some(),
        "{:?}, {:?} for {:?}, det={:?}, me={:?}",
        det,
        me,
        g,
        det_correct,
        me_corrrect,
    );
    (det_took, det_states, me_took, me_states)
}

fn main() {
    for i in 6..50 {
        let n = (0..i).collect::<Vec<_>>();
        let _start = Instant::now();

        let mut a_max = 0;
        let mut b_max = 0;
        for _ in 0..50 {
            let mut rng = &mut rand::thread_rng();
            // let density = rng.gen::<f64>() * 0.5 + 0.4; // 0.4 - 0.9
            let c: multigraph::MultiGraph<u32, 3> = multigraph::MultiGraph::random(0.6, &n);
            // let c: multigraph::MultiGraph<u32, 3> = multigraph::MultiGraph::complete(&n);
            // let c: multigraph::MultiGraph<u32, 6> = multigraph::MultiGraph::random(700, &n);
            let (_, an, _, bn) = run_for_all(&c, C);
            // println!("{} {}ms {} states | {}ms {} states", i, a.as_millis(), an, b.as_millis(), bn)
            a_max = a_max.max(an);
            b_max = b_max.max(bn);
        }
        println!("{} {} states | {} states", i, a_max, b_max)
        // println!(" TOOK: {}ms", start.elapsed().as_millis());
    }
}
