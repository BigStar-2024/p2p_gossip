mod cli;

use cli::get_args;
use std::net::{SocketAddr, UdpSocket};
use std::thread;
use std::time::Duration;

fn serialize_vec_socketaddr(vec_sockaddr: Vec<SocketAddr>, _my_addr: SocketAddr) -> String {
    let mut serialized_data = "add".to_string() + &_my_addr.to_string();
    for addr in vec_sockaddr {
        serialized_data = serialized_data + "," + &format!("{}", addr);
    }
    serialized_data
}

fn deserialize_vec_socketaddr(serialized_data: String, _my_addr: SocketAddr) -> Vec<SocketAddr> {
    let mut vec_sockaddr = Vec::new();
    let vec_string: Vec<&str> = serialized_data.split(',').collect();
    for addr_str in vec_string { 
        let addr = format!("{}", addr_str).parse().unwrap();
        if _my_addr != addr {
            println!("Connected to the peers at {}", addr_str);
            vec_sockaddr.push(addr);
        }
    }
    vec_sockaddr
}
fn main() {
    let args = get_args();
    let mut timer = 0;
    let my_addr: SocketAddr = format!("127.0.0.1:{}", args.port).parse().unwrap();
    println!("My address is '{}'", my_addr);
    let period = args.period;
    let mut target_address: Vec<SocketAddr> = vec![];
    let random_message = "hello";
    let txt_message = String::from("txt");
    let message = txt_message + &random_message;
    let socket = UdpSocket::bind(my_addr).expect("Failed to bind socket");
    let duration = Duration::from_millis(1);

    socket.set_nonblocking(true).expect("Failed to set non-blocking mode");
    
    if let Some(connect) = args.connect {
        let connect_message = "con"; // Convert to String
        let send_addr: SocketAddr = format!("127.0.0.1:{}", connect).parse().unwrap();
        socket.send_to(connect_message.as_bytes(), send_addr).expect("connection failed");
    }
    
    loop {
        if timer >= period * 1000 {
            for addr in &target_address {
                socket.send_to(message.as_bytes(), addr).expect("Failed to send message");
                println!("Sending message {} to {}", random_message, addr);
            }
            timer = 0;
        }

        thread::sleep(duration);
        timer += 1;

        let mut buf = [0; 1024];
        match socket.recv_from(&mut buf) {
            Ok((num_bytes, src_addr)) => {
                let received_message = String::from_utf8_lossy(&buf[3..num_bytes]);
                if &buf[..3] == b"txt" {
                    println!("Received message {} from {}", received_message, src_addr);
                } else if &buf[..3] == b"con" {
                    target_address.push(src_addr);
                    let serialized_data = serialize_vec_socketaddr(target_address.clone(), my_addr);
                    for addr in &target_address {
                        socket.send_to(serialized_data.as_bytes(), addr).expect("Failed to send message");
                    }
                } else {
                    println!("receive address list {}", received_message);
                    let socket_addr = deserialize_vec_socketaddr(received_message.to_string(), my_addr);
                    target_address = socket_addr;
                }
            }
            Err(ref err) if err.kind() == std::io::ErrorKind::WouldBlock => {
                // No message available, continue to the next iteration
                continue;
            }
            Err(err) => {
                eprintln!("Failed to receive message: {}", err);
                break;
            }
        }
    }
}
