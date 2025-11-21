mod basics {
    pub mod print;
    pub mod variables;
    pub mod ownership;
    pub mod borrow;
}

fn main() {
    basics::print::run();
    basics::variables::run();
    basics::ownership::run();
    basics::borrow::run();
}