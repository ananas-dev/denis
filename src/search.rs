use std::time::{Duration, Instant};

use crate::{net::FeedForwardNetwork, pos::{Position, Action}};

pub fn find_best_move(net: &mut FeedForwardNetwork, pos: &Position) -> ((usize, usize, usize), Vec<Action>) {
    let mut maxscore = -f64::INFINITY;
    let mut best_move = (0, 0, 0);

    for &(x, y, rot) in pos.legal_moves().iter() {
        if let Some(pos) = pos.apply_move(x, y, rot, false) {
            let score = search(net, pos, 1);

            if score > maxscore {
                maxscore = score;
                best_move = (x, y, rot);
            }
        }
    }

    let test = (best_move.0 as i32, best_move.1 as i32, best_move.2 as i32);

    (best_move, pos.path(test))
}

fn search(net: &mut FeedForwardNetwork, pos: Position, depth: usize) -> f64 {
    if depth == 0 {
        // Flatten the board and pass it to the neural net
        // return net.activate(
        //     pos.board
        //         .into_iter()
        //         .flat_map(|inner| inner)
        //         .map(|c| if c != 0 { 1. } else { 0. })
        //         .collect(),
        // )[0];
        let features = pos.features();

        return net.activate(vec![
            0.,
            features.holes,
            features.bumpiness,
            features.aggregate_height,
        ])[0];
    }

    let mut maxscore = -f64::INFINITY;

    for &(x, y, rot) in pos.legal_moves().iter() {
        if let Some(pos) = pos.apply_move(x, y, rot, false) {
            let score = search(net, pos, depth - 1);

            if score > maxscore {
                maxscore = score;
            }
        }
    }

    maxscore
}
