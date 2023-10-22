use crate::{
    types::{ConnectionTrait, Result},
    window_reader::{get_window_all_titles, Connection},
};

pub fn list_all_window() {
    let titles = get_window_all_titles();
    list_windows(titles);
}

fn list_windows(window_titles: Vec<String>) {
    for title in window_titles {
        println!("{}", title);
    }
}

fn list_filtered_windows() {
    let connection = Connection::new().unwrap();
    let titles = connection.window_titles().unwrap();
    let count = titles.len();

    titles.into_iter().for_each(|title| println!("{}", title));
    println!("Count: {}", count);
}
