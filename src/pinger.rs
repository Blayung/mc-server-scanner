mod common;

use std::io::BufRead;

const MASSCAN_LIST_PATH: &str = "./data/masscan-list.txt";
const PING_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(1);

async fn ping_server(ip: &str, port: u16) -> Result<craftping::Response, ()> {
    Ok(match match tokio::time::timeout(PING_TIMEOUT, craftping::tokio::ping(&mut match match tokio::time::timeout(PING_TIMEOUT, tokio::net::TcpStream::connect((ip, port))).await {
        Err(_) => return Err(()),
        Ok(tcp_stream_result) => tcp_stream_result
    } {
        Err(_) => return Err(()),
        Ok(tcp_stream) => tcp_stream
    }, ip, port)).await {
        Err(_) => return Err(()),
        Ok(ping_result) => ping_result
    } {
        Err(_) => return Err(()),
        Ok(ping_response) => ping_response
    })
}

#[tokio::main]
async fn main() {
    if !std::path::Path::new(common::SERVER_LIST_PATH).exists() {
        std::fs::write(common::SERVER_LIST_PATH, bincode::serialize::<Vec<common::ServerListEntry>>(&Vec::new()).unwrap()).unwrap();
    }

    let mut lines = 0;
    for _ in std::io::BufReader::new(std::fs::File::open(MASSCAN_LIST_PATH).unwrap()).lines() {
        lines += 1;
    }
    
    for (line_num, line) in std::io::BufReader::new(std::fs::File::open(MASSCAN_LIST_PATH).unwrap()).lines().enumerate() {
        let line = line.unwrap();
        if line.starts_with('#') {
            continue;
        }

        let mut splitted_line = line.split(' ');

        let port = splitted_line.nth(2).unwrap();
        let ip = splitted_line.next().unwrap();

        println!("Pinging {}:{}... ({}%)", ip, port, ((line_num as f32 / lines as f32) * 100.0) as u8);

        let mut server_list: Vec<common::ServerListEntry> = bincode::deserialize(&std::fs::read(common::SERVER_LIST_PATH).unwrap()).unwrap();

        server_list.push(common::ServerListEntry {
            ip: ip.to_owned(),
            port: port.to_owned(),
            ping_response: ping_server(ip, port.parse().unwrap()).await
        });

        std::fs::write(common::SERVER_LIST_PATH, bincode::serialize(&server_list).unwrap()).unwrap();
    }
}
