use std::time::Instant;

use nn::FeedForwardNetwork;

use crate::pos::Position;

mod nn;
mod pos;
mod comm;
mod search;

    // let node_evals = vec![
    //     (13, -1.400910578179514, 1.0, vec![(-3, 1.467054101340855)]),
    //     (
    //         0,
    //         -0.4410094679025036,
    //         1.0,
    //         vec![
    //             (-1, 0.09617213827046978),
    //             (-2, -0.3845614757103543),
    //             (-4, -0.005114015014206566),
    //             (13, -0.5493702611867279),
    //         ],
    //     ),
    // ];

    // let inputs = vec![-1, -2, -3, -4];
    // let outputs = vec![0];

fn main() {
    comm::start().unwrap();
}
