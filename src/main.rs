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

    let index_html = b"GET / HTTP/1.1\r\n";
    if buffer.starts_with(index_html) {
        serve_page(stream,"HTTP/1.1 200 OK","view/index.html");
    }
    else{
        serve_page(stream,"HTTP/1.1 404 NOT FOUND","view/404.html");
    }
}

fn serve_page(mut stream:TcpStream,status_line:&str,path:&str){
    let content = fs::read_to_string(path).unwrap();
    let response = format!("{}\r\nContent-Lenght:{} \r\n\r\n{}",status_line,content.len(),content);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}