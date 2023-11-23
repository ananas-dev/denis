import os
import numpy as np
import neat
import pickle # Used to save the model
import graphics
import engine


### TRAINING PARAMETERS ###
train = False


##########################
### NEAT CONFIGURATION ###

pop_size = 16
fitness_threshold = 1000
num_inputs = 4
num_outputs = 1 # Score based on the 7 inputs
num_generations = 1000000

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

def eval_genome(genome, config):
    play_engine = engine.Engine("./target/release/neat-tetris")

    net = neat.nn.FeedForwardNetwork.create(genome, config)
    cleaned_node_evals = []

    for e in net.node_evals:
        a, _, _, b, c, d = e
        cleaned_node_evals.append((a, b, c, d))

    play_engine.load(net.input_nodes, net.output_nodes, cleaned_node_evals)
    return play_engine.play_game()

def load_genome(genome_path):
        config = neat.Config(neat.DefaultGenome, neat.DefaultReproduction,
                            neat.DefaultSpeciesSet, neat.DefaultStagnation,
                            "config.txt")
        print("Loading the best genome...")
        genome = pickle.load(open(genome_path, 'rb'))
        print("Genome loaded")
        net = neat.nn.FeedForwardNetwork.create(genome, config)
        return net

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
        checkpoints_filenames = [filename for filename in os.listdir(".") if filename.startswith("neat-checkpoint-99")]
        checkpoints_filenames.sort()
        filename = checkpoints_filenames[-1]
        p = neat.Checkpointer.restore_checkpoint(filename)
    
    p.add_reporter(neat.StdOutReporter(True))
    p.add_reporter(neat.StatisticsReporter())
    p.add_reporter(neat.Checkpointer(500, None)) # Saves the model every two generations

    evaluator = neat.ParallelEvaluator(16, eval_genome)

    winner = p.run(evaluator.evaluate, num_generations) # Runs the population 'number_generations' generations
    pickle.dump(winner, open('winner.pkl', 'wb')) # Saves the best genome

if __name__ == "__main__":
    modify_config_file()
    # Tests on a game
    if not train: 
        net = load_genome("nes-strong-2.pkl")
        # Tests the best genome on a test game
        play_engine = engine.Engine("./target/release/neat-tetris")

        cleaned_node_evals = []
        for e in net.node_evals:
            a, _, _, b, c, d = e
            cleaned_node_evals.append((a, b, c, d))

        play_engine.load(net.input_nodes, net.output_nodes, cleaned_node_evals)

        pos = play_engine.peek()

        board = np.array(pos["board"])

        graphic = graphics.Graphic(300, (0, 0, 0), (0, 0, 0), (255, 255, 255), board, 30)
        while True:
            move = play_engine.go()

            action_list = move["action_list"]

            graphic.action_list = action_list


            graphic.board = np.array(pos["board"])
            graphic.current_piece = pos["current_piece"]
            graphic.next_pieces = [pos["next_piece"]]
            graphic.score = pos["score"]

            graphic.tick()
            graphic.draw()

            pos = play_engine.peek()

        print("Game over !")
        print("Score :", t.score)

        play_engine.terminate()

    # Trains the model
    else:
        run("config.txt", retrain=False)