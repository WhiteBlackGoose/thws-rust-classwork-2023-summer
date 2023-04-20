use std::{net::{UdpSocket, TcpStream}, io::{Write, Read}};

use regex::Regex;

fn main() {
    let mut stream = TcpStream::connect("fiw.fhws.de:80").unwrap();
    stream.write(String::from("hello\n").as_bytes()).unwrap();
    let mut reply : [u8; 2048] = [0; 2048];
    stream.read(&mut reply).unwrap();
    let response = String::from_utf8(reply.to_vec()).unwrap();
    let re = Regex::new(r"Server: (.+)").unwrap();
    let m = re.captures(response.as_str());
    if let Some(mat) = m {
        println!("{}", mat.get(1).unwrap().as_str());
    } else {
        print!("Ohno! \n{}", response);
    }
}
