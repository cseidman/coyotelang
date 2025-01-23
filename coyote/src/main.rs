extern crate core;

mod cli;

fn main() {
    println!(
        r#"
 ▄████████  ▄██████▄  ▄██   ▄    ▄██████▄      ███        ▄████████
███    ███ ███    ███ ███   ██▄ ███    ███ ▀█████████▄   ███    ███
███    █▀  ███    ███ ███▄▄▄███ ███    ███    ▀███▀▀██   ███    █▀
███        ███    ███ ▀▀▀▀▀▀███ ███    ███     ███   ▀  ▄███▄▄▄
███        ███    ███ ▄██   ███ ███    ███     ███     ▀▀███▀▀▀
███    █▄  ███    ███ ███   ███ ███    ███     ███       ███    █▄
███    ███ ███    ███ ███   ███ ███    ███     ███       ███    ███
████████▀   ▀██████▀   ▀█████▀   ▀██████▀     ▄████▀     ██████████
    "#
    );

    println!(
        r#"
    Coyote Copyright (C) 2025 Claude Seidman: Qubescript Ltd.
    This program comes with ABSOLUTELY NO WARRANTY; for details
    type `coyote --show w' on the command line. This is free software,
    and you are welcome to redistribute it under certain conditions;
    type `coyote --show c' on the command line for details.
    "#
    );

    if let Err(e) = cli::run() {
        println!("Error: {}", e);
    }
}
