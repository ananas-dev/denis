use lazy_static::lazy_static;
use rand::{seq::SliceRandom, thread_rng, Rng};
use std::{collections::VecDeque, fmt, hash::{Hash, Hasher}};

lazy_static! {
    static ref PIECES: Vec<Vec<Vec<Vec<u8>>>> = {
        let piece_shapes: Vec<Vec<Vec<u8>>> = vec![
            vec![vec![0, 0, 1], vec![1, 1, 1]],
            vec![vec![2, 0, 0], vec![2, 2, 2]],
            vec![vec![0, 3, 3], vec![3, 3, 0]],
            vec![vec![4, 4, 0], vec![0, 4, 4]],
            vec![vec![0, 5, 0], vec![5, 5, 5]],
            vec![vec![6, 6], vec![6, 6]],
            vec![vec![7, 7, 7, 7]],
        ];

        let mut pieces = Vec::new();

        for piece_number in 1..8 {
            let mut rotations = Vec::new();
            let mut last_shape = piece_shapes[piece_number - 1].clone();
            rotations.push(last_shape.clone());

            for _ in 1..4 {
                rotate_matrix(&mut last_shape);
                rotations.push(last_shape.clone());
            }

            pieces.push(rotations);
        }

        pieces
    };
    static ref ZOBRIST: Vec<u64> = {
        let mut rng = thread_rng();

        (0..(22 * 12)).map(|_| rng.gen()).collect()
    };
}

fn rotate_matrix(matrix: &mut Vec<Vec<u8>>) {
    let n = matrix.len();
    let m = matrix[0].len();
    let mut result = vec![vec![0; n]; m];

    for i in 0..n {
        for j in 0..m {
            result[j][n - 1 - i] = matrix[i][j];
        }
    }

    *matrix = result;
}

#[derive(Debug)]
pub struct Features {
    pub holes: f64,
    pub blocades: f64,
    pub height: f64,
    pub lines: f64,
}

#[derive(Debug)]
pub struct Position {
    pub score: i64,
    pub current_piece: usize,
    pub next_pieces: VecDeque<usize>,
    pub pocket: Option<usize>,
    pub bag: Vec<usize>,
    pub lines: usize,
    pub board: Vec<Vec<u8>>,
}

impl Position {
    pub fn new(
        current_piece: usize,
        next_pieces: VecDeque<usize>,
        lines: usize,
        score: i64,
        board: Vec<Vec<u8>>,
        bag: Vec<usize>,
        pocket: Option<usize>,
    ) -> Self {
        Position {
            current_piece,
            next_pieces,
            lines,
            bag,
            score,
            board,
            pocket,
        }
    }

    pub fn gen_legal_moves(&self) -> Vec<(usize, usize, bool)> {
        let mut legal_moves = Vec::new();

        for rotation in 0..4 {
            let piece = &PIECES[self.current_piece - 1][rotation];
            let size_x = piece[0].len();
            for x in 0..(13 - size_x) {
                legal_moves.push((x, rotation, false));
            }
            if let Some(pocket_index) = self.pocket {
                let piece = &PIECES[pocket_index - 1][rotation];
                let size_x = piece[0].len();
                for x in 0..(13 - size_x) {
                    legal_moves.push((x, rotation, true));
                }
            } else if let Some(&next_piece) = self.next_pieces.get(0) {
                let piece = &PIECES[next_piece - 1][rotation];
                let size_x = piece[0].len();
                for x in 0..(13 - size_x) {
                    legal_moves.push((x, rotation, true));
                }
            }
        }

        legal_moves
    }

    pub fn features(&self) -> Features {
        let mut holes = 0;
        let mut blocades = 0;
        let mut height = 0;

        for y in 1..22 {
            for x in 0..12 {
                if self.board[y][x] != 0 {
                    height += 22 - y as u32;
                }

                if self.board[y - 1][x] != 0 && self.board[y][x] == 0 {
                    holes += 1;
                    blocades += 1;

                    let mut k = 2;
                    let mut l = 1;

                    while y - k >= 0 && self.board[y - k][x] != 0 {
                        blocades += 1;
                        k += 1;
                    }

                    while y + l < 22 && self.board[y + l][x] == 0 {
                        holes += 1;
                        l += 1;
                    }
                }
            }
        }

        Features {
            holes: holes as f64,
            blocades: blocades as f64,
            height: height as f64,
            lines: self.lines as f64,
        }
    }

