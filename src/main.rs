use std::io;

fn main() {
    println!("Select day: ");
    let mut line = String::new();

    loop {
        line.clear();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line!");

        /* Check exit options */
        match line.trim() {
            "exit" | "q" => break,
            "" => continue,
            _ => (),
        };
    }
}
