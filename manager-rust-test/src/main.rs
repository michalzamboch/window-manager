use types::ConnectionTrait;
use window_reader::Connection;

mod types;
mod window_reader;

fn main() {
    let connection = Connection::new().unwrap();
    let titles = connection.window_titles().unwrap();
    let count = titles.len();

    titles.into_iter().for_each(|title| println!("{}", title));
    println!("Count: {}", count);
}
