import numpy as np
import pygame as pg
import random
import graphics
import time

PIECES = [
    [
        [0,0,1],
        [1,1,1],
    ],
    [
        [1,0,0],
        [1,1,1],
    ],
    [
        [0,1,1],
        [1,1,0],
    ],
    [
        [1,1,0],
        [0,1,1],
    ],
    [
        [0,1,0],
        [1,1,1],
    ],
    [
        [1,1],
        [1,1]
    ],
    [
        [1,1,1,1],
    ]
]

class Tetris:
    def __init__(
            self,
            board=np.zeros((22, 12)),
            score = 0,
            game_over = False,
            current_piece=random.randint(1, 7),
            next_piece=random.randint(1, 7)):
        self.board = board
        self.score = score
        self.game_over = game_over
        self.current_piece = current_piece
        self.next_piece = next_piece

    def gen_legal_moves(self):
        legal_moves = []
        for rot in range(0, 4):
            piece = np.rot90(PIECES[self.current_piece - 1], rot)
            _, size_x = piece.shape
            for col in range(0, 13 - size_x):
                legal_moves.append((rot, col))

        return legal_moves
    
    def place_pieces(self, piece, size_x, size_y, x, y):
        new_board = self.board.copy()
        for i in range(size_x):
            for j in range(size_y):
                if new_board[y + j][x + i] == 0 and piece[j][i] != 0:
                    new_board[y + j][x + i] = piece[j][i] * self.current_piece

        clear_count = 0

        for y in range(22):
            if np.all(new_board[y] != 0):
                new_board[1:y+1] = new_board[0:y]
                new_board[0] = 0
                clear_count += 1

        return new_board

    # def next_pos(self):
    #     self.next_piece = random.randint(1, 7)

    #     if self.next_piece == 0:
    #         self.current_piece = random.randint(1, 7)
    #     else:
    #         self.current_piece = self.next_piece

    def height_muliplier(self):
        res = 0
        for i, row in enumerate(self.board):
            for val in row:
                if val != 0:
                    penalty += 22 - i

        return res

    def holes(self):
        res = 0

    def apply_move(self, rot, col):
        piece = np.rot90(PIECES[self.current_piece - 1], rot)
        size_y, size_x = piece.shape

        for y in range(0, 23 - size_y):
            if y == 22 - size_y:
                new_board = self.place_pieces(piece, size_x, size_y, col, y)
                return Tetris(new_board, self.score, self.game_over, current_piece=self.next_piece, next_piece=0)

            for i in range(size_x):
                for j in range(size_y):
                    if col + i < 12 and piece[j][i] != 0 and self.board[j + y + 1][i + col] != 0:
                        new_board = self.place_pieces(piece, size_x, size_y, col, y)

                        if np.any(new_board[0] != 0):
                            self.game_over = True

                        return Tetris(new_board, self.score, self.game_over, current_piece=self.next_piece, next_piece=0)
                
    def print(self):
        print(self.board)

if __name__ == "__main__":
    t0 = Tetris()

    for rot0, col0 in t0.gen_legal_moves():
        t1 = t0.apply_move(rot0, col0)
        for rot1, col1 in t1.gen_legal_moves():
            t2 = t1.apply_move(rot1, col1)
            t2.print()

