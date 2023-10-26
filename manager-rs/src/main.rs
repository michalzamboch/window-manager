use crate::utility::io::*;
use listers::*;

mod connection;
mod listers;
mod types;
mod utility;

fn main() {
    list_window_placements();
    print_delimiter();
    list_window_rects();
}
