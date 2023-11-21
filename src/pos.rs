use lazy_static::lazy_static;
use rand::Rng;
use std::{fmt, hash::Hasher};

const BOARD_WIDTH: usize = 10;
const BOARD_HEIGHT: usize = 22;

lazy_static! {
    #[rustfmt::skip]
    static ref PIECES: Vec<Vec<Vec<Vec<usize>>>> = vec![
        vec![
            vec![
                vec![1, 1, 1, 1]
            ],
            vec![
                vec![1],
                vec![1],
                vec![1],
                vec![1],
            ],
        ],
        vec![
            vec![
                vec![2, 2],
                vec![2, 2]
            ]
        ],
        vec![
            vec![
                vec![3, 3, 3],
                vec![0, 3, 1],
            ],
            vec![
                vec![0, 3],
                vec![0, 3],
                vec![3, 3],
            ],
            vec![
                vec![3, 0, 0],
                vec![3, 3, 3],
            ],
            vec![
                vec![3, 3],
                vec![3, 3],
                vec![3, 3],
            ],
        ],
        vec![
            vec![
                vec![4, 4, 4],
                vec![4, 0, 0],
            ],
            vec![
                vec![4, 4],
                vec![0, 4],
                vec![0, 4],
            ],
            vec![
                vec![0, 0, 4],
                vec![4, 4, 4],
            ],
            vec![
                vec![4, 0],
                vec![4, 0],
                vec![4, 4],
            ],
        ],
        vec![
            vec![
                vec![0, 5, 5],
                vec![5, 5, 0],
            ],
            vec![
                vec![5, 0],
                vec![5, 5],
                vec![0, 5],
            ],
        ],
        vec![
            vec![
                vec![6, 6, 6],
                vec![0, 6, 0],
            ],
            vec![
                vec![0, 6],
                vec![6, 6],
                vec![0, 6],
            ],
            vec![
                vec![0, 6, 0],
                vec![6, 6, 6],
            ],
            vec![
                vec![6, 0],
                vec![6, 6],
                vec![6, 0],
            ],
        ],
        vec![
            vec![
                vec![7, 7, 0],
                vec![0, 7, 7],
            ],
            vec![
                vec![0, 7],
                vec![7, 7],
                vec![7, 0],
            ],
        ],
    ];
}

#[derive(Debug)]
pub struct Features {
    pub holes: f64,
    pub bumpiness: f64,
    pub aggregate_height: f64,
    pub completed_lines: f64,
}

#[derive(Debug)]
pub struct Position {
    pub score: i64,
    pub current_piece: usize,
    pub next_piece: usize,
    pub lines: usize,
    pub board: Vec<Vec<usize>>,
}

impl Position {
    pub fn new(
        current_piece: usize,
        next_piece: usize,
        lines: usize,
        score: i64,
        board: Vec<Vec<usize>>,
    ) -> Self {
        Position {
            current_piece,
            next_piece,
            lines,
            score,
            board,
        }
    }

    pub fn gen_legal_moves(&self) -> Vec<(usize, usize)> {
        let mut legal_moves = Vec::new();

        let piece_type = &PIECES[self.current_piece - 1];

        for (rotation, piece) in piece_type.iter().enumerate() {
            let size_x = piece[0].len();
            for x in 0..(BOARD_WIDTH + 1 - size_x) {
                legal_moves.push((x, rotation));
            }
        }

        legal_moves
    }

    pub fn features(&self) -> Features {
        let mut holes = 0;
        let mut aggregate_height = 0;
        let mut heights: [f64; BOARD_WIDTH] = [0.; BOARD_WIDTH];

        for y in 1..BOARD_HEIGHT {
            for x in 0..BOARD_WIDTH {
                if self.board[y][x] != 0 {
                    aggregate_height += BOARD_HEIGHT - y;
                    heights[x] += 1.;
                }

                if self.board[y - 1][x] != 0 && self.board[y][x] == 0 {
                    holes += 1;

                    let mut l = 1;

                    while y + l < BOARD_HEIGHT && self.board[y + l][x] == 0 {
                        holes += 1;
                        l += 1;
                    }
                }
            }
        }

        let bumpiness = heights
            .windows(2)
            .map(|window| (window[0] - window[1]).abs())
            .sum();

        Features {
            holes: holes as f64,
            aggregate_height: aggregate_height as f64,
            bumpiness,
            completed_lines: self.lines as f64,
        }
    }

    pub fn apply_move(&self, x: usize, rotation: usize) -> Option<Position> {
        let piece = &PIECES[self.current_piece - 1][rotation];
        let size_x = piece[0].len();
        let size_y = piece.len();

        for y in 0..((BOARD_HEIGHT + 1) - size_y) {
            for i in 0..size_x {
                for j in 0..size_y {
                    if y == BOARD_HEIGHT - size_y
                        || (x + i < BOARD_WIDTH
                            && piece[j][i] != 0
                            && self.board[j + y + 1][i + x] != 0)
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
                        for j in 0..BOARD_HEIGHT {
                            let full_line = new_board[j].iter().all(|&cell| cell != 0);

                            if full_line {
                                line_count += 1;
                                new_board.remove(j);
                                new_board.insert(0, vec![0; BOARD_WIDTH]);
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
                        for i in 0..BOARD_WIDTH {
                            if new_board[0][i] != 0 || new_board[1][i] != 0 {
                                return None;
                            }
                        }

                        return Some(Position::new(
                            self.next_piece,
                            rand::thread_rng().gen_range(1..8),
                            self.lines + line_count,
                            new_score,
                            new_board,
                        ));
                    }
                }
            }
        }

        None
    }

    fn get_hash(&self) -> u64 {
        let mut hash = 0;

        for x in 0..BOARD_WIDTH {
            for y in 0..BOARD_HEIGHT {
                let piece = self.board[y][x] as usize;
                // hash ^= ZOBRIST[(y * BOARD_HEIGHT + x) * (22 * BOARD_WIDTH) + piece];
            }
        }

        hash
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..BOARD_HEIGHT {
            for x in 0..BOARD_WIDTH {
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

        Self {
            current_piece: rng.gen_range(1..8),
            next_piece: rng.gen_range(1..8),
            lines: 0,
            score: 0,
            board: vec![vec![0; BOARD_WIDTH]; BOARD_HEIGHT],
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
