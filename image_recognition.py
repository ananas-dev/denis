import pyautogui
import numpy as np
from PIL import Image
import time


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

def getData(frame):

    width, height = frame.size
    new_width = width // 2.3  # Ratio de la Info_Box / Board_Box
    
    # Extraction de la partie STATS et de la partie NEXT_PIECE
    next_piece = frame.crop((0, (height//2)*0.75, width, height - (height // 3)))
    board = frame.crop((new_width, 0, width, height))

    PIECE_CORRESPONDANCE = {
        "Green": (56, 196, 79),
        "Blue": (50, 164, 250),
        "Orange": (255, 102, 0),
        "Purple": (204, 84, 196),
        "Yellow": (255, 172, 28),
        "Red": (255, 0, 0),
        "Gray": (153, 153, 153)
    }

    # --- Récupération de la prochaine pièce. ---
    next_piece_pixels = np.array(next_piece); COLOR = None
    for color_name, target_color in PIECE_CORRESPONDANCE.items():
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
            
            if cp in PIECE_CORRESPONDANCE.values():
                row.append('1')
            else: row.append('0')
        
        matrix.append(row)    

    return matrix



w = getData(getFrame())
for row in w:
    for elem in row:
        print(elem, end=' ')
    print("")
