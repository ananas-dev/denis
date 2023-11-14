use lazy_static::lazy_static;
use rand::Rng;
use std::fmt;

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
    pub next_piece: usize,
    pub lines: usize,
    pub board: Vec<u8>,
}

impl Position {
    pub fn new(
        current_piece: usize,
        next_piece: usize,
        lines: usize,
        score: i64,
        board: Vec<u8>,
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

        for rotation in 0..4 {
            let piece = &PIECES[self.current_piece - 1][rotation];
            let size_x = piece[0].len();
            for x in 0..(13 - size_x) {
                legal_moves.push((x, rotation));
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
                if self.board[y * 12 + x] != 0 {
                    height += 22 - y as u32;
                }

                if self.board[(y - 1) * 12 + x] != 0 && self.board[y * 12 + x] == 0 {
                    holes += 1;
                    blocades += 1;

                    let mut k = 2;
                    let mut l = 1;

                    while y - k >= 0 && self.board[(y - k) * 12 + x] != 0 {
                        blocades += 1;
                        k += 1;
                    }

                    while y + l < 22 && self.board[(y + l) * 12 + x] == 0 {
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

    pub fn apply_move(&self, x: usize, rotation: usize) -> Option<Position> {
        let piece = &PIECES[self.current_piece - 1][rotation];
        let size_x = piece[0].len();
        let size_y = piece.len();

        for y in 0..(23 - size_y) {
            for i in 0..size_x {
                for j in 0..size_y {
                    if y == 22 - size_y
                        || (x + i < 12
                            && piece[j][i] != 0
                            && self.board[(j + y + 1) * 12 + (i + x)] != 0)
                    {
                        let mut new_board = self.board.clone();
                        let mut new_score = self.score;

                        // Place the piece
                        for i in 0..size_x {
                            for j in 0..size_y {
                                if new_board[(y + j) * 12 + (x + i)] == 0 && piece[j][i] != 0 {
                                    new_board[(y + j) * 12 + (x + i)] = piece[j][i]
                                }
                            }
                        }

                        // Update lines
                        let mut line_count: usize = 0;
                        for j in 0..22 {
                            let mut count = 0;
                            for i in 0..12 {
                                if new_board[j * 12 + i] != 0 {
                                    count += 1;
                                }
                            }

                            if count == 12 {
                                line_count += 1;

                                for k in 0..j {
                                    for l in 0..12 {
                                        new_board[(k + 1) * 12 + l] = self.board[k * 12 + l]
                                    }
                                }

                                // Clear top line
                                for l in 0..12 {
                                    new_board[l] = 0
                                }
                            }
                        }

                        new_score += 1000 * line_count as i64;

                        // if y < 5 - size_y {
                        //     new_score -= 500
                        // }

                        // Check game over
                        for i in 0..12 {
                            if new_board[i] != 0 {
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
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..22 {
            for x in 0..12 {
                write!(f, "{} ", self.board[y * 12 + x])?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl Default for Position {
    fn default() -> Self {
        // TODO: maybe pass the rng
        let mut rng = rand::thread_rng();

        Self {
            current_piece: rng.gen_range(1..8),
            next_piece: rng.gen_range(1..8),
            lines: 0,
            score: 0,
            board: vec![0; 22 * 12],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_features() {
        let ctr_board = vec![
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 0, 0, 0,
            0, 4, 0, 0, 0, 0, 0, 0, 7, 0, 3, 0, 4, 4, 0, 0, 0, 0, 0, 0, 2, 0, 3, 3, 4, 0, 0, 0, 0,
            0, 0, 0, 2, 0, 0, 3, 5, 0, 0, 0, 0, 0, 0, 2, 2, 0, 0, 5, 5, 0, 0, 0, 0, 0, 0, 0, 3, 0,
            6, 6, 5, 0, 0, 0, 0, 0, 0, 0, 3, 3, 6, 6, 0, 0, 0, 0, 0, 0, 5, 0, 0, 3, 4, 0, 0, 0, 0,
            0, 0, 5, 5, 0, 0, 4, 4, 0, 0, 0, 0, 0, 1, 1, 5, 2, 3, 4, 0, 0, 0, 0, 0, 0, 0, 1, 0, 2,
            3, 3, 0, 0, 0, 0, 0, 0, 0, 1, 2, 2, 0, 3, 4, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 4, 4, 0, 0,
            0, 0, 0, 0, 0, 1, 0, 0, 4, 4, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 4, 4, 6, 6, 0, 0, 0, 2, 0,
            5, 0, 0, 4, 3, 6, 6, 2, 0, 0, 2, 5, 5, 0, 0, 0, 3, 3, 0, 2, 0, 2, 2, 0, 5, 0, 0, 0, 0,
            3, 2, 2,
        ];

        let pos = Position::new(0, 0, 0, 0, ctr_board);
        let feat = pos.features();

        assert_eq!(feat.holes, 65.);
        assert_eq!(feat.blocades, 61.);
        assert_eq!(feat.height, 700.);
    }
}
