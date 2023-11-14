import pyautogui
import numpy as np
from PIL import Image
from RGB_CONFIG import *


def getAllPieces(matrix: list) -> list:

    # Fonction qui récupère toutes les pièces
    # dans la matrice. Une pièce est un amas d'un même
    # nombre. [!] Pas connectées en diagonale ! (Tetris)

    def DFS(_matrix, _x, _y, value, visited=None):

        if visited is None: visited = []
        perms = [(1, 0), (0, 1), (-1, 0), (0, -1)]

        for perm in perms:
            
            tx, ty = _x + perm[0], _y + perm[1]
            if tx >= 0 and tx < len(_matrix[0]) and ty >= 0 and ty < len(_matrix):
                if (tx, ty) not in visited and _matrix[ty][tx] == value:
                    new_visited = visited + [(tx, ty)]
                    visited = DFS(_matrix, tx, ty, value, new_visited)

        return visited
    
    visited = []; pieces = []

    for y, row in enumerate(matrix):
        for x, case in enumerate(row):

            if case != 0 and (x, y) not in visited:
                piece = DFS(matrix, x, y, case, None)
                pieces.append(piece)
                for i in piece: visited.append(i)

    return pieces

def getFlyingPiece(matrix: list) -> bool:

    # Fonction récupèrant la pièce volante dans
    # la matrice.

    pieces = getAllPieces(matrix)

    def lineariser(coords: list) -> list:

        coords = sorted(coords, key=lambda coord: coord[0])
        linearized_coords = []
        current_x = None
        max_y = float('-inf')

        for coord in coords:
            x, y = coord
            if x != current_x:
                if current_x is not None:
                    linearized_coords.append((current_x, max_y))
                current_x = x
                max_y = y
            else:
                max_y = max(max_y, y)

        if current_x is not None:
            linearized_coords.append((current_x, max_y))
        return linearized_coords

    for p in pieces: 

        flying = True; linear_coordinates = lineariser(p)

        for i in linear_coordinates:
            # On va vérifier qu'il y'a des 0 sous toutes
            # les coordonnées linéaires de la pièce.
            if i[1] < len(matrix) - 1:
                if matrix[i[1]+1][i[0]] != 0:
                    flying = False
            else: flying = False
        if flying: return p

    return None

def masquageMatrice(matrix: list) -> None:

    # Fonction de masquage de la Matrice "matrix".
    # S'il n'y a pas de pièce volante, renvoie 0.
    # Sinon, renvoie l'ID de la pièce masquée.

    flying_piece = getFlyingPiece(matrix)

    if flying_piece is not None:

        if len(flying_piece) == 0: return 0
        # On récupère l'ID de la pièce volante.
        ID = matrix[flying_piece[0][1]][flying_piece[0][0]]
        for _x, _y in flying_piece:
            matrix[_y][_x] = 0  # Masquage

        return ID
    
    else: return 0  # Pas de flying piece

def getGameFrame(tolerance: int = 0):

    # Transformation d'un screenshot en array NumPY
    screenshot = pyautogui.screenshot()
    image_np = np.array(screenshot)

    target_color = np.array(BORDER_RGB)
    # RGB Values de la bordure. Susceptible de changer un peu, en fonction de la version du jeu. (1.19)
    # Calculer la distance entre chaque pixel de l'image et la couleur de la bordure
    color_distance = np.linalg.norm(image_np - target_color, axis=-1)
    mask = color_distance <= tolerance

    matching_pixels = np.argwhere(mask)

    if matching_pixels.size > 0:

        min_x, min_y = matching_pixels.min(axis=0)
        max_x, max_y = matching_pixels.max(axis=0)

        # Extraire la région de l'image originale délimitée par ces coordonnées
        region = image_np[min_x:max_x + 1, min_y:max_y + 1, :]
        return Image.fromarray(region)  # Convertir la région en une image PIL

    else: return None

