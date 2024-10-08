use std::env;
use data_tree::*;
pub mod nested;
pub mod data_tree;

//
fn main() {
    env::set_var("RUST_LOG", "debug");
    env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let tree = Parent::new("".to_string());
    println!("tree: {:#?}", tree);
    println!("tree.path: {:#?}", tree.path());
    println!("tree.child.path: {:#?}", tree.child.path());
}