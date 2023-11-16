mod comm;
mod net;
mod pos;
mod search;
mod transposition;
mod mcts;

fn main() {
    comm::start().unwrap();
}
