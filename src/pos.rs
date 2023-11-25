use lazy_static::lazy_static;
use rand::Rng;
use rustc_hash::{FxHashMap, FxHashSet};
use serde::Serialize;
use std::{
    cmp::{self, Ordering},
    collections::BinaryHeap,
    fmt,
};

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

    static ref SPAWNS: Vec<(i32, i32, i32)> = vec![(3, 1, 0), (4, 0, 0), (3, 0, 0), (3, 0, 0), (3, 0, 0), (3, 0, 0), (3, 0, 0)];

    static ref ZOBRISTS: Vec<Vec<Vec<u64>>> = {
        let mut rng = rand::thread_rng();

        let mut board = Vec::new();
        for _ in  0..BOARD_HEIGHT {
            let mut row = Vec::new();
            for _ in 0..BOARD_WIDTH {
                let mut coord = Vec::new();
                for _ in 0..7 {
                    coord.push(rng.gen::<u64>());
                }
                row.push(coord);
            }
            board.push(row);
        }

        board
    };
}

#[derive(Debug)]
pub struct Features {
    pub holes: f64,
    pub bumpiness: f64,
    pub aggregate_height: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Hash, Serialize)]
pub enum Action {
    MoveLeft,
    MoveRight,
    SoftDrop,
    RotateCounterclockwise,
    RotateClockwise,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Hash)]
struct Move {
    action: Action,
    dest: (i32, i32, i32),
}

