use crate::{
    net::FeedForwardNetwork,
    pos::{Action, Position},
    transposition::TranspositionTable,
};

pub struct Search {
    tt: TranspositionTable,
}

impl Search {
    pub fn new() -> Search {
        Search { tt: TranspositionTable::new(16384) }
    }
    pub fn find_best_move(
        &mut self,
        net: &mut FeedForwardNetwork,
        pos: &Position,
    ) -> ((usize, usize, usize), Vec<Action>) {
        let mut maxscore = -f64::INFINITY;
        let mut best_move = (0, 0, 0);

        for &(x, y, rot) in pos.legal_moves().iter() {
            if let Some(pos) = pos.apply_move(x, y, rot, false) {
                let score = self.search(net, pos, 1);

                if score > maxscore {
                    maxscore = score;
                    best_move = (x, y, rot);
                }
            }
        }

        let test = (best_move.0 as i32, best_move.1 as i32, best_move.2 as i32);

        (best_move, pos.path(test))
    }

    fn search(&mut self, net: &mut FeedForwardNetwork, pos: Position, depth: usize) -> f64 {
        if depth == 0 {

            if let Some(score) = self.tt.get(pos.hash) {
                return score
            } else {
                let features = pos.features();

                let score = net.activate(vec![
                    0.,
                    features.holes,
                    features.bumpiness,
                    features.aggregate_height,
                ])[0];

                self.tt.set(pos.hash, score);

                return score;
            }

        }

        let mut maxscore = -f64::INFINITY;

        for &(x, y, rot) in pos.legal_moves().iter() {
            if let Some(pos) = pos.apply_move(x, y, rot, false) {
                let score = self.search(net, pos, depth - 1);

                if score > maxscore {
                    maxscore = score;
                }
            }
        }

        maxscore
    }
}
