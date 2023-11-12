use pyo3::prelude::*;
use std::{error::Error, ops::Range};

use rand::{Rng, rngs::ThreadRng};

#[derive(Clone)]

#[pyclass(get_all)]
struct Board{
    board: [[usize; 12]; 22]
}

#[derive(PartialEq, Eq, Clone)]
#[pyclass]
struct Block([[usize;4];4] , usize);

#[pymethods]
impl Block{
    #[classattr]
    const L: Self = Self([
        [0,0,1,0],
        [1,1,1,0],
        [0,0,0,0],
        [0,0,0,0]
    ], 0);
    #[classattr]
    const J: Self = Self([
        [1,0,0,0],
        [1,1,1,0],
        [0,0,0,0],
        [0,0,0,0]
    ], 1);
    #[classattr]
    const S: Self = Self([
        [0,1,1,0],
        [1,1,0,0],
        [0,0,0,0],
        [0,0,0,0]
    ], 2);
    #[classattr]
    const Z: Self = Self([
        [1,1,0,0],
        [0,1,1,0],
        [0,0,0,0],
        [0,0,0,0]
    ], 3);
    #[classattr]
    const T: Self = Self ([
        [0,1,0,0],
        [1,1,1,0],
        [0,0,0,0],
        [0,0,0,0]
    ], 4);
    #[classattr]
    const O: Self = Self([
        [1,1,0,0],
        [1,1,0,0],
        [0,0,0,0],
        [0,0,0,0]
    ], 5);
    #[classattr]
    const I: Self = Self([
        [0,0,0,0],
        [1,1,1,1],
        [0,0,0,0],
        [0,0,0,0]
    ], 6);

    fn rotate(&mut self){
        let mut new_block: [[usize; 4];4] = [[0,0,0,0],[0,0,0,0], [0,0,0,0], [0,0,0,0]];
        let block_len = match self.1{
            6 => {4},
            5 => {
                self.0 = Block::O.0;
                return;
            }
            (_) => {3}  
        };
        for (y, rows) in self.0.iter().enumerate(){
            for (x, value) in rows.iter().enumerate(){
                if value == &1{
                    new_block[block_len-1-x][y] = value.to_owned();
                }
            }
        };

        self.0 = new_block;
    }
    #[new]
    fn new(int : usize) -> Block{
        match int{
            (6) => Block::I,
            (5) => Block::O,
            (4) => Block::T,
            (3) => Block::Z,
            (2) => Block::S,
            (1) => Block::J,
            (0) => Block::L,
            (_) => Block([[0; 4];4], 7),
        }
    }
}

#[derive(Clone)]
#[pyclass(get_all)]
struct Tetris{
    board: Board,
    active_piece: Block,
    next_piece: Block,
    score: i32,
    game_over: bool
}

#[pymethods]
impl Tetris {
    #[new]
    fn new() -> Tetris{
        let mut rng = rand::thread_rng();
        let active: usize = rng.gen_range(0..7);
        let next: usize = rng.gen_range(0..7);
        Tetris{
            board : Board{board: [[0; 12];22]},
            active_piece : Block::new(active),
            next_piece : Block::new(next),
            score : 0,
            game_over : false,
        }
    }
    fn next(&mut self, columns: usize, rotations: usize){
        match self.check_move(rotations){
            Some(x) => {self.play_move(x, columns, rotations);},
            None => self.kill_the_guy()
        };
        self.active_piece = self.next_piece.clone();
        self.next_piece = Block::new(rand::thread_rng().gen_range(0..7))
    }