impl Move {
    fn new(action: Action, dest: (i32, i32, i32)) -> Move {
        Move { action, dest }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct OrderedMove {
    mv: Move,
    priority: i32,
}

impl OrderedMove {
    fn new(mv: Move, priority: i32) -> OrderedMove {
        OrderedMove { mv, priority }
    }
}

impl Ord for OrderedMove {
    fn cmp(&self, other: &Self) -> Ordering {
        other.priority.cmp(&self.priority)
    }
}

impl PartialOrd for OrderedMove {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
pub struct Position {
    pub score: i64,
    pub current_piece: usize,
    pub next_piece: usize,
    pub board: Board,
    pub hash: u64,
}

impl Position {
    pub fn new(
        current_piece: usize,
        next_piece: usize,
        score: i64,
        board: Board,
        hash: u64,
    ) -> Self {
        Position {
            current_piece,
            next_piece,
            score,
            board,
            hash,
        }
    }

    pub fn path(&self, goal: (i32, i32, i32)) -> Vec<Action> {
        let rot_num = ROTATION_OFFSETS[self.current_piece - 1].len() as i32;

        let mut goal_mv = None;

        let start = SPAWNS[self.current_piece - 1];
        let start_move = OrderedMove::new(Move::new(Action::None, start), 0);

        let mut frontier = BinaryHeap::new();
        frontier.push(start_move);

        let mut came_from = FxHashMap::default();
        let mut cost_so_far = FxHashMap::default();
        came_from.insert(start, None);
        cost_so_far.insert(start, 0);

        while !frontier.is_empty() {
            let current = frontier.pop().unwrap().mv;
            let dest = current.dest;

            if dest == goal {
                goal_mv = Some(current);
                break;
            }

            let mut piece =
                &PIECES[self.current_piece - 1][wrap_rot(current.dest.2, rot_num) as usize];

            let mut move_list = Vec::new();

            if !check_colision(&self.board, piece, dest.0 - 1, dest.1) {
                move_list.push(Move::new(Action::MoveLeft, (dest.0 - 1, dest.1, dest.2)));
            }

            if !check_colision(&self.board, piece, dest.0 + 1, dest.1) {
                move_list.push(Move::new(Action::MoveRight, (dest.0 + 1, dest.1, dest.2)));
            }

            if !check_colision(&self.board, piece, dest.0, dest.1 + 1) {
                move_list.push(Move::new(Action::SoftDrop, (dest.0, dest.1 + 1, dest.2)));
            }

            let mut rot = wrap_rot(dest.2 - 1, rot_num) as usize;
            let mut rot_offset = ROTATION_OFFSETS[self.current_piece - 1][rot];

            piece = &PIECES[self.current_piece - 1][rot];

            if !check_colision(
                &self.board,
                piece,
                dest.0 - rot_offset.0,
                dest.1 - rot_offset.1,
            ) {
                move_list.push(Move::new(
                    Action::RotateCounterclockwise,
                    (
                        dest.0 - rot_offset.0,
                        dest.1 - rot_offset.1,
                        wrap_rot(dest.2 - 1, rot_num),
                    ),
                ));
            }

            rot = wrap_rot(dest.2, rot_num) as usize;
            rot_offset = ROTATION_OFFSETS[self.current_piece - 1][rot];
            piece = &PIECES[self.current_piece - 1][rot];

            if !check_colision(
                &self.board,
                piece,
                dest.0 + rot_offset.0,
                dest.1 + rot_offset.1,
            ) {
                move_list.push(Move::new(
                    Action::RotateClockwise,
                    (
                        dest.0 + rot_offset.0,
                        dest.1 + rot_offset.1,
                        wrap_rot(dest.2 + 1, rot_num),
                    ),
                ));
            }

            for &next in move_list.iter() {
                // Lower costs to higher actions
                let c = match next.action {
                    Action::SoftDrop => 1,
                    _ => next.dest.1 + 1,
                };

                let new_cost = cost_so_far.get(&current.dest).unwrap() + c;
                if !cost_so_far.contains_key(&next.dest)
                    || new_cost < *cost_so_far.get(&next.dest).unwrap()
                {
                    cost_so_far.insert(next.dest, new_cost);
                    let priority = new_cost + proximity(goal, next.dest, rot_num);
                    frontier.push(OrderedMove::new(next, priority));
                    came_from.insert(next.dest, Some(current));
                }
            }
        }

        let mut current = goal;
        let mut path = Vec::new();
        let mut mv: Option<Move> = goal_mv;
        while current != start {
            if let Some(mv) = mv {
                path.push(mv.action);
            }
            mv = *came_from.get(&current).unwrap();
            current = mv.unwrap().dest;
        }

        path.reverse();

        path
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

        if check_colision(&self.board, piece, x, y) {
            return false;
        }

        if !check_colision(open_air_mask, piece, x, y) {
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

    pub fn legal_moves(&self) -> Vec<(usize, usize, usize)> {
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
                    if !check_colision(&self.board, piece, x as i32, y as i32)
                        && check_colision(&self.board, piece, x as i32, (y + 1) as i32)
                    {
                        if check_colision(&open_air_mask, piece, x as i32, y as i32) {
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

                            cache.clear();
                        } else {
                            legal_moves.push((x, y, rot));
                        }
                    }
                }
            }
        }

        legal_moves
    }

    pub fn hash(&self) -> u64 {
        self.hash
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

    pub fn apply_move(&self, x: usize, y: usize, rot: usize, gen_next: bool) -> Option<Position> {
        let piece = &PIECES[self.current_piece - 1][rot];
        let size_x = piece[0].len();
        let size_y = piece.len();

        let mut new_board = self.board.clone();
        let mut new_score = self.score;
        let mut new_hash = self.hash;

        // Place the piece
        for i in 0..size_x {
            for j in 0..size_y {
                if new_board[y + j][x + i] == 0 && piece[j][i] != 0 {
                    let piece_type = piece[j][i];
                    new_board[y + j][x + i] = piece_type;
                    new_hash ^= ZOBRISTS[y + j][x + i][piece_type - 1];
                }
            }
        }

        // Update lines
        let mut line_count = 0;
        for j in 0..BOARD_HEIGHT {
            let full_line = new_board[j].iter().all(|&cell| cell != 0);

            if full_line {
                let new_board_copy = new_board.clone();
                line_count += 1;
                for y in 0..j {
                    for x in 0..BOARD_WIDTH {
                        let piece_type = self.board[y][x];
                        let old_piece_type = self.board[y+1][x];

                        if old_piece_type != 0 {
                            new_hash ^= ZOBRISTS[y+1][x][old_piece_type - 1];
                        }

                        if piece_type != 0 {
                            new_hash ^= ZOBRISTS[y+1][x][piece_type - 1];
                        }

                        new_board[y + 1][x] = new_board_copy[y][x];
                    }
                }

                for x in 0..BOARD_WIDTH {
                    new_hash ^= ZOBRISTS[y][x][0];
                }
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
            if gen_next {
                Position::gen_piece(self.next_piece)
            } else {
                0
            },
            new_score,
            new_board,
            new_hash,
        ));
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
        let current_piece = Position::gen_piece(0);
        let board = vec![vec![0; BOARD_WIDTH]; BOARD_HEIGHT];

        Self {
            current_piece: Position::gen_piece(0),
            next_piece: Position::gen_piece(current_piece),
            score: 0,
            hash: hash_board(&board),
            board,
        }
    }
}

fn check_colision(board: &Board, piece: &Piece, x: i32, y: i32) -> bool {
    let size_x = piece[0].len() as i32;
    let size_y = piece.len() as i32;

    if x < 0 || x > BOARD_WIDTH as i32 - size_x || y < 0 || y > BOARD_HEIGHT as i32 - size_y {
        return true;
    }

    for i in 0..size_x {
        for j in 0..size_y {
            if board[(y + j) as usize][(x + i) as usize] != 0 && piece[j as usize][i as usize] != 0
            {
                return true;
            }
        }
    }

    false
}

fn wrap_rot(rot: i32, dim: i32) -> i32 {
    (rot % dim + dim) % dim
}

fn proximity(a: (i32, i32, i32), b: (i32, i32, i32), rot_dim: i32) -> i32 {
    (a.0 - b.0).abs() + cmp::min(wrap_rot(a.2 - b.2, rot_dim), wrap_rot(b.2 - a.2, rot_dim))
}

fn hash_board(board: &Board) -> u64 {
    let mut hash = 0;

    for x in 0..BOARD_WIDTH {
        for y in 0..BOARD_HEIGHT {
            let piece = board[y][x] as usize;
            if piece != 0 {
                hash ^= ZOBRISTS[y][x][piece - 1];
            }
        }
    }

    hash
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

        let feat = Position::new(1, 1, 0, board, 0).features();

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

        let moves = Position::new(6, 1, 0, board, 0).legal_moves();

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

        Position::new(6, 1, 0, board, 0).legal_moves();
    }
}
