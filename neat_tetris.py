import os
import pygame as pg
import numpy as np
import neat
import pickle # Used to save the model
import multiprocessing
import graphics
import engine
import tetris


### TRAINING PARAMETERS ###
train = False


##########################
### NEAT CONFIGURATION ###

pop_size = 16
fitness_threshold = 1000
num_inputs = 4
num_outputs = 1 # Score based on the 7 inputs
num_generations = 100

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
    play_engine = engine.Engine("./target/release/neat-tetris")


    for _, genome in genomes:
        net = neat.nn.FeedForwardNetwork.create(genome, config)
        cleaned_node_evals = []

        for e in net.node_evals:
            a, _, _, b, c, d = e
            cleaned_node_evals.append((a, b, c, d))

        play_engine.load(net.input_nodes, net.output_nodes, cleaned_node_evals)
        genome.fitness = play_engine.play_game()

    play_engine.terminate()

def load_genome(genome_path):
        config = neat.Config(neat.DefaultGenome, neat.DefaultReproduction,
                            neat.DefaultSpeciesSet, neat.DefaultStagnation,
                            "config.txt")
        print("Loading the best genome...")
        genome = pickle.load(open(genome_path, 'rb'))
        print("Genome loaded")
        net = neat.nn.FeedForwardNetwork.create(genome, config)
        return net

def neat_command(current_block, next_block, game_board, net):
    if current_block == None:
        return (None, None)

    t = tetris.Tetris(board=np.array(game_board), current_piece=current_block, next_piece=next_block)
    
    best_move = (None, None)
    max1_score = float("-inf")
    for rot0, col0 in t.gen_legal_moves():
        game_over, t1 = t.apply_move(rot0, col0)
        if game_over: continue

        max2_score = float("-inf")
        for rot1, col1 in t1.gen_legal_moves():
            game_over, t2 = t1.apply_move(rot1, col1)
            if game_over: continue
            t2_score = net.activate((t2.cleared ,*t2.get_stats()))[0]
            if t2_score > max2_score:
                max2_score = t2_score

        if max2_score > max1_score:
            max1_score = max2_score
            best_move = (rot0, col0)

    return best_move[0], best_move[1]

### RUN FUNCTION ###
def run(config_file, retrain=False):
    global p
    if not retrain:
        # Load configuration.
        config = neat.Config(neat.DefaultGenome, neat.DefaultReproduction,
                            neat.DefaultSpeciesSet, neat.DefaultStagnation,
                            config_file)
        
        p = neat.Population(config) # Creates the population
    
    else:
        # Load the last checkpoint
        checkpoints_filenames = [filename for filename in os.listdir(".") if filename.startswith("neat-checkpoint-")]
        checkpoints_filenames.sort()
        filename = checkpoints_filenames[-1]
        p = neat.Checkpointer.restore_checkpoint(filename)
    
    p.add_reporter(neat.StdOutReporter(True))
    p.add_reporter(neat.StatisticsReporter())
    p.add_reporter(neat.Checkpointer(10, None)) # Saves the model every two generations

    winner = p.run(eval_genomes, num_generations) # Runs the population 'number_generations' generations
    pickle.dump(winner, open('winner.pkl', 'wb')) # Saves the best genome
    



if __name__ == "__main__":
    modify_config_file()
    # Tests on a game
    if not train: 
        net = load_genome("winner.pkl")
        # Tests the best genome on a test game
        clock = pg.time.Clock()
        # t = tetris.Tetris()
        play_engine = engine.Engine("./target/release/neat-tetris")

        cleaned_node_evals = []
        for e in net.node_evals:
            a, _, _, b, c, d = e
            cleaned_node_evals.append((a, b, c, d))

        play_engine.load(net.input_nodes, net.output_nodes, cleaned_node_evals)

        pos = play_engine.peek()
        board = np.array(pos["board"]).reshape(22, 12)

        graphic = graphics.Graphic(300, (0, 0, 0), (0, 0, 0), (255, 255, 255), board)
        while True:
            clock.tick(60)

            play_engine.go()
            pos = play_engine.peek()

            graphic.board = np.array(pos["board"]).reshape(22, 12)

            for event in pg.event.get():
                if event.type == pg.QUIT:
                    pg.quit()
                    quit()

            graphic.draw()

        print("Game over !")
        print("Score :", t.score)

        play_engine.terminate()

    # Trains the model
    else:
        run("config.txt", retrain=False)