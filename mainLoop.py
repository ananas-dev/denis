from image_recognition import getData
import numpy as np


def main():

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
            print(f"On doit jouer un coup !")
            ...  # Get bests moves from AI.
            ...  # Some function 2 execute inputs.

        prev_iteration = iteration  # On update l'ancienne itération. >___<"


main()
