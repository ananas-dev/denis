use crate::{net::FeedForwardNetwork, pos::Position};

pub fn find_best_move(net: &mut FeedForwardNetwork, pos: &Position) -> (usize, usize) {
    let mut maxscore = -f64::INFINITY;
    let mut best_move = (0, 0);

    for move1 in pos.gen_legal_moves().iter() {
        let mut sub_maxscore = -f64::INFINITY;
        let (x, rotation) = *move1;

        if let Some(pos) = pos.apply_move(x, rotation) {
            for move2 in pos.gen_legal_moves().iter() {
                let (x, rotation) = *move2;

                if let Some(pos) = pos.apply_move(x, rotation) {
                    let input = pos.board.iter().map(|&byte| byte as f64).collect();

                    let score = net.activate(input)[0];

                    if score > sub_maxscore {
                        sub_maxscore = score
                    }
                }
            }
        }

        if sub_maxscore > maxscore {
            maxscore = sub_maxscore;
            best_move = *move1;
        }
    }

    best_move
}
