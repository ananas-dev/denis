import numpy as np
import random
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
            current_piece=random.randint(1, 7),
            next_piece=random.randint(1, 7)):
        self.board = board
        self.score = score
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
        new_score = self.score

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


        new_score += clear_count * 1000
        return new_board, new_score

    def height_multiplier(self):
        res = 0
        for i, row in enumerate(self.board):
            for val in row:
                if val != 0:
                    res += 22 - i

        return res

    def holes(self):
        res = 0

    def blocades(self):
        res = 0

    # (int, int, bool) => game_over, new_leaf
    def apply_move(self, rot, col, leaf=False):
        # Avoid some costs
        if leaf:
            next_piece = 0
        else:
            next_piece = random.randint(1, 7)


        piece = np.rot90(PIECES[self.current_piece - 1], rot)
        size_y, size_x = piece.shape

        for y in range(0, 23 - size_y):
            if y == 22 - size_y:
                new_board, new_score = self.place_pieces(piece, size_x, size_y, col, y)
                return False, Tetris(new_board, new_score, self.next_piece, next_piece)

            for i in range(size_x):
                for j in range(size_y):
                    if col + i < 12 and piece[j][i] != 0 and self.board[j + y + 1][i + col] != 0:
                        new_board, new_score = self.place_pieces(piece, size_x, size_y, col, y)

                        if np.any(new_board[0] != 0):
                            return True, None

                        return False, Tetris(new_board, new_score, self.next_piece, next_piece)
                
    def print(self):
        print(self.board)

if __name__ == "__main__":
    time_1 = time.time()

    t0 = Tetris()

    for rot0, col0 in t0.gen_legal_moves():
        game_over, t1 = t0.apply_move(rot0, col0)
        if game_over:
            exit()
        for rot1, col1 in t1.gen_legal_moves():
            game_over, t2 = t1.apply_move(rot1, col1, leaf=True)
            t2.height_muliplier()
            if game_over:
                exit()
            # t2.print()

    time_2 = time.time()

    print(time_2 - time_1)

