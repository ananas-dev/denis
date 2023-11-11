import numpy as np
import pygame as pg
import graphics

pieces = [
    [
        [0,0,1],
        [1,1,1],
        [0,0,0]
    ],
    [
        [1,0,0],
        [1,1,1],
        [0,0,0]
    ],
    [
        [0,1,1],
        [1,1,0],
        [0,0,0]
    ],
    [
        [1,1,0],
        [0,1,1],
        [0,0,0]
    ],
    [
        [0,1,0],
        [1,1,1],
        [0,0,0]
    ],
    [
        [1,1],
        [1,1]
    ],
    [
        [0,0,0,0],
        [1,1,1,1],
        [0,0,0,0],
        [0,0,0,0]
    ]
]

class Tetris:
    def __init__(self):
        self.board = np.zeros((22, 12))
        self.score = 0

    def place_pieces(self, piece, size, x, y):
        for i in range(size):
            for j in range(size):
                if y + j < 22 and self.board[y + j][x + i] != 1 and piece[j][i] != 0:
                    self.board[y + j][x + i] = piece[j][i]

    def check_lines(self):
        for y in range(22):
            if np.all(self.board[y] != 0):
                self.board[1:y+1] = self.board[0:y]
                self.board[0] = 0
                
    def play(self, piece_idx, rot, col):
        piece = np.rot90(pieces[piece_idx], rot)
        size = len(pieces[piece_idx])

        # Place the piece
        last_y = 0
        for y in range(0, 24 - size):
            if y == 23 - size:
                self.place_pieces(piece, size, col, y)
                self.check_lines()
                return

            for i in range(size):
                for j in range(size):
                    if piece[i][j] != 0 and self.board[i + y][j + col] != 0:
                        self.place_pieces(piece, size, col, last_y)
                        self.check_lines()
                        return
            last_y = y

    def print(self):
        print(self.board)


if __name__ == "__main__":
    t = Tetris()
    t.play(1, 0, 0)
    t.play(1, 0, 3)
    t.play(1, 0, 6)
    t.play(1, 0, 9)
    graphic = graphics.Graphic(300, (100, 100, 100), (255, 255, 255), t.board)
    graphic.draw()
    while True:
        for event in pg.event.get():
            if event.type == pg.QUIT:
                pg.quit()
                quit()