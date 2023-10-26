use listers::*;

mod connection;
mod listers;
mod types;

fn main() {
    list_window_placements();
    println!("---------------------");
    list_window_rects();
}
