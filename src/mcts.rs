use crate::pos::{Position, Features};

pub struct Node {
    childs: Vec<Node>,
    reward: f64,
    visit_count: f64,
    game: Position,
    parent: Option<Box<Node>>,
    done: bool,
    features: Features,
    action_index: (usize, usize, bool)
}

impl Node {
    pub fn get_ucb_score(&self) -> f64 {
        if self.visit_count == 0. {
            return f64::INFINITY;
        }

        if let Some(top_node) = &self.parent {
            (self.visit_count / self.reward) + f64::sqrt(f64::ln(top_node.reward) / self.reward)
        } else {
            (self.visit_count / self.reward) + f64::sqrt(f64::ln(self.reward) / self.reward)
        }
    }
}