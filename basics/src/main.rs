mod ownership;
mod expressions;
mod iteration;

fn main() {
    expressions::expressions();
    iteration::iteration();
    ownership::make_memory_move();
}
