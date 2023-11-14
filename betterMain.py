
from betterDetection import getData                 # Game detection and image recognition.
import numpy as np                                  # Array gestion.
import pyautogui                                    # Keyboard input.
from neat_tetris import load_genome, neat_command   # Decision and communication with AI.
import time                                         # Time module for testing purposes.


def getOffset(columun: int, rotation: int, piece: int) -> int:

    # Fonction retournant le nombre de déplacements à effectuer
    # en fonction de la rotation de la pièce et de son type.
    # Résultat à mettre en valeur absolue.
    # - offset > 0 : vers la droite !
    # - offset < 0 : vers la gauche !

    offset = 0

    if piece == 6: offset = columun - 5
    elif piece == 7:
        if rotation == 0 or rotation == 2: offset = columun - 4
        elif rotation == 1: offset = columun - 5
        else: offset = columun - 6  # Rotation == 3
    else:
        if rotation == 3: offset = columun - 5
        else: offset = columun - 4
    
    return offset

def playMove(column: int, rotation: int, piece: int) -> None:

    # Fonction permettant de jouer le coup décidé par l'IA.
    # Format de la décision : (Colonne, Rotation).

    for _ in range(rotation): pyautogui.press('up')

    offset = getOffset(column, rotation, piece)
    
    if offset > 0: 
        for _ in range(abs(offset)): pyautogui.press('right')
    elif offset < 0: 
        for _ in range(abs(offset)): pyautogui.press('left')
    else: pass  # No left / right actions.

    pyautogui.press('space')  # --> Need to be reworked around the spam bug.    

def main(NN_name: str):

    net = load_genome(NN_name)
    # Initialisation de l'AI depuis un dataset .pkl

    frame = getData()
    prev_iteration = np.array(frame.get('Matrix'))

    while 1:

        frame = getData()

        if frame.get('pieceActuelle') == 0:
            frame = getData()  # On reload une frame

        iteration = np.array(frame.get('Matrix'))
        difference = np.array(iteration - prev_iteration)

        # On calcule la différence entre les deux itérations et
        # on vérifie si y'a une différence.

        if not np.all(difference == 0):

            # On doit jouer un coup avec les data de Frame.
            pieceActuelle, pieceSuivante, gameMatrix = frame.get('pieceActuelle'), frame.get('pieceSuivante'), frame.get('Matrix')
            
            # Génération du meilleur coup selon l'AI <net> (Format : [Rotation, Column])
            _r, _c = neat_command(pieceActuelle, pieceSuivante, gameMatrix, net)
            playMove(_c, _r, pieceActuelle)  # On joue le coup.

        prev_iteration = iteration  # On update l'ancienne itération.

    print('[!] Execution stopped.')


if __name__ == '__main__': 
    # Faster inputs (.01 delay is fine it seems)
    time.sleep(2)
    pyautogui.PAUSE = 0.001 
    main('winner.pkl')  # Main execution