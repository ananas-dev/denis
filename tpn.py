from dataclasses import dataclass

@dataclass
class Position:
    board: list[list[int]]
    current_piece: int
    next_piece: int
    score: int

piece_to_int = {
    'I': 1,
    'O': 2,
    'J': 3,
    'L': 4,
    'S': 5,
    'T': 6,
    'Z': 7,
}


def loads(s: str) -> Position:
    board = [[0 for _ in range(10)] for _ in range(22)]
    tokens = s.split(" ")

    board_tok = tokens[0]
    curr_x = 0
    curr_y = 0

    for x in board_tok:
        if x == '/':
            curr_x = 0
            curr_y += 1
        elif x > '0' and x <= '9':
            curr_x += int(x)
        else:
            board[curr_y][curr_x] = piece_to_int[x]
            curr_x += 1

    current_piece = piece_to_int[tokens[1][0]]
    next_piece = piece_to_int[tokens[2][0]]
    score = int(tokens[3])

    return Position(board, current_piece, next_piece, score)