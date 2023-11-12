from tetrispy import *
import copy
t = Tetris()

moves = t.gen_genomes(False)
for f in moves:
    for i in f.board.board:
        print(i)
    print("\n")
    f.get_stats()
    