mod comm;
mod net;
mod pos;
mod search;
mod transposition;

fn main() {
    comm::start().unwrap();
}
