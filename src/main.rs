use std::{io::Read, net::{TcpListener, TcpStream}, process::exit};
fn main() {
    let port : u64 = 7878;
    match TcpListener::bind(format!("localhost:{}",port)) {
        Ok(listener)=>{
            println!("listening on port {}:",port);
            for stream in listener.incoming().map(|s|s.unwrap()) {
                handler(stream);
            }
        }
        Err(_)=>{
            eprintln!("failed to start tcp server,exiting...");
            exit(1);
        }
    }
}

fn handler(mut stream:TcpStream){
    let mut buffer = [0u8;1024];
    stream.read(&mut buffer).unwrap();
    println!("client request:\n{}",String::from_utf8_lossy(&buffer));
}