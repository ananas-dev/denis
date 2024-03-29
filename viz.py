import neat
import pickle
import visualize

if __name__ == "__main__":
    config = neat.Config(neat.DefaultGenome, neat.DefaultReproduction,
                        neat.DefaultSpeciesSet, neat.DefaultStagnation,
                        "config.txt")
    genome = pickle.load(open("winner.pkl", 'rb'))
    net = neat.nn.FeedForwardNetwork.create(genome, config)
    print(net.input_nodes)
    print(net.output_nodes)
    print(net.node_evals)
    visualize.draw_net(config, genome, True, "test", prune_unused=True)
