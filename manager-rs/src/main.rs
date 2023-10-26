use crate::utility::io::*;
use events::runner::set_window_event;
use listers::*;

mod connection;
mod events;
mod listers;
mod types;
mod utility;

fn main() {
    list_window_placements();
    print_delimiter();
    list_window_rects();
}
