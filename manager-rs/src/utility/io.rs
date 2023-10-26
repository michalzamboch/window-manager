pub fn wait_for_input() {
    let mut line = String::new();
    std::io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
}

pub fn print_delimiter() {
    println!("---------------------");
}