    fn play_move(&mut self, offsets: (usize, usize), columns:usize, rotations: usize){
        if rotations != 0{
            for i in (0..rotations){
                self.active_piece.rotate()
            }
        }
        match self.active_piece.1{
            (5) => self.find_height(offsets, columns),
            (_) => self.find_height(offsets, columns)
        }
    }
    fn find_height(&mut self, offsets: (usize, usize), columns:usize){
        let start = columns;
        let finish = start + offsets.1;
        let search = &self.board.board.iter().rev().enumerate().find(|row| row.1[start..finish].iter().sum::<usize>() == 0);
        let essai = match search{
            Some(x) => self.set_piece((start, finish), x.0),
            None => {self.kill_the_guy(); return;},
        };
        match essai{
            Some(x) => {self.board = x.clone();},
            None => {self.board = match self.set_piece((start, finish), search.unwrap().0+1) {
                Some(x) => x,
                None => {self.kill_the_guy(); Board{board : [[0; 12];22]}}
            }
        }
    }
    }
    fn set_piece(&self, offsets: (usize, usize), height: usize) -> Option<Board>{
        let mut board = self.board.clone().board;
        board.reverse();
        let piece = self.active_piece.clone();
        let mut piece = piece.0.iter().rev().filter(|x| x.iter().sum::<usize>() != 0);
        for row in piece.enumerate(){
            for ind in row.1.iter().enumerate(){
                if height > 0{
                    board[height+row.0-1][offsets.0+ind.0] += ind.1.clone();
                }
                else{
                    board[height+row.0][offsets.0+ind.0] += ind.1.clone();
                }
            };
        };
            for x in &board[0..22]{
                for y in x{
                if y == &2{
                    board.reverse();
                    return None;
                }}
            }
        board.reverse();
        Some(Board{board : board})
    }
    fn kill_the_guy(&mut self){
        self.game_over = true;
    }

    
    fn check_move(&self, rotations:usize) -> Option<(usize, usize)>{
        if rotations %2 == 0{
            match self.active_piece.1 {
                (0..=4) => Some((0, 3)),
                (5) => Some((1, 2)),
                (6) => Some((0, 4)),
                (_) => None
            }
        }
        else if rotations == 1 {
            match self.active_piece.1 {
                (0..=4) => Some((0, 2)),
                (5) => Some((1, 2)),
                (6) => Some((1, 1)),
                (_) => None
            }
        }
        else {
            match self.active_piece.1 {
                (0..=4) => Some((1, 2)),
                (5) => Some((1, 2)),
                (6) => Some((2, 1)),
                (_) => None
            }
        }
    }
    fn gen_moves(&self, block: usize) -> Vec<(usize, usize, (usize, usize))>{
        let mut moves: Vec<(usize, usize, (usize, usize))> = vec![];
        for x in (0..4){
            let y = self.check_move(x).unwrap();
            for i in (0..12-y.1){moves.push((x, i, y))}
        };
        moves
    }
    fn gen_genomes(&mut self, last: bool) -> Vec<Tetris>{
        let mut batch: Vec<Tetris> = vec![];
        for i in self.gen_moves(self.active_piece.1){
            let mut genome = self.clone();
            genome.next(i.1, i.0);
            batch.push(genome);
        }
        let mut out: Vec<Tetris> = vec![];
        if last{
            return batch;
        }
        else{
            for mut x in batch{
                out.append(x.gen_genomes(true).as_mut());
            }
            return out;
        }
    }

    fn get_stats(&mut self) -> (usize, usize, usize){
    let mut holes = 0;
    let mut blocades = 0;
    let mut height = 0;
    let mut height_mul = 0;

    for i in 0..12{
        height_mul = 0;
        for j in 1..22{
            height_mul += 1;
            
            if self.board.board[j][i] != 0{
            height += height_mul;
            }
            
            if self.board.board[j-1][i] != 0 && self.board.board[j][i] == 0{
                holes += 1;
                blocades += 1;
                
                let mut k = 1;
                let mut l = 1;
                
                while j - k >= 0 && self.board.board[j-k][i] != 0{
                    blocades += 1;
                    k += 1;
                }
                
                while j + l < 22 && self.board.board[j+l][i] == 0{
                    holes += 1;
                    l += 1;
                }
            }
            }
        }
        return (holes, blocades, height);
            
    }

    fn height_multiplier(&self) -> usize{
        let mut res = 0;
        for (i, row) in self.board.board.iter().enumerate(){
            for val in row{
                if val != &0{
                    res += 22 - i;
                }
            }
            }
        res
    }
    }

#[pymodule]
fn tetrispy(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<Tetris>()?;
    Ok(())
}


