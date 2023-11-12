import numpy as np
import pygame as pg
import random
import time

from graphics import Graphic

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
            cleared = 0,
            current_piece=random.randint(1, 7),
            next_piece=random.randint(1, 7)):
        self.board = board
        self.score = score
        self.cleared = cleared
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

        if y < 5 - size_y:
            self.score -= 500

        return new_board, self.score + clear_count * 1000, self.cleared + clear_count

    def get_stats(self):
        holes = 0
        blocades = 0
        height = 0
        height_mul = 22
        touching_floor = 0
        touching_walls = 0
        touching_edges = 0

        for j in range(1, 22):
            height_mul -= 1
            for i in range(12):
                if self.board[j][i] != 0:
                    height += height_mul

                    if j == 21:
                        touching_floor += 1

                    if i == 0 or i == 11:
                        touching_walls += 1

                    if i > 0 and self.board[j][i - 1] != 0:
                        touching_edges += 1  # Left edge
                    if i < 11 and self.board[j][i + 1] != 0:
                        touching_edges += 1  # Right edge
                    if j > 1 and self.board[j - 1][i] != 0:
                        touching_edges += 1  # Top edge
                    if j < 21 and self.board[j + 1][i] != 0:
                        touching_edges += 1  # Bottom edge

                if self.board[j-1][i] != 0 and self.board[j][i] == 0:
                    holes += 1
                    blocades += 1

                    k = 2
                    l = 1

                    while j - k >= 0 and self.board[j-k][i] != 0:
                        blocades += 1
                        k += 1

                    while j + l < 22 and self.board[j+l][i] == 0:
                        holes += 1
                        l += 1

        return holes, blocades, height, touching_floor, touching_walls, touching_edges

    # (int, int, bool) => game_over, new_leaf
    def apply_move(self, rot, col, gen_next_piece=False):
        # Avoid some costs
        if gen_next_piece:
            next_piece = random.randint(1, 7)
        else:
            next_piece = 0


        piece = np.rot90(PIECES[self.current_piece - 1], rot)
        size_y, size_x = piece.shape

        for y in range(0, 23 - size_y):
            if y == 22 - size_y:
                new_board, new_score, new_cleared = self.place_pieces(piece, size_x, size_y, col, y)
                return False, Tetris(new_board, new_score, new_cleared, self.next_piece, next_piece)

            for i in range(size_x):
                for j in range(size_y):
                    if col + i < 12 and piece[j][i] != 0 and self.board[j + y + 1][i + col] != 0:
                        new_board, new_score, new_cleared = self.place_pieces(piece, size_x, size_y, col, y)

                        if np.any(new_board[0] != 0):
                            return True, None

                        return False, Tetris(new_board, new_score, new_cleared, self.next_piece, next_piece)
                
    def print(self):
        print(self.board)

if __name__ == "__main__":
    t = Tetris()

    for _ in range(20):
        move = random.choice(t.gen_legal_moves())
        game_over, t = t.apply_move(*move, gen_next_piece=True)

        if game_over:
            break

    
    holes, blocades, height, touching_floor, touching_walls, touching_edges = t.get_stats()

    print("Holes:", holes)
    print("Blocades:", blocades)
    print("Height:", height)
    print("Touching down:", touching_floor)
    print("Touching walls:", touching_walls)
    print("Touching walls:", touching_edges)

    graphic = Graphic(400, (0, 0, 0), (0, 0, 0),  (255, 255, 255), t.board)
    graphic.draw()
    while True:
        for event in pg.event.get():
            if event.type == pg.QUIT:
                pg.quit()
                quit()


