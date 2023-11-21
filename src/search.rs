use crate::{net::FeedForwardNetwork, pos::Position};

pub fn find_best_move(net: &mut FeedForwardNetwork, pos: &Position) -> (usize, usize) {
    let mut maxscore = -f64::INFINITY;
    let mut best_move = (0, 0);

    for &(col, rot) in pos.gen_legal_moves().iter() {
        if let Some(pos) = pos.apply_move(col, rot) {
            let score = search(net, pos, 1);

            if score > maxscore {
                maxscore = score;
                best_move = (col, rot);
            }
        }
    }

    best_move
}

fn search(net: &mut FeedForwardNetwork, pos: Position, depth: usize) -> f64 {
    if depth == 0 {
        let features = pos.features();

        return -0.510066 * features.aggregate_height
            + 0.760666 * features.completed_lines
            + -0.35663 * features.holes
            + -0.184483 * features.bumpiness;
        // return net.activate(vec![
        //     features.completed_lines,
        //     features.holes,
        //     features.bumpiness,
        //     features.aggregate_height,
        // ])[0];
    }

    let mut maxscore = -f64::INFINITY;

    for &(col, rot) in pos.gen_legal_moves().iter() {
        if let Some(pos) = pos.apply_move(col, rot) {
            let score = search(net, pos, depth - 1);

            if score > maxscore {
                maxscore = score;
            }
        }
    }

    maxscore
}
