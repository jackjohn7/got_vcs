use std::{fs, fmt::Debug};


pub fn handle_init_command() {
    fs::create_dir(".got").expect("Could not initialize got repo");
    match fs::File::create(".got/gotdb.sqlite") {
        Ok(_) => {
            println!("File created: ./got/gotdb.sqlite");
        },
        Err(e) => {
            eprintln!("{}", e.to_string());
        }
    }
}
