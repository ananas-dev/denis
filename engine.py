import subprocess
import json
import tpn

class Engine:
    def __init__(self, path):
        self.process = subprocess.Popen([path],
                                        stdin=subprocess.PIPE,
                                        stdout=subprocess.PIPE,
                                        text=True)

    def send_message(self, message):
        serialized_message = json.dumps(message)
        self.process.stdin.write(serialized_message + '\n')
        self.process.stdin.flush()

    def receive_message(self):
        while True:
            raw_message = self.process.stdout.readline()
            try:
                return json.loads(raw_message)
            except:
                assert False, "Unknown message:" + raw_message

    def load(self, input_nodes, output_nodes, node_evals):
        msg = {
            "type": "Load",
            "input_nodes": input_nodes,
            "output_nodes": output_nodes,
            "node_evals": node_evals
        }

        self.send_message(msg)

    def play_game(self):
        msg = { "type": "PlayGame" }
        self.send_message(msg)
        response = self.receive_message()
        return response["score"]

    def pos(self, tpn):
        msg = { "type": "Pos", "tpn": tpn }
        self.send_message(msg)

    def go(self):
        msg = { "type": "Go" }
        self.send_message(msg)
        return self.receive_message()

    
    def peek(self):
        msg = { "type": "Peek" }
        self.send_message(msg)
        out = self.receive_message()
        print(out)
        return tpn.loads(out["tpn"])

    def ready(self):
        msg = { "type": "Ready" }
        self.send_message(msg)
        return self.receive_message()

    def terminate(self):
        self.process.terminate()