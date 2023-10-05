use fixedstr::fstr;
use limousine_engine::prelude::*;

create_hybrid_index! {
    name: Index,
    path: "layout.config",
}

fn main() {
    let mut index = Index::<i32, fstr<32>>::empty();

    index.insert(0, fstr::from("Hello, World!"));
    index.insert(1, fstr::from("Hello, World"));
    index.insert(2, fstr::from("Hello, Worl"));
    index.insert(3, fstr::from("Hello, Wor"));
    index.insert(4, fstr::from("Hello, Wo"));
    index.insert(5, fstr::from("Hello, W"));
    index.insert(6, fstr::from("Hello, "));
    index.insert(7, fstr::from("Hello,"));

    dbg!(index.search(&5).unwrap());
}
