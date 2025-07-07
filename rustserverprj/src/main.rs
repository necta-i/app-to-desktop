use std::{
    net::{TcpListener, TcpStream, UdpSocket, SocketAddr},
    io::{BufReader, prelude::*},
};
fn main() {
    let socket = UdpSocket::bind("0.0.0.0:0").expect("couldn't bind to address");

    let google_dns: SocketAddr = "8.8.8.8:80".parse().expect("invalid address format");
    socket.connect(google_dns).expect("couldn't connect to test address");

    let local_addr = socket.local_addr().expect("couldn't get local address");
    let ip_and_port = format!("{}:3001", local_addr.ip()); // only use IP part
    let listener = TcpListener::bind(ip_and_port).unwrap();
    for stream in listener.incoming(){
        let stream = stream.unwrap();
        readStream(stream);
    }
}

fn readStream(mut stream: TcpStream){
    let bReader = BufReader::new(&stream);
    let msg: Vec<_> = bReader.lines().map(|result|result.unwrap())
    .take_while(|line|!line.is_empty()).collect();
    println!("Message: {msg:#?}");
    let response = "Cool, thanks\n";
    stream.write_all(response.as_bytes()).unwrap();
}