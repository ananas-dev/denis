from tetris import *
import copy
t = Tetris()

moves = t.gen_legal_moves()
for f, j in moves:
    s = copy.deepcopy(t)
    s.apply_move(f, j, True)
    for x, y in s.gen_legal_moves():
        v = copy.deepcopy(s)
        v.apply_move(x, y)
        v.get_stats()