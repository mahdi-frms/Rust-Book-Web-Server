use std::{fs, io::{Read, Write}, net::{TcpListener, TcpStream}, process::exit};
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

    let content = fs::read_to_string("view/index.html").unwrap();
    
    let response = format!("HTTP/1.1 200 ok\r\nContent-Lenght:{} \r\n\r\n{}",content.len(),content);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}