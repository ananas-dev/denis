use lazy_static::lazy_static;
use rand::Rng;
use rustc_hash::FxHashSet;
use std::{fmt, hash::Hasher};

const BOARD_WIDTH: usize = 10;
const BOARD_HEIGHT: usize = 22;

type Board = Vec<Vec<usize>>;
type Piece = Vec<Vec<usize>>;

lazy_static! {
    #[rustfmt::skip]
    static ref PIECES: Vec<Vec<Piece>> = vec![
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
                vec![0, 0, 3],
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
                vec![3, 0],
                vec![3, 0],
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

    static ref ROTATION_OFFSETS: Vec<Vec<(i32, i32)>> = vec![
        vec![(2, -2), (-2, 2)],
        vec![(0, 0)],
        vec![(0, -1), (0, 0), (1, 0), (-1, 1)],
        vec![(0, -1), (0, 0), (1, 0), (-1, 1)],
        vec![(1, -1), (-1, 1)],
        vec![(0, -1), (0, 0), (1, 0), (-1, 1)],
        vec![(1, -1), (-1, 1)],
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
    pub board: Board,
}

impl Position {
    pub fn new(
        current_piece: usize,
        next_piece: usize,
        lines: usize,
        score: i64,
        board: Board,
    ) -> Self {
        Position {
            current_piece,
            next_piece,
            lines,
            score,
            board,
        }
    }

    fn pathfind_open_air(
        &self,
        open_air_mask: &Board,
        piece_idx: usize,
        x: i32,
        y: i32,
        rot: i32,
        cache: &mut FxHashSet<(i32, i32, i32)>,
    ) -> bool {
        if cache.contains(&(x, y, rot)) {
            return false;
        }

        cache.insert((x, y, rot));

        let piece = &PIECES[piece_idx][rot as usize];
        let size_x = piece[0].len() as i32;
        let size_y = piece.len() as i32;

        if x < 0
            || x > BOARD_WIDTH as i32 - size_x
            || y < 0
            || y > BOARD_HEIGHT as i32 - size_y
            || check_colision(&self.board, piece, x as usize, y as usize)
        {
            return false;
        }

        if !check_colision(open_air_mask, piece, x as usize, y as usize) {
            return true;
        }

        if self.pathfind_open_air(open_air_mask, piece_idx, x + 1, y, rot, cache) {
            return true;
        }

        if self.pathfind_open_air(open_air_mask, piece_idx, x - 1, y, rot, cache) {
            return true;
        }

        if self.pathfind_open_air(open_air_mask, piece_idx, x, y - 1, rot, cache) {
            return true;
        }

        let rot_num = *&ROTATION_OFFSETS[piece_idx].len() as i32;
        let mut rot_offset =
            ROTATION_OFFSETS[piece_idx][(((rot - 1) % rot_num + rot_num) % rot_num) as usize];

        if self.pathfind_open_air(
            open_air_mask,
            piece_idx,
            x - rot_offset.0,
            y - rot_offset.1,
            ((rot - 1) % rot_num + rot_num) % rot_num,
            cache,
        ) {
            return true;
        }

        rot_offset = ROTATION_OFFSETS[piece_idx][rot as usize];

        if self.pathfind_open_air(
            open_air_mask,
            piece_idx,
            x + rot_offset.0,
            y + rot_offset.1,
            (rot + 1) % rot_num,
            cache,
        ) {
            return true;
        }

        false
    }

    pub fn gen_legal_moves(&self) -> Vec<(usize, usize, usize)> {
        let mut legal_moves = Vec::new();
        let mut open_air_mask = vec![vec![1; BOARD_WIDTH]; BOARD_HEIGHT];
        let mut cache = FxHashSet::default();

        for x in 0..BOARD_WIDTH {
            let mut y = 0;

            while y < BOARD_HEIGHT && self.board[y][x] == 0 {
                open_air_mask[y][x] = 0;
                y += 1;
            }
        }

        let piece_kind = &PIECES[self.current_piece - 1];

        for (rot, piece) in piece_kind.iter().enumerate() {
            let size_x = piece[0].len();
            let size_y = piece.len();
            for x in 0..(BOARD_WIDTH - size_x + 1) {
                for y in 0..(BOARD_HEIGHT - size_y + 1) {
                    if !check_colision(&self.board, piece, x, y)
                        && (y == BOARD_HEIGHT - size_y
                            || check_colision(&self.board, piece, x, y + 1))
                    {
                        if check_colision(&open_air_mask, piece, x, y) {
                            if self.pathfind_open_air(
                                &open_air_mask,
                                self.current_piece - 1,
                                x as i32,
                                y as i32,
                                rot as i32,
                                &mut cache,
                            ) {
                                legal_moves.push((x, y, rot));
                            }
                        } else {
                            legal_moves.push((x, y, rot));
                        }
                    }
                }
            }
        }

        legal_moves
    }

    pub fn features(&self) -> Features {
        let mut holes = 0;
        let mut heights: [f64; BOARD_WIDTH] = [0.; BOARD_WIDTH];

        for y in (1..BOARD_HEIGHT).rev() {
            for x in 0..BOARD_WIDTH {
                if self.board[y][x] != 0 {
                    heights[x] = (BOARD_HEIGHT - y) as f64;
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

        let aggregate_height = heights.iter().sum();

        Features {
            holes: holes as f64,
            aggregate_height,
            bumpiness,
            completed_lines: self.lines as f64,
        }
    }

    fn gen_piece(prev: usize) -> usize {
        let mut next_piece = rand::thread_rng().gen_range(1..8);

        if next_piece == prev {
            let reroll = rand::thread_rng().gen_range(1..7);

            if reroll < next_piece {
                next_piece = reroll;
            } else {
                next_piece = reroll + 1;
            }
        }

        next_piece
    }

    pub fn apply_move(&self, x: usize, y: usize, rot: usize) -> Option<Position> {
        let piece = &PIECES[self.current_piece - 1][rot];
        let size_x = piece[0].len();
        let size_y = piece.len();

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
            Position::gen_piece(self.next_piece),
            self.lines + line_count,
            new_score,
            new_board,
        ));
    }

    // fn get_hash(&self) -> u64 {
    //     let mut hash = 0;

    //     for x in 0..BOARD_WIDTH {
    //         for y in 0..BOARD_HEIGHT {
    //             let piece = self.board[y][x] as usize;
    //             // hash ^= ZOBRIST[(y * BOARD_HEIGHT + x) * (22 * BOARD_WIDTH) + piece];
    //         }
    //     }

    //     hash
    // }
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
        let current_piece = Position::gen_piece(0);

        Self {
            current_piece: Position::gen_piece(0),
            next_piece: Position::gen_piece(current_piece),
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

fn check_colision(board: &Board, piece: &Piece, x: usize, y: usize) -> bool {
    let size_x = piece[0].len();
    let size_y = piece.len();

    for i in 0..size_x {
        for j in 0..size_y {
            if board[y + j][x + i] != 0 && piece[j][i] != 0 {
                return true;
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_features() {
        let board = vec![
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 1, 1, 0, 0, 0, 0],
            vec![0, 1, 1, 1, 1, 1, 1, 0, 0, 1],
            vec![0, 1, 1, 0, 1, 1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            vec![1, 1, 1, 0, 1, 1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        ];

        let feat = Position::new(1, 1, 0, 0, board).features();

        assert_eq!(feat.bumpiness, 6.);
        assert_eq!(feat.aggregate_height, 48.);
        assert_eq!(feat.holes, 2.);
    }

    #[test]
    fn test_gen_moves() {
        let board = vec![
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![1, 0, 0, 1, 0, 0, 0, 0, 0, 0],
            vec![1, 0, 0, 0, 1, 1, 1, 1, 1, 1],
            vec![1, 1, 0, 1, 1, 1, 1, 1, 1, 1],
        ];

        let moves = Position::new(6, 1, 0, 0, board).gen_legal_moves();

        let expected = vec![
            (0, 18, 0),
            (1, 18, 0),
            (1, 20, 0),
            (2, 17, 0),
            (3, 18, 0),
            (4, 18, 0),
            (5, 18, 0),
            (6, 18, 0),
            (7, 18, 0),
            (0, 17, 1),
            (1, 19, 1),
            (2, 16, 1),
            (3, 17, 1),
            (4, 17, 1),
            (5, 17, 1),
            (6, 17, 1),
            (7, 17, 1),
            (8, 17, 1),
            (0, 17, 2),
            (1, 17, 2),
            (1, 19, 2),
            (2, 17, 2),
            (3, 17, 2),
            (4, 18, 2),
            (5, 18, 2),
            (6, 18, 2),
            (7, 18, 2),
            (0, 16, 3),
            (1, 18, 3),
            (2, 17, 3),
            (2, 19, 3),
            (3, 16, 3),
            (4, 17, 3),
            (5, 17, 3),
            (6, 17, 3),
            (7, 17, 3),
            (8, 17, 3),
        ];

        assert_eq!(moves, expected)
    }

    #[test]
    fn stack_overflow() {
        let board = vec![
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 5, 0, 1, 1, 1, 1, 0],
            vec![0, 0, 0, 5, 5, 1, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 5, 1, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 4, 1, 0, 0, 0, 0],
            vec![2, 2, 0, 0, 4, 1, 7, 0, 7, 0],
            vec![2, 2, 0, 0, 4, 4, 7, 7, 7, 7],
        ];

        Position::new(6, 1, 0, 0, board).gen_legal_moves();
    }
}
