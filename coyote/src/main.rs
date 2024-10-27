mod cli;

fn main() {
    if let Err(e) = cli::run() {
        println!("Error: {}", e);
    }
}

