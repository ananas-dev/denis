import os
import pygame as pg
import numpy as np
import neat
import pickle # Used to save the model
import multiprocessing
import tetris
import graphics


### TRAINING PARAMETERS ###
train = False


##########################
### NEAT CONFIGURATION ###

pop_size = 1000
fitness_threshold = 1000
num_inputs = 7  # 7 inputs : height multiplier, blockades, hole, clear, ...
num_outputs = 1 # Score based on the 7 inputs
num_generations = 10000

##########################
##########################


def modify_config_file():
    with open("config.txt", "r") as f:
        pop_index, threshold_index, inputs_index, outputs_index = 0, 0, 0, 0
        lines = f.readlines()
        for i in range(len(lines)):
            if lines[i].startswith("pop_size"):
                pop_index = i
            elif lines[i].startswith("fitness_threshold"):
                threshold_index = i
            elif lines[i].startswith("num_inputs"):
                inputs_index = i
            elif lines[i].startswith("num_outputs"):
                outputs_index = i

    with open("config.txt", "w") as f:
        lines[pop_index] = f"pop_size              = {pop_size}\n"
        lines[threshold_index] = f"fitness_threshold     = {fitness_threshold}\n"
        lines[inputs_index] = f"num_inputs              = {num_inputs}\n"
        lines[outputs_index] = f"num_outputs             = {num_outputs}\n"
        f.writelines(lines)


### EVAL FUNCTION ###
def convert_command(commands, num_columns):
    command_1, command_2 = commands

    # Converts to number of rotations
    if command_1 <= -0.5:
        command_1 = 0
    elif -0.5 < command_1 <= 0:
        command_1 = 1
    elif 0 < command_1 <= 0.5:
        command_1 = 2
    else:
        command_1 = 3

    # Converts [-1, 1] to index of column [0, num_columns - 1]
    command_2 = int((command_2 + 1) * (num_columns - 1) / 2)

    return [command_1, command_2]

def flatten_matrix(matrix):
    """Flattens a matrix into a list

    Args:
        matrix (list): matrix to flatten

    Returns:
        list: flattened matrix
    """
    return [item for sublist in matrix for item in sublist]

def eval_genomes(genomes, config):
    """Creates the eval function that will be used in a thread pool for each genome

    Args:
        genomes (genome): genome object
        config (config): config object
        
        returns : fitness of the genome
    """
    global p
    games = []
    ge = []
    nets = []

    for _id, genome in genomes:
        games.append(tetris.Tetris())
        ge.append(genome)
        net = neat.nn.FeedForwardNetwork.create(genome, config)
        nets.append(net)
        genome.fitness = 0

    # Runs the game until it ends
    while True:

        # for i, t in enumerate(tet):
        #     if t.game_over:
        #         tet.pop(i)
        #         ge.pop(i)
        #         nets.pop(i)

        # if len(tet) == 0:
        #     break
        
        # for i, t in enumerate(tet):
        #     game_state = t.board  # Gets the current state of the game
        #     current_block = t.current_piece
        #     next_block = t.next_piece
        #     commands = nets[i].activate((current_block, next_block, *flatten_matrix(game_state)))
        #     commands = convert_command(commands, 12)
        #     tet[i].play(*commands)
        #     ge[i].fitness = t.score
        #     t.next_pos() 

        for i, t in enumerate(games):
            positions = t.get_positions()
            for pos in positions:
                pass


def load_genome(genome_path):
        config = neat.Config(neat.DefaultGenome, neat.DefaultReproduction,
                            neat.DefaultSpeciesSet, neat.DefaultStagnation,
                            "config.txt")
        print("Loading the best genome...")
        genome = pickle.load(open('winner.pkl', 'rb'))
        print("Genome loaded")
        net = neat.nn.FeedForwardNetwork.create(genome, config)
        return net

def neat_command(current_block, next_block, game_board, net):
    commands = net.activate((current_block, next_block, *flatten_matrix(game_board)))
    commands = convert_command(commands, 12)
    return commands

    
            

### RUN FUNCTION ###
def run(config_file):
    global p
    # Load configuration.
    config = neat.Config(neat.DefaultGenome, neat.DefaultReproduction,
                         neat.DefaultSpeciesSet, neat.DefaultStagnation,
                         config_file)
    
    p = neat.Population(config) # Creates the population
    winner = p.run(eval_genomes, num_generations) # Runs the population until the fitness threshold is reached
    with open("winner.pkl", "wb") as f:
        pickle.dump(winner, f)
        f.close()



if __name__ == "__main__":

    # Tests on a game
    if not train: 
        config = neat.Config(neat.DefaultGenome, neat.DefaultReproduction,
                            neat.DefaultSpeciesSet, neat.DefaultStagnation,
                            "config.txt")
        print("Loading the best genome...")
        genome = pickle.load(open('winner.pkl', 'rb'))
        print("Genome loaded")
        net = neat.nn.FeedForwardNetwork.create(genome, config)
        # Tests the best genome on a test game
        clock = pg.time.Clock()
        while True:
            t = tetris.Tetris()
            graphic = graphics.Graphic(300, (64, 201, 255), (232, 28, 255), (255, 255, 255), t.board)
            while not t.game_over:
                for event in pg.event.get():
                    if event.type == pg.QUIT:
                        pg.quit()
                        quit()

                state = t.board
                current_block = t.current_piece
                next_block = t.next_piece
                commands = net.activate((current_block, next_block, *flatten_matrix(state)))
                commands = convert_command(commands, 12)
                t.next_pos()
                t.play(*commands)
                graphic.draw()
                clock.tick(5)
            
            print("Game Over \tScore : ", t.score)

    # Trains the model
    else:
        run("config.txt")