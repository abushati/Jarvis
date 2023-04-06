use std::process::Command;
use sqlite::Connection;

fn main() {
    // let output = Connection::open("../../Jarvis/jarvis.db");

    match Connection::open("../../jarvis.db") {
        Ok(_) => println!("SQLite database is running"),
        Err(e) => {
            let output = Command::new("sudo")
                         .arg("apt-get")
                         .arg("-y")
                         .arg("install")
                         .arg("sqlite3")
                         .arg("libsqlite3-dev")
                         .output()
                         .expect("failed to execute apt-get command");
            if output.status.success() {
                println!("SQLite packages installed successfully.");
            } else {
                let error_message = String::from_utf8_lossy(&output.stderr);
                println!("Failed to install SQLite packages: {}", error_message);
            }
        }
    }

    let _ = Command::new("./target/debug/diskmanager").spawn().unwrap();
    let _  = Command::new("./target/debug/webserver").spawn().unwrap();
}