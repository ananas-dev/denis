import pygame as pg
import numpy as np
import imgui

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
        for i in range(self.num_columns):
            pg.draw.line(self.display, self.block_color, (i * self.block_size, 0), (i * self.block_size, self.height))

        for i in range(self.num_rows):
            pg.draw.line(self.display, self.block_color, (0, i * self.block_size), (self.width, i * self.block_size))

    def draw_board(self):
        for i in range(self.num_columns):
            for j in range(self.num_rows):
                if self.board[i][j] == 1:
                    pg.draw.rect(self.display, self.block_color, (i * self.block_size, j * self.block_size, self.block_size, self.block_size))

    def draw(self):
        self.fill_gradient(self.display, self.bg_color, (0, 0, 0), vertical=True, forward=True)
        self.draw_grid()
        self.draw_board()
        pg.display.update()


if __name__ == "__main__":
    board = np.array([[0, 0, 0, 0, 0, 0, 0, 0, 0, 0], [0, 1, 0, 1, 0, 0, 1, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0, 0, 0], [0, 1, 0, 1, 0, 0, 1, 0, 0, 0], [0, 1, 0, 1, 0, 0, 1, 0, 0, 0], [0, 1, 0, 1, 0, 0, 1, 0, 0, 0]])
    graphic = Graphic(200, (100, 100, 100), (255, 255, 255), board)
    graphic.draw()
    while True:
        for event in pg.event.get():
            if event.type == pg.QUIT:
                pg.quit()
                quit()