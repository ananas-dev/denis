use arrayvec::ArrayVec;
use lazy_static::lazy_static;
use rand::{
    distributions::Distribution,
    Rng,
    rngs::SmallRng, SeedableRng,
};
use rustc_hash::{FxHashMap, FxHashSet};
use serde::Serialize;
use std::{
    cmp::{self, Ordering},
    collections::BinaryHeap,
    fmt,
    str::FromStr,
};

const BOARD_WIDTH: usize = 10;
const BOARD_HEIGHT: usize = 22;
const PIECE_NUMBER: usize = 7;

type Board<T> = [[T; BOARD_WIDTH]; BOARD_HEIGHT];
type Piece = Vec<Vec<Color>>;

macro_rules! piece {
    ($color:expr, $( $vec:expr ),* ) => {
        {
            let mut result = Vec::new();

            $(
                let mut new_piece = Vec::new();
                for row in $vec.iter() {
                    let mut new_row = Vec::new();
                    for cell in row.iter() {
                        match cell {
                            0 => new_row.push(Color::Empty),
                            1 => new_row.push($color),
                            _ => panic!("Invalid value in the input vector"),
                        }
                    }
                    new_piece.push(new_row);
                }
                result.push(new_piece);
            )*

            result
        }
    };
}

lazy_static! {
    #[rustfmt::skip]
    static ref PIECES: [Vec<Piece>; PIECE_NUMBER] = [
        piece!(
            Color::I,
            vec![
                vec![1, 1, 1, 1]
            ],
            vec![
                vec![1],
                vec![1],
                vec![1],
                vec![1],
            ]
        ),
        piece!(
            Color::O,
            vec![
                vec![1, 1],
                vec![1, 1]
            ]
        ),
        piece!(
            Color::J,
            vec![
                vec![1, 1, 1],
                vec![0, 0, 1],
            ],
            vec![
                vec![0, 1],
                vec![0, 1],
                vec![1, 1],
            ],
            vec![
                vec![1, 0, 0],
                vec![1, 1, 1],
            ],
            vec![
                vec![1, 1],
                vec![1, 0],
                vec![1, 0],
            ]
        ),
        piece!(
            Color::L,
            vec![
                vec![1, 1, 1],
                vec![1, 0, 0],
            ],
            vec![
                vec![1, 1],
                vec![0, 1],
                vec![0, 1],
            ],
            vec![
                vec![0, 0, 1],
                vec![1, 1, 1],
            ],
            vec![
                vec![1, 0],
                vec![1, 0],
                vec![1, 1],
            ]
        ),
        piece!(
            Color::S,
            vec![
                vec![0, 1, 1],
                vec![1, 1, 0],
            ],
            vec![
                vec![1, 0],
                vec![1, 1],
                vec![0, 1],
            ]
        ),
        piece!(
            Color::T,
            vec![
                vec![1, 1, 1],
                vec![0, 1, 0],
            ],
            vec![
                vec![0, 1],
                vec![1, 1],
                vec![0, 1],
            ],
            vec![
                vec![0, 1, 0],
                vec![1, 1, 1],
            ],
            vec![
                vec![1, 0],
                vec![1, 1],
                vec![1, 0],
            ]
        ),
        piece!(
            Color::Z,
            vec![
                vec![1, 1, 0],
                vec![0, 1, 1],
            ],
            vec![
                vec![0, 1],
                vec![1, 1],
                vec![1, 0],
            ]
        ),
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
        let mut rng = SmallRng::seed_from_u64(0xDEADBEEF12345678);

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

trait Cell {
    fn is_empty(&self) -> bool;
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum Color {
    I,
    O,
    J,
    L,
    S,
    T,
    Z,
    Empty,
}

impl From<u8> for Color {
    fn from(value: u8) -> Self {
        match value {
            0 => Color::I,
            1 => Color::O,
            2 => Color::J,
            3 => Color::L,
            4 => Color::S,
            5 => Color::T,
            6 => Color::Z,
            _ => Color::Empty,
        }
    }
}

impl Cell for Color {
    fn is_empty(&self) -> bool {
        *self == Color::Empty
    }
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
pub enum Mask {
    Unset,
    Set,
}

impl Cell for Mask {
    fn is_empty(&self) -> bool {
        *self == Mask::Unset
    }
}

// TODO: custom error type
impl TryFrom<char> for Color {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'I' => Ok(Color::I),
            'O' => Ok(Color::O),
            'J' => Ok(Color::J),
            'L' => Ok(Color::L),
            'S' => Ok(Color::S),
            'T' => Ok(Color::T),
            'Z' => Ok(Color::Z),
            _ => Err(()),
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Color::I => 'I',
                Color::O => 'O',
                Color::J => 'J',
                Color::L => 'L',
                Color::S => 'S',
                Color::T => 'T',
                Color::Z => 'Z',
                _ => ' ',
            }
        )?;

        Ok(())
    }
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
    pub current_piece: Color,
    pub next_piece: Color,
    pub board: Board<Color>,
    pub hash: u64,
}

impl Position {
    pub fn new(
        current_piece: Color,
        next_piece: Color,
        score: i64,
        board: Board<Color>,
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
        let rot_num = ROTATION_OFFSETS[self.current_piece as usize].len() as i32;

        let mut goal_mv = None;

        let start = SPAWNS[self.current_piece as usize];
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
                &PIECES[self.current_piece as usize][wrap_rot(current.dest.2, rot_num) as usize];

            let mut move_list: ArrayVec<Move, 5> = ArrayVec::new();

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
            let mut rot_offset = ROTATION_OFFSETS[self.current_piece as usize][rot];

            piece = &PIECES[self.current_piece as usize][rot];

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
            rot_offset = ROTATION_OFFSETS[self.current_piece as usize][rot];
            piece = &PIECES[self.current_piece as usize][rot];

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
        open_air_mask: &Board<Mask>,
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
        let mut rot_offset = ROTATION_OFFSETS[piece_idx][wrap_rot(rot - 1, rot_num) as usize];

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
        let mut open_air_mask = [[Mask::Set; BOARD_WIDTH]; BOARD_HEIGHT];
        let mut cache = FxHashSet::default();

        for x in 0..BOARD_WIDTH {
            let mut y = 0;

            while y < BOARD_HEIGHT && self.board[y][x].is_empty() {
                open_air_mask[y][x] = Mask::Unset;
                y += 1;
            }
        }

        let piece_kind = &PIECES[self.current_piece as usize];

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
                                self.current_piece as usize,
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

    pub fn features(&self) -> Features {
        let mut holes = 0;
        let mut heights: [f64; BOARD_WIDTH] = [0.; BOARD_WIDTH];

        for y in (1..BOARD_HEIGHT).rev() {
            for x in 0..BOARD_WIDTH {
                if !self.board[y][x].is_empty() {
                    heights[x] = (BOARD_HEIGHT - y) as f64;
                }

                if !self.board[y - 1][x].is_empty() && self.board[y][x].is_empty() {
                    holes += 1;

                    let mut l = 1;

                    while y + l < BOARD_HEIGHT && self.board[y + l][x].is_empty() {
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

    pub fn apply_move(&self, x: usize, y: usize, rot: usize, gen_next: bool) -> Option<Position> {
        let piece = &PIECES[self.current_piece as usize][rot];
        let size_x = piece[0].len();
        let size_y = piece.len();

        let mut new_board = self.board.clone();
        let mut new_score = self.score;
        let mut new_hash = self.hash;

        // Place the piece
        for i in 0..size_x {
            for j in 0..size_y {
                if new_board[y + j][x + i].is_empty() && !piece[j][i].is_empty() {
                    let piece_type = piece[j][i];
                    new_board[y + j][x + i] = piece_type;
                    new_hash ^= ZOBRISTS[y + j][x + i][piece_type as usize];
                }
            }
        }

        // Update lines
        let mut line_count = 0;
        for j in 0..BOARD_HEIGHT {
            let full_line = new_board[j].iter().all(|&cell| !cell.is_empty());

            if full_line {
                let new_board_copy = new_board.clone();
                line_count += 1;
                for y in 0..j {
                    for x in 0..BOARD_WIDTH {
                        let piece_type = new_board_copy[y][x];
                        let old_piece_type = new_board_copy[y + 1][x];

                        if !old_piece_type.is_empty() {
                            new_hash ^= ZOBRISTS[y + 1][x][old_piece_type as usize];
                        }

                        if !piece_type.is_empty() {
                            new_hash ^= ZOBRISTS[y + 1][x][piece_type as usize];
                        }

                        new_board[y + 1][x] = piece_type;
                    }
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
            if !new_board[0][i].is_empty() || !new_board[1][i].is_empty() {
                return None;
            }
        }

        return Some(Position::new(
            self.next_piece,
            if gen_next {
                self.sample(&mut rand::thread_rng())
            } else {
                Color::Empty
            },
            new_score,
            new_board,
            new_hash,
        ));
    }
}

impl Default for Position {
    fn default() -> Self {
        let board = [[Color::Empty; BOARD_WIDTH]; BOARD_HEIGHT];

        // TODO: Fix random
        Self {
            current_piece: Color::I,
            next_piece: Color::J,
            score: 0,
            hash: hash_board(&board),
            board,
        }
    }
}

impl Distribution<Color> for Position {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Color {
        let next_piece = rng.gen_range(0..7);

        if next_piece == self.next_piece as u8 {
            let reroll = rng.gen_range(0..6);

            if reroll < next_piece {
                Color::from(reroll)
            } else {
                Color::from(reroll + 1)
            }
        } else {
            Color::from(next_piece)
        }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut empty_cells = 0;
        for y in 0..BOARD_HEIGHT {
            for x in 0..BOARD_WIDTH {
                if !self.board[y][x].is_empty() && empty_cells > 0 && empty_cells < 10 {
                    write!(f, "{}", empty_cells)?;
                    empty_cells = 0;
                }

                if !self.board[y][x].is_empty() {
                    write!(f, "{}", self.board[y][x])?;
                } else {
                    empty_cells += 1;
                }
            }

            if empty_cells > 0 && empty_cells < 10 {
                write!(f, "{}", empty_cells)?;
            }

            write!(f, "/")?;
            empty_cells = 0;
        }

        write!(
            f,
            " {} {} {}",
            self.current_piece, self.next_piece, self.score
        )?;

        Ok(())
    }
}

// TODO: overflow error handling
impl FromStr for Position {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut board = [[Color::Empty; BOARD_WIDTH]; BOARD_HEIGHT];
        let mut curr_x = 0;
        let mut curr_y = 0;

        let tokens: Vec<&str> = s.split(' ').collect();
        if tokens.len() < 3 {
            return Err(());
        }

        let board_tok = tokens[0];
        let curr_piece_tok = tokens[1];
        let next_piece_tok = tokens[2];
        let score_tok = tokens[3];

        for x in board_tok.chars() {
            match x {
                '/' => {
                    curr_x = 0;
                    curr_y += 1;
                }
                '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    curr_x += (x as usize) - ('0' as usize)
                }
                _ => {
                    let piece = Color::try_from(x)?;
                    board[curr_y][curr_x] = piece;
                    curr_x += 1;
                }
            }
        }

        let current_piece = Color::try_from(curr_piece_tok.chars().next().ok_or(())?)?;
        let next_piece = Color::try_from(next_piece_tok.chars().next().ok_or(())?)?;
        let score = i64::from_str(score_tok).map_err(|_| ())?;

        let hash = hash_board(&board);
        Ok(Position::new(current_piece, next_piece, score, board, hash))
    }
}

fn check_colision<T: Cell>(board: &Board<T>, piece: &Piece, x: i32, y: i32) -> bool {
    let size_x = piece[0].len() as i32;
    let size_y = piece.len() as i32;

    if x < 0 || x > BOARD_WIDTH as i32 - size_x || y < 0 || y > BOARD_HEIGHT as i32 - size_y {
        return true;
    }

    for i in 0..size_x {
        for j in 0..size_y {
            if !board[(y + j) as usize][(x + i) as usize].is_empty()
                && !piece[j as usize][i as usize].is_empty()
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

fn hash_board(board: &Board<Color>) -> u64 {
    let mut hash = 0;

    for x in 0..BOARD_WIDTH {
        for y in 0..BOARD_HEIGHT {
            let piece = board[y][x];
            if !piece.is_empty() {
                hash ^= ZOBRISTS[y][x][piece as usize];
            }
        }
    }

    hash
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_tpn() {
        let pos = Position::default();
        assert_eq!(
            pos.to_string(),
            format!(
                "////////////////////// {} {} 0",
                pos.current_piece, pos.next_piece
            )
        );
    }

    #[test]
    fn test_hash_empty() {
        let pos1 = Position::default();
        let pos2 = Position::from_str(&pos1.to_string()).unwrap();

        assert!(pos1.hash == pos2.hash)
    }
}
