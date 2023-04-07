use std::process::Command;
use sqlite::Connection;
use std::env;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use ctrlc;

fn main() {
    
    // Create a flag to indicate whether the parent process is running
    let running = Arc::new(AtomicBool::new(true));
    let running_clone = running.clone();
    let mut children_processes = vec![];
    
    
    // let output = Connection::open("../../Jarvis/jarvis.db");
    let output = Command::new("hostname")
    .output()
    .expect("failed to execute process");
    println!("{:?}",String::from_utf8_lossy(&output.stdout));
    
    let diskmanager_child: std::process::Child;
    let webserver:std::process::Child;
    if String::from_utf8_lossy(&output.stdout) != "Arvids-MacBook-Pro.local\\n" {
        webserver  = Command::new("./target/debug/webserver").spawn().unwrap();
        diskmanager_child = Command::new("./target/debug/diskmanager").spawn().unwrap();
        
    } else {
        webserver  = Command::new("./webserver").spawn().unwrap();
        diskmanager_child = Command::new("./diskmanager").spawn().unwrap();
        
    }
    children_processes.push(diskmanager_child);
    children_processes.push(webserver);

       // Register a signal handler for SIGTERM
    ctrlc::set_handler(move || {
        println!("Stoping");
        // Set the flag to indicate that the parent process is no longer running
        running_clone.store(false, Ordering::SeqCst);
        for c in &mut children_processes {
            // Kill the child process
            println!("Stoping {:?}",&c);
            c.kill().expect("failed to kill child process");
        }
        while running.load(Ordering::SeqCst) {
            for cd in &mut children_processes{
                println!("Stoping {:?}",&cd);
                while cd.try_wait().unwrap().is_none() {}
            }
        } 
        
    })
    .expect("failed to register signal handler");

    // Wait for the child process to exit


}