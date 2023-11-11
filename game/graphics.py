import pygame as pg
import numpy as np

class Graphic():

    def __init__(self, width, bg_color, block_color, board):
        self.board = board
        self.num_columns, self.num_rows = board.shape[1], board.shape[0]
        self.width = width
        self.height = self.width * self.num_rows / self.num_columns
        self.bg_color = bg_color
        self.block_color = block_color
        self.block_width = self.width / self.num_columns
        self.block_height = self.height / self.num_rows
        self.display = pg.display.set_mode((self.width, self.height))


    def fill_gradient(self, surface, color, gradient, rect=None, vertical=True, forward=True):
        """fill a surface with a gradient pattern
        Parameters:
        color -> starting color
        gradient -> final color
        rect -> area to fill; default is surface's rect
        vertical -> True=vertical; False=horizontal
        forward -> True=forward; False=reverse
        """
        if rect is None: rect = surface.get_rect()
        x1,x2 = rect.left, rect.right
        y1,y2 = rect.top, rect.bottom
        if vertical: h = y2-y1
        else:        h = x2-x1
        if forward: a, b = color, gradient
        else:       b, a = color, gradient
        rate = (
            float(b[0]-a[0])/h,
            float(b[1]-a[1])/h,
            float(b[2]-a[2])/h
        )
        fn_line = pg.draw.line
        if vertical:
            for line in range(y1,y2):
                color = (
                    min(max(a[0]+(rate[0]*(line-y1)),0),255),
                    min(max(a[1]+(rate[1]*(line-y1)),0),255),
                    min(max(a[2]+(rate[2]*(line-y1)),0),255)
                )
                fn_line(surface, color, (x1,line), (x2,line))
        else:
            for col in range(x1,x2):
                color = (
                    min(max(a[0]+(rate[0]*(col-x1)),0),255),
                    min(max(a[1]+(rate[1]*(col-x1)),0),255),
                    min(max(a[2]+(rate[2]*(col-x1)),0),255)
                )
                fn_line(surface, color, (col,y1), (col,y2))


    def draw_grid(self):
        for row in range(self.num_rows):
            for col in range(self.num_columns):
                pg.draw.rect(self.display, self.block_color, (col * self.block_width, row * self.block_height, self.block_width, self.block_height), 1)

    def draw_board(self):
        for row in range(self.num_rows):
            for col in range(self.num_columns):
                if self.board[row][col] == 1:
                    pg.draw.rect(self.display, self.block_color, (col * self.block_width, row * self.block_height, self.block_width, self.block_height), 0)

    def draw(self):
        self.fill_gradient(self.display, self.bg_color, (0, 0, 0))
        self.draw_grid()
        self.draw_board()
        pg.display.update()


if __name__ == "__main__":
    board = np.array([[0, 0, 0, 0, 0, 0, 0, 0, 0, 0], [0, 1, 0, 1, 0, 0, 1, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0, 0, 0], [0, 1, 0, 1, 0, 0, 1, 0, 0, 0], [0, 1, 0, 1, 0, 0, 1, 0, 0, 0], [0, 1, 0, 1, 0, 0, 1, 0, 0, 0], [0, 1, 0, 1, 0, 0, 1, 0, 0, 0], [0, 1, 0, 1, 0, 0, 1, 0, 0, 0], [0, 1, 0, 1, 0, 0, 1, 0, 0, 0], [0, 1, 0, 1, 0, 0, 1, 0, 0, 0]])
    graphic = Graphic(600, (100, 100, 100), (255, 255, 255), board)
    graphic.draw()
    while True:
        for event in pg.event.get():
            if event.type == pg.QUIT:
                pg.quit()
                quit()