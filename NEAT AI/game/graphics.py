import pygame as pg
import numpy as np

class Graphic():

    def __init__(self, width, bg_color, block_color, board):
        self.board = board
        self.num_columns = board.shape[0]
        self.num_rows = board.shape[1]
        self.ratio = board.shape[0] / board.shape[1]
        self.width = width
        self.height = int(width / self.ratio)
        self.block_size = int(self.width / board.shape[0])
        self.bg_color = bg_color
        self.block_color = block_color
        pg.init()
        pg.display.set_caption("Tetris")
        self.display = pg.display.set_mode((self.width, self.height))

    def draw_grid(self):
        for i in range(self.num_columns):
            for j in range(self.num_rows):
                pg.draw.rect(self.display, self.block_color, (i * self.block_size, j * self.block_size, self.block_size, self.block_size), 1)

    def draw_board(self):
        for i in range(self.num_columns):
            for j in range(self.num_rows):
                if self.board[i][j] == 1:
                    pg.draw.rect(self.display, self.block_color, (i * self.block_size, j * self.block_size, self.block_size, self.block_size))

    def draw(self):
        self.display.fill(self.bg_color)
        self.draw_grid()
        self.draw_board()
        pg.display.update()


if __name__ == "__main__":
    board = np.array([[0, 0, 0, 0, 0, 0, 0, 0, 0, 0], [0, 1, 0, 1, 0, 0, 1, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0, 0, 0], [0, 1, 0, 1, 0, 0, 1, 0, 0, 0]])
    graphic = Graphic(200, (0, 0, 0), (255, 255, 255), board)
    graphic.draw()
    while True:
        for event in pg.event.get():
            if event.type == pg.QUIT:
                pg.quit()
                quit()