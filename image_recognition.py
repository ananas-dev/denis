import pyautogui
import numpy as np
from PIL import Image
import time


def isFloating(matrix, forme):
    
    linear_coords = {}
    
    for x, y in forme:
        if x in linear_coords:
            linear_coords[x] = max(linear_coords[x], y)
        else:
            linear_coords[x] = y
    
    w = [(x, y) for x, y in linear_coords.items()]
    
    floating = True

    for subcoord in w:
        try:
            if matrix[subcoord[1]+1][subcoord[0]] == 0:
                pass
            else: floating = False
        except: floating = False
    return floating

def findFloating(matrix: list) -> list:
    
    formes = []
    already_visited = []
    flying = None

    for y, row in enumerate(matrix):
        for x, elem in enumerate(row):
        
            # Nouvelle pièce à ajouter à la collection
            if elem != 0 and (x, y) not in already_visited:
                w = getPiece(matrix, x, y)  # Forme W enregistrée.
                # On voudrait éviter de refaire un traitement pour une case
                # qui appartient à W, parce que ça ferait exactement la même forme.
                for coord in w:
                    already_visited.append(coord)
                formes.append(w)

                if isFloating(matrix, w):
                    return w
    
    return None

def getPiece(matrix, x, y):
    target_num = matrix[y][x]
    rows = len(matrix)
    cols = len(matrix[0])
    visited = [[False] * cols for _ in range(rows)]
    
    def dfs(x, y):
        if x < 0 or x >= cols or y < 0 or y >= rows or visited[y][x] or matrix[y][x] != target_num:
            return []
        visited[y][x] = True
        coordinates = [(x, y)]
        for dx, dy in [(1, 0), (-1, 0), (0, 1), (0, -1)]:
            next_x, next_y = x + dx, y + dy
            coordinates += dfs(next_x, next_y)
        return coordinates

    return dfs(x, y)

def getFrame():

    # Transformation d'un screenshot en array NumPY
    screenshot = pyautogui.screenshot()
    image_np = np.array(screenshot)

    target_color = np.array([190, 158, 124])    # RGB Values de la bordure.
    tolerance = 0                               # Tolérance pour la variation de couleur (test purpose)
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

    else: print("Aucun pixel correspondant à la couleur cible n'a été trouvé."); raise Exception()

def masquageMatrix(matrix: list) -> list:
    
    flying = findFloating(matrix)
    # On récupère la couleur de la pièce avant
    # de masquer.
    flying_piece_type = matrix[flying[0][1]][flying[0][0]]
    for _x, _y in flying:
        matrix[_y][_x] = 0
    return flying_piece_type

def getData():

    frame = getFrame()  # Récupération de l'image (pyautogui duh)

    width, height = frame.size
    new_width = width // 2.3  # Ratio de la Info_Box / Board_Box
    
    # Extraction de la partie STATS et de la partie NEXT_PIECE
    next_piece = frame.crop((0, (height//2)*0.75, width // 2.25, height - (height // 3)))
    board = frame.crop((new_width, 0, width, height))

    RGB_2_ID = {
        (56, 196, 79): 1,
        (50, 164, 250): 2,
        (255, 172, 28): 3,
        (255, 102, 0): 4,
        (204, 84, 196): 5,
        (153, 153, 153): 6,
        (255, 0, 0): 7}
    # 1 : Green   | 2 : Blue
    # 3 : Yellow  | 4 : Orange
    # 5 : Purple  | 6 : Gray
    # 7 : Red

    # --- Récupération de la prochaine pièce. ---
    next_piece_pixels = np.array(next_piece); COLOR = None
    for target_color, color_name in RGB_2_ID.items():
        # On vérifie si notre couleur se trouve dans l'array NumPY.
        mask = np.all(next_piece_pixels == np.array(target_color), axis=2)
        if np.any(mask): COLOR = color_name
    del next_piece  # Variable inutile, on peut libérer l'espace.


    # --- Récupération de la matrice du Board ---
    board_width, board_height = board.size
    sub_width = board_width // 12
    sub_height = board_height // 22

    matrix = []

    for y in range(22):
        row = []
        for x in range(12):

            left = x * sub_width
            upper = y * sub_height
            right = left + sub_width
            lower = upper + sub_height
            box = (left, upper, right, lower)
            sub_image = board.crop(box)
            
            cx, cy = sub_image.size
            cx = cx // 2
            cy = cy // 2
            cp = sub_image.getpixel((cx, cy))
            row.append(RGB_2_ID.get(cp, 0))
        
        matrix.append(row)

    pieceActuelle = masquageMatrix(matrix)

    return {'Matrix': matrix,
            'pieceActuelle': pieceActuelle,
            'pieceSuivante': COLOR}


"""
time.sleep(3)
w = getData()
for row in w.get('Matrix'):
    print(row)
print(f'Pièce Actuelle : {w.get("pieceActuelle")}')
print(f'Pièce Suivante : {w.get("pieceSuivante")}')
"""