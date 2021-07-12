use std::{net::TcpListener, process::exit};
fn main() {
    let port : u64 = 7878;
    match TcpListener::bind(format!("localhost:{}",port)) {
        Ok(listener)=>{
            println!("listening on port {}:",port);
            for _ in listener.incoming().map(|s|s.unwrap()) {
                println!("connected!");
            }
        }
        Err(_)=>{
            eprintln!("failed to start tcp server,exiting...");
            exit(1);
        }
    } 
}