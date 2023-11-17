import pygame as pg
import numpy as np

class Graphic():

    def __init__(self, width, bg_color_1, bg_color_2, grid_color, board, fps=10):
        pg.init()
        pg.font.init()
        self.clock = pg.time.Clock()
        self.fps = fps
        self.board = board
        self.current_piece = -1
        self.next_pieces = []
        self.pocket = None
        self.score = 0
        self.num_columns, self.num_rows = board.shape[1], board.shape[0]
        self.width = width
        self.height = self.width * self.num_rows / self.num_columns
        self.side_panel_cols = 6
        self.bg_color_1 = bg_color_1
        self.bg_color_2 = bg_color_2
        self.grid_color = grid_color
        self.block_width = self.width / self.num_columns
        self.block_height = self.height / self.num_rows
        self.display = pg.display.set_mode((self.width + self.side_panel_cols*self.block_width, self.height))
        pg.display.set_caption('NEAT Tetris')
        self.font = pg.font.SysFont("CMU Serif Roman", int(self.block_width*0.7))
        self.ID_2_RGB = {
            1 : (56, 196, 79),
            2 : (50, 164, 250),
            3 : (255, 172, 28),
            4 : (255, 102, 0),
            5 : (204, 84, 196),
            6 : (153, 153, 153),
            7 : (255, 0, 0)}


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
        for row in range(self.num_rows+1):
            pg.draw.line(self.display, self.grid_color, (0, row * self.block_height), (self.width, row * self.block_height), 2)
        for col in range(self.num_columns+1):
            pg.draw.line(self.display, self.grid_color, (col * self.block_width, 0), (col * self.block_width, self.height), 2)

    def draw_board(self):
        for row in range(self.num_rows):
            for col in range(self.num_columns):
                if self.board[row][col] != 0:
                    color = self.ID_2_RGB[self.board[row][col]]
                    pg.draw.rect(self.display, color, (col * self.block_width, row * self.block_height, self.block_width, self.block_height), 0)


    def draw_piece(self, piece, row):
        """Draws a piece on the board

        Args:
            piece (int): piece to draw
            pos (tuple): row and columns where to draw the piece in the side panel (0, 0) is the top left corner of the side panel
        """
        color = self.ID_2_RGB[piece]
        if piece == 1: # L
            pg.draw.rect(self.display, color=color, rect=((self.num_columns+3)*self.block_width, row*self.block_height, self.block_width, self.block_height))
            pg.draw.rect(self.display, color=color, rect=((self.num_columns+1)*self.block_width, (row+1)*self.block_height, self.block_width, self.block_height))
            pg.draw.rect(self.display, color=color, rect=((self.num_columns+2)*self.block_width, (row+1)*self.block_height, self.block_width, self.block_height))
            pg.draw.rect(self.display, color=color, rect=((self.num_columns+3)*self.block_width, (row+1)*self.block_height, self.block_width, self.block_height))

        elif piece == 2: # J
            pg.draw.rect(self.display, color=color, rect=((self.num_columns+1.5)*self.block_width, row*self.block_height, self.block_width, self.block_height))
            pg.draw.rect(self.display, color=color, rect=((self.num_columns+1.5)*self.block_width, (row+1)*self.block_height, self.block_width, self.block_height))
            pg.draw.rect(self.display, color=color, rect=((self.num_columns+2.5)*self.block_width, (row+1)*self.block_height, self.block_width, self.block_height))
            pg.draw.rect(self.display, color=color, rect=((self.num_columns+3.5)*self.block_width, (row+1)*self.block_height, self.block_width, self.block_height))

        elif piece == 3: # S
            pg.draw.rect(self.display, color=color, rect=((self.num_columns+2.5)*self.block_width, row*self.block_height, self.block_width, self.block_height))
            pg.draw.rect(self.display, color=color, rect=((self.num_columns+3.5)*self.block_width, row*self.block_height, self.block_width, self.block_height))
            pg.draw.rect(self.display, color=color, rect=((self.num_columns+1.5)*self.block_width, (row+1)*self.block_height, self.block_width, self.block_height))
            pg.draw.rect(self.display, color=color, rect=((self.num_columns+2.5)*self.block_width, (row+1)*self.block_height, self.block_width, self.block_height))

        elif piece == 4: # Z
            pg.draw.rect(self.display, color=color, rect=((self.num_columns+1.5)*self.block_width, row*self.block_height, self.block_width, self.block_height))
            pg.draw.rect(self.display, color=color, rect=((self.num_columns+2.5)*self.block_width, row*self.block_height, self.block_width, self.block_height))
            pg.draw.rect(self.display, color=color, rect=((self.num_columns+2.5)*self.block_width, (row+1)*self.block_height, self.block_width, self.block_height))
            pg.draw.rect(self.display, color=color, rect=((self.num_columns+3.5)*self.block_width, (row+1)*self.block_height, self.block_width, self.block_height))
        
        elif piece == 5: # T
            pg.draw.rect(self.display, color=color, rect=((self.num_columns+2.5)*self.block_width, row*self.block_height, self.block_width, self.block_height))
            pg.draw.rect(self.display, color=color, rect=((self.num_columns+1.5)*self.block_width, (row+1)*self.block_height, self.block_width, self.block_height))
            pg.draw.rect(self.display, color=color, rect=((self.num_columns+2.5)*self.block_width, (row+1)*self.block_height, self.block_width, self.block_height))
            pg.draw.rect(self.display, color=color, rect=((self.num_columns+3.5)*self.block_width, (row+1)*self.block_height, self.block_width, self.block_height))
        
        elif piece == 6: # O
            pg.draw.rect(self.display, color=color, rect=((self.num_columns+2)*self.block_width, row*self.block_height, self.block_width, self.block_height))
            pg.draw.rect(self.display, color=color, rect=((self.num_columns+3)*self.block_width, row*self.block_height, self.block_width, self.block_height))
            pg.draw.rect(self.display, color=color, rect=((self.num_columns+2)*self.block_width, (row+1)*self.block_height, self.block_width, self.block_height))
            pg.draw.rect(self.display, color=color, rect=((self.num_columns+3)*self.block_width, (row+1)*self.block_height, self.block_width, self.block_height))
        
        elif piece == 7: # I
            pg.draw.rect(self.display, color=color, rect=((self.num_columns+1)*self.block_width, row*self.block_height, self.block_width, self.block_height))
            pg.draw.rect(self.display, color=color, rect=((self.num_columns+2)*self.block_width, row*self.block_height, self.block_width, self.block_height))
            pg.draw.rect(self.display, color=color, rect=((self.num_columns+3)*self.block_width, row*self.block_height, self.block_width, self.block_height))
            pg.draw.rect(self.display, color=color, rect=((self.num_columns+4)*self.block_width, row*self.block_height, self.block_width, self.block_height))


    def draw_pieces(self):
        """Draws current piece and next pieces on the side panel

        Args:
            current_piece (int): id of the current piece
            next_pieces (list): list of id corresponding to the n next pieces
        """
        self.draw_piece(self.current_piece, 1)
        if self.pocket != None:
            self.draw_piece(self.pocket, 18)
        pg.draw.rect(self.display, self.grid_color, ((self.num_columns+1)*self.block_width, self.block_height, (self.side_panel_cols-2)*self.block_width, 2*self.block_height), 2)
        pg.draw.rect(self.display, self.grid_color, ((self.num_columns+1)*self.block_width, 4*self.block_height, (self.side_panel_cols-2)*self.block_width, 12*self.block_height), 2)
        for i, piece in enumerate(self.next_pieces):
            self.draw_piece(piece, 3*i+4)


    def show_score(self):
        score_text = self.font.render(f'{self.score:012}', True, self.grid_color)
        pos = ((self.num_columns + 0.8)*self.block_width, 5)
        self.display.blit(score_text, pos)
        

    def draw(self):
        self.fill_gradient(self.display, self.bg_color_1, self.bg_color_2)
        self.draw_board()
        self.draw_grid()
        self.show_score()
        self.draw_pieces()
        pg.display.update()

    def tick(self):
        self.clock.tick(self.fps)
        for event in pg.event.get():
            if event.type == pg.QUIT:
                pg.quit()
                quit()


if __name__ == "__main__":
    board = np.zeros((22, 12))
    board[-1] = np.ones(12)
    board[-2] = np.ones(12) + 2
    graphic = Graphic(300, (0, 0, 0), (0, 0,0),  (255, 255, 255), board)
    graphic.current_piece = 3
    graphic.score = 123456789000
    graphic.next_pieces = [4, 6, 6]
    while True:
        graphic.draw()
        graphic.tick()