from image_recognition import getData
import numpy as np
import pyautogui
from neat_tetris import load_genome, neat_command
import time

def playMove(col: int, rotate: int, piece_t: int):
    
    offset = 0

    if piece_t == 6:
        offset = col - 5
    elif piece_t == 7:
        if rotate == 0 or rotate == 2:
            offset = col - 4
        elif rotate == 1:
            offset = col - 5
        elif rotate == 3:
            offset = col - 6
    else:
        if rotate == 3:
            offset = col - 5
        else:
            offset = col - 4

    for _ in range(rotate): pyautogui.press('up')

    if offset < 0:
        for _ in range(abs(offset)): pyautogui.press('left')
    elif offset > 0:
        for _ in range(abs(offset)): pyautogui.press('right')

    pyautogui.press('space')
 
def main():

    net = load_genome("winner.pkl")
    # Initialisation de l'AI depuis un dataset .pkl
    frame = getData()
    prev_iteration = np.array(frame.get('Matrix'))

    while 1:

        frame = getData()
        if frame.get('pieceActuelle') is None: frame = getData()
        # Sécurité dans le cas ou on a récupéré une grille entre
        # le changement des pièces (.135s d'exécution, safe 1 fois)
        iteration = np.array(frame.get('Matrix'))

        # On calcule la différence entre les deux itérations et
        # on vérifie si y'a une différence.
        difference = np.array(iteration - prev_iteration)
        if not np.all(difference == 0):
            # On doit jouer un coup avec les data de Frame.
            pieceActuelle, pieceSuivante, gameMatrix = frame.get('pieceActuelle'), frame.get('pieceSuivante'), frame.get('Matrix')
            # Génération du meilleur coup selon l'AI <net>
            commands = neat_command(pieceActuelle, pieceSuivante, gameMatrix, net)
            # Format : [Turns, Row]
            # print(f'Colonne : {commands[1]} ({commands[0]} rotations).')
            if commands == (None, None):
                time.sleep(0.5)
                pyautogui.write("Denis")
                time.sleep(0.5)
                pyautogui.press("enter")
                time.sleep(0.5)
                pyautogui.press("n")
                time.sleep(0.1)
                pyautogui.press("enter")
                continue

                

            playMove(commands[1], commands[0], pieceActuelle)  # On joue le coup suggéré
        prev_iteration = iteration  # On update l'ancienne itération. >___<"
    

if __name__ == '__main__': 
    # Faster inputs (.001 delay is fine it seems)
    pyautogui.PAUSE = 0.001 
    main()  # Main execution