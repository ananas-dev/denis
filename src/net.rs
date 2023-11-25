use rustc_hash::FxHashMap;

pub struct FeedForwardNetwork {
    input_nodes: Vec<i64>,
    output_nodes: Vec<i64>,
    node_evals: Vec<(i64, f64, f64, Vec<(i64, f64)>)>,
    values: FxHashMap<i64, f64>,
}

impl FeedForwardNetwork {
    pub fn new(
        inputs: Vec<i64>,
        outputs: Vec<i64>,
        node_evals: Vec<(i64, f64, f64, Vec<(i64, f64)>)>,
    ) -> Self {
        let mut values = FxHashMap::default();
        for key in inputs.iter().chain(outputs.iter()) {
            values.insert(*key, 0.0 as f64);
        }

        FeedForwardNetwork {
            input_nodes: inputs,
            output_nodes: outputs,
            node_evals,
            values,
        }
    }

    pub fn activate(&mut self, inputs: Vec<f64>) -> Vec<f64> {
        if self.input_nodes.len() != inputs.len() {
            panic!(
                "Expected {} inputs, got {}",
                self.input_nodes.len(),
                inputs.len()
            );
        }

        for (key, value) in self.input_nodes.iter().zip(inputs) {
            *self.values.get_mut(key).unwrap() = value;
        }

        for (node, bias, response, links) in &self.node_evals {
            let node_inputs: Vec<f64> = links.iter().map(|(i, w)| self.values[i] * w).collect();
            let s: f64 = node_inputs.iter().sum();
            self.values.insert(*node, (bias + response * s).tanh());
        }

        self.output_nodes
            .iter()
            .map(|&i| *self.values.get(&i).unwrap())
            .collect()
    }
}
