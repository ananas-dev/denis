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
    def __init__(self):
        self.board = np.zeros((22, 12))
        self.score = 0
        self.game_over = False
        self.current_piece = 0
        self.next_piece = 0

    def place_pieces(self, piece, size_x, size_y, x, y):
        for i in range(size_x):
            for j in range(size_y):
                if self.board[y + j][x + i] == 0 and piece[j][i] != 0:
                    self.board[y + j][x + i] = piece[j][i] * self.current_piece

    def check_lines(self):
        clear_count = 0

        for y in range(22):
            if np.all(self.board[y] != 0):
                self.board[1:y+1] = self.board[0:y]
                self.board[0] = 0
                clear_count += 1

        match clear_count:
            case 1:
                self.score += 1
            case 2:
                self.score += 2
            case 3:
                self.score += 4
            case 4:
                self.score += 20

    def next_pos(self):
        self.next_piece = random.randint(1, 7)

        if self.next_piece == 0:
            self.current_piece = random.randint(1, 7)
        else:
            self.current_piece = self.next_piece
                
    def play(self, rot, col):
        piece = np.rot90(PIECES[self.current_piece - 1], rot)
        size_y, size_x = piece.shape

        # Quick out of bound check
        if col + size_x > 12:
            self.game_over = True
            return

        # Place the piece
        for y in range(0, 23 - size_y):
            if y == 22 - size_y:
                self.place_pieces(piece, size_x, size_y, col, y)
                self.check_lines()
                return

            for i in range(size_x):
                for j in range(size_y):
                    if col + i < 12 and piece[j][i] != 0 and self.board[j + y + 1][i + col] != 0:
                        self.place_pieces(piece, size_x, size_y, col, y)

                        if np.any(self.board[0] != 0):
                            self.game_over = True
                            return

                        self.check_lines()
                        return

    def print(self):
        print(self.board)

if __name__ == "__main__":

    while True:
        t = Tetris()

        while not t.game_over:
            t.next_pos()
            t.play(random.randint(0, 3), random.randint(0, 8))
            t.print()

    # graphic = graphics.Graphic(300, (64, 201, 255), (232, 28, 255), (255, 255, 255), t.board)
    # graphic.draw()
    # pg.quit()
    # quit()
    # while True:
    #      for event in pg.event.get():
    #          if event.type == pg.QUIT:
    #              pg.quit()
    #              quit()