#[macro_use]
extern crate f4n_wcf_core;

mod tiles;

#[derive(Debug)]
enum Hoge {
    Fuga
}

fn main() {
    println!("{:#?}", enums!(Hoge:{Fuga, Fuga}));

    f4n_wcf_visualizer::start();
}
