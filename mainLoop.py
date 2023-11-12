from image_recognition import getData
import numpy as np
import pyautogui
from neat_tetris import load_genome, neat_command


def playMove(move, rotate: int):
    if move > 0:
        for _ in range(move): pyautogui.press('right')
    elif move < 0:
        for _ in range(move * -1): pyautogui.press('left')
    else: pass
    for _ in range(rotate): pyautogui.press('up')
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
            playMove(5 - commands[1], commands[0])  # On joue le coup suggéré

        prev_iteration = iteration  # On update l'ancienne itération. >___<"


if __name__ == '__main__':
    # Faster inputs (.01 delay is fine it seems)
    pyautogui.PAUSE = 0.01
    main()  # Main execution