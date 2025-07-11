use serde_json::json;
use std::{
    env, 
    io::{prelude::*, BufReader}, 
    net::{SocketAddr, TcpListener, TcpStream, UdpSocket}, 
    path::PathBuf,
    fs
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
    let mut entire_path = Some(current_directory.as_path());
    let mut directories: Vec<String> = Vec::new();

    //make an array containing each directory 
    while let Some(dir) = entire_path{
       if let Some(name) = dir.file_name().and_then(|n| n.to_str()) {
            directories.push(name.to_string());
        } else {
            directories.push("C".to_string()); // fallback for root or unknown
        }
        entire_path = dir.parent();
    }

    let last_path = PathBuf::from(&directories[0]); 
    //get the contents of the server's directory
    let contents = fs::read_dir(&last_path)
        .unwrap_or_else(|_| fs::read_dir(".").unwrap())
        .filter_map(|entry| entry.ok().map(|e| e.file_name().to_string_lossy().to_string()))
        .collect::<Vec<_>>();
    //formulate a json response for the app to read
    let response_json = json!({
        "directories": directories,
        "last_dir_contents": contents
    }).to_string() + "\n";
    stream.write_all(response_json.as_bytes()).unwrap();
}