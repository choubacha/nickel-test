#[macro_use]
extern crate nickel;
use nickel::{Nickel, HttpRouter};

fn main() {
    let mut server = Nickel::new();
    server.get("/hello", middleware!("world"));
    server.listen("0.0.0.0:1643");
}
