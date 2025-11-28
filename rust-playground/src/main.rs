mod basics {
    pub mod print;
    pub mod variables;
    pub mod ownership;
    pub mod borrow;
    pub mod slice;
    pub mod vecs;
    pub mod hashmaps;
}

fn main() {
    basics::print::run();
    basics::variables::run();
    basics::ownership::run();
    basics::borrow::run();
    basics::slice::run();
    basics::vecs::run();
    basics::hashmaps::run();
}