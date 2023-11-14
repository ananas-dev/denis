mod comm;
mod net;
mod pos;
mod search;

fn main() {
    comm::start().unwrap();
}