    pub fn apply_move(&self, x: usize, rotation: usize, swap: bool, gen_next_piece: bool) -> Option<Position> {
        let mut new_next_pieces = self.next_pieces.clone();
        let mut new_current_piece = new_next_pieces.pop_front().unwrap();

        let mut new_bag = self.bag.clone();
        let mut new_pocket = self.pocket.clone();

        if gen_next_piece {
            let rand = self.random_piece();
            new_next_pieces.push_front(rand.0);
            new_bag = rand.1;
        }

        let piece = {
            if !swap {
                &PIECES[self.current_piece - 1][rotation]
            } else if let Some(pocket_index) = self.pocket {
                new_pocket = Some(self.current_piece); 
                &PIECES[pocket_index - 1][rotation]
            } else {
                new_pocket = Some(self.current_piece); 
                let piece = &PIECES[new_current_piece - 1][rotation];
                new_current_piece = new_next_pieces.pop_front().unwrap();
                piece
            }
        };

        let size_x = piece[0].len();
        let size_y = piece.len();

        for y in 0..(23 - size_y) {
            for i in 0..size_x {
                for j in 0..size_y {
                    if y == 22 - size_y
                        || (x + i < 12 && piece[j][i] != 0 && self.board[j + y + 1][i + x] != 0)
                    {
                        let mut new_board = self.board.clone();
                        let mut new_score = self.score;

                        // Place the piece
                        for i in 0..size_x {
                            for j in 0..size_y {
                                if new_board[y + j][x + i] == 0 && piece[j][i] != 0 {
                                    new_board[y + j][x + i] = piece[j][i]
                                }
                            }
                        }

                        // Update lines
                        let mut line_count = 0;
                        for j in 0..22 {
                            let full_line = new_board[j].iter().all(|&cell| cell != 0);

                            if full_line {
                                line_count += 1;
                                new_board.remove(j);
                                new_board.insert(0, vec![0; 12]);
                            }
                        }

                        new_score += match line_count {
                            1 => 40,
                            2 => 100,
                            3 => 300,
                            4 => 1200,
                            _ => 0,
                        };

                        // Check game over
                        for i in 0..12 {
                            if new_board[0][i] != 0 {
                                return None;
                            }
                        }

                        return Some(Position::new(
                            new_current_piece,
                            new_next_pieces,
                            self.lines + line_count,
                            new_score,
                            new_board,
                            new_bag,
                            new_pocket,
                        ));
                    }
                }
            }
        }

        None
    }

    fn random_piece(&self) -> (usize, Vec<usize>) {
        let mut new_bag = self.bag.clone();

        if new_bag.is_empty() {
            new_bag = (1..8).collect();
            new_bag.shuffle(&mut thread_rng());
        }

        (new_bag.pop().unwrap(), new_bag)
    }

    fn get_hash(&self) -> u64 {
        let mut hash = 0;

        for x in 0..12 {
            for y in 0..22 {
                let piece = self.board[y][x] as usize;
                hash ^= ZOBRIST[(y * 22 + x) * (22 * 12) + piece];
            }
        }

        hash
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..22 {
            for x in 0..12 {
                write!(f, "{} ", self.board[y][x])?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl Default for Position {
    fn default() -> Self {
        let mut rng = rand::thread_rng();

        let mut bag: Vec<usize> = (1..8).collect();
        bag.shuffle(&mut rng);

        Self {
            current_piece: bag.pop().unwrap(),
            next_pieces: VecDeque::from([bag.pop().unwrap(); 4]),
            lines: 0,
            score: 0,
            board: vec![vec![0; 12]; 22],
            bag,
            pocket: None,
        }
    }
}

impl Hasher for Position {
    fn finish(&self) -> u64 {
        todo!()
    }

    fn write(&mut self, bytes: &[u8]) {
        todo!()
    }
}