def getData():

    # Fonction de récupération des datas sur la grille
    # de jeu sous forme d'un dictionnaire (voir retour).
    # Les exceptions du code sont gérées normalement sans exit (par des return de 0).

    gameFrame = getGameFrame(TOLERANCE_BORDER)  # On récupère la Frame du Tetris (Tolérance set sur 0, peut changer à max ~ 5).
    if gameFrame is None:
        while gameFrame is None:
            print('[?] Trying to get game Frame...')
            gameFrame = getGameFrame(TOLERANCE_BORDER)
    
    width, height = gameFrame.size
    border = width / 2.3  # Ratio entre les infos et le board

    # Traitement de la partie jeu de la gameFrame
    gameBoard = gameFrame.crop((border, 0, width, height))
    npGameBoard = np.array(gameBoard)  # On transforme en array numPy

    largeur_bloc = gameBoard.width // 12    # Dimension en largeur du board !
    hauteur_bloc = gameBoard.height // 22   # Dimension en hauteur du board !
    gameMatrix = [[0 for _ in range(12)] for _ in range(22)]
    
    RGB_2_ID = {GREEN: 1, BLUE: 2, YELLOW: 3, ORANGE: 4, PURPLE: 5, GRAY: 6, RED: 7}

    # 1 : Green   | 2 : Blue
    # 3 : Yellow  | 4 : Orange
    # 5 : Purple  | 6 : Gray
    # 7 : Red

    # On crée une fonction locale pour trouver la couleur la plus proche
    # de celle stockée dans le dictionnaire. Évite d'ignorer les
    # valeurs trop proches, obtenues à cause d'imprécisions.

    def find_closest_color(rgb_color, color_dict, tolerance):
        closest_color_id = 0
        min_distance = float('inf')

        for key, value in color_dict.items():
            r, g, b = key
            distance = np.sqrt((rgb_color[0] - r) ** 2 + (rgb_color[1] - g) ** 2 + (rgb_color[2] - b) ** 2)

            if distance < tolerance and distance < min_distance:
                min_distance = distance
                closest_color_id = value

        return closest_color_id

    for i in range(22):
        for j in range(12):

            left = (j) * largeur_bloc + round(largeur_bloc / 7)     # Offset pour centrer le tir (default : 7)
            upper = (i) * hauteur_bloc + round(hauteur_bloc / 7)    # Offset pour centrer le tir (default : 7)
            right = left + largeur_bloc
            lower = upper + hauteur_bloc

            # On trouve les coordonnées centrales de chaque sous bloc.
            cx, cy = (left + right) // 2, (upper + lower) // 2
            _r, _g, _b = npGameBoard[cy, cx]  # On obtient la couleur au centre.
            
            gameMatrix[i][j] = find_closest_color((_r, _g, _b), RGB_2_ID, tolerance=TOLERANCE_PIECES)
            # On place l'ID de la couleur correspondante dans la matrice.
            # Tolérance set à 20 par défaut ; si pas accurate l'auguementer (il devrait pas y'avoir besoin).

    # Traitement de la partie Info de la gameFrame
    gameInfo = gameFrame.crop((0, height // 2.5, border, height // 1.5))  # Crop sélectif pour n'avoir que la pièce suivante.
    npGameInfo = np.array(gameInfo)

    COLOR = 0
    for target_color, color_ID in RGB_2_ID.items():
        mask = np.all(np.abs(npGameInfo - np.array(target_color)) <= TOLERANCE_PIECES, axis=2)
        if np.any(mask): COLOR = color_ID; continue
    pieceSuivante = COLOR  # On récupère la pièce suivante :)

    # Pour la pièce actuelle, on va faire appel à la fonction de Masquage pour la Matrice.
    pieceActuelle = masquageMatrice(gameMatrix)  # 0 indique qu'il n'y a pas de pièce actuelle.
    
    return {
        'Matrix': gameMatrix,
        'pieceActuelle': pieceActuelle,
        'pieceSuivante': pieceSuivante}



if __name__ == '__main__':

    # Testing purpose >__<"
    w = getData()
    for row in w.get('Matrix'):
        print(row)
        