
use std::env;
// use wallet::

fn main() {
    println!("-> start");

    let args: Vec<String> = env::args().collect();
    println!("-> cmd: '{:?}'", args);

    println!("-> end");
}
