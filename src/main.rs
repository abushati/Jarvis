use std::{env, fs};

fn main() {
    let current_dir = env::current_dir();
    println!(
        "Entries modified in the last 24 hours in {:?}:",
        current_dir
    );
    let dire =  current_dir.unwrap();
    println!("{}", dire.display());
}


fn walk_directory(directory:&str) {
    // directory.replace("Hello", "replaced");
    println!("{}", directory);
}