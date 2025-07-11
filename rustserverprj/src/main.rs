use std::{
    env, io::{prelude::*, BufReader}, net::{SocketAddr, TcpListener, TcpStream, UdpSocket}, path::PathBuf
};
fn main() {
    //get current directory for the server
    let cur_dir = env::current_dir().expect("Uh oh");
    println!("{}", cur_dir.display());

    //find local ipv4 address
    let socket = UdpSocket::bind("0.0.0.0:0").expect("couldn't bind to address");
    let google_dns: SocketAddr = "8.8.8.8:80".parse().expect("invalid address format");
    socket.connect(google_dns).expect("couldn't connect to test address");
    let local_addr = socket.local_addr().expect("couldn't get local address");
    let ip_and_port = format!("{}:3001", local_addr.ip()); // only use IP part
    //listen for tcp connections
    let listener = TcpListener::bind(ip_and_port).unwrap();
    for stream in listener.incoming(){
        let stream = stream.unwrap();
        let cdir_clone = cur_dir.clone();
        read_stream(stream, cdir_clone);
    }
}

fn read_stream(mut stream: TcpStream, current_directory: PathBuf){
    //read stream from client
    let b_reader = BufReader::new(&stream);
    //break stream into lines
    let msg: Vec<_> = b_reader.lines().map(|result|result.unwrap())
    .take_while(|line|!line.is_empty()).collect();
    //print message to console
    println!("Message: {msg:#?}");
    //respond
    let response = format!("Cool, thanks. Also, here's my current directory{}", current_directory.display());
    stream.write_all(response.as_bytes()).unwrap();
}