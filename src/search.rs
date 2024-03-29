use crate::{
    net::FeedForwardNetwork,
    pos::{Action, Position},
    transposition::TranspositionTable,
};

const MAX_DEPTH: usize = 3;

pub struct Search {
    tt: TranspositionTable,
}

impl Search {
    pub fn new() -> Search {
        Search {
            tt: TranspositionTable::new(16384),
        }
    }

    pub fn run(
        &mut self,
        net: &mut FeedForwardNetwork,
        pos: &Position,
    ) -> Option<((usize, usize, usize), Vec<Action>)> {
        let mut best_move = None;
        for depth in 2..3 {
            best_move = self.search_root(net, depth, pos);
        }

        let best_move = best_move?;

        Some((
            best_move,
            pos.path((best_move.0 as i32, best_move.1 as i32, best_move.2 as i32)),
        ))
    }

    fn search_root(
        &mut self,
        net: &mut FeedForwardNetwork,
        depth: usize,
        pos: &Position,
    ) -> Option<(usize, usize, usize)> {
        let mut maxscore = -f64::INFINITY;
        let mut best_move = None;

        for &(p, x, y, rot) in pos.legal_moves()[0].iter() {
            let pos = pos.apply_move(p, x, y, rot, false);
            let score = self.search(net, pos, depth - 1);

            if score > maxscore {
                maxscore = score;
                best_move = Some((x, y, rot));
            }
        }

        best_move
    }

    fn search(&mut self, net: &mut FeedForwardNetwork, pos: Position, depth: usize) -> f64 {
        if depth == 0 {
            if let Some(score) = self.tt.get(pos.hash) {
                return score;
            } else {
                let features = pos.features();

                // let score = features.aggregate_height * -0.510066
                //     + features.holes * -0.35663
                //     + features.bumpiness * -0.184483;

                let score = net.activate(vec![
                    features.holes,
                    features.bumpiness,
                    features.aggregate_height,
                ])[0];

                self.tt.set(pos.hash, score);

                return score;
            }
        }

        let mut maxscore = 0.;
        let piece_list = pos.legal_moves();
        for piece_moves in piece_list {
            if piece_moves.is_empty() {
                continue;
            }

            let mut piece_maxscore = -f64::INFINITY;

            let piece_color = piece_moves[0].0;

            for (p, x, y, rot) in piece_moves {
                let pos = pos.apply_move(p, x, y, rot, false);
                let score = self.search(net, pos, depth - 1);

                if score > piece_maxscore {
                    piece_maxscore = score;
                }
            }

            let prob = if pos.last_piece == piece_color {
                3.57
            } else {
                16.07
            };

            maxscore += piece_maxscore / prob;
        }

        maxscore
    }
}
