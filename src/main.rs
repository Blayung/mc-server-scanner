use std::io::Write;
use std::io::BufRead;
use base64::Engine;

const MASSCAN_LIST_PATH: &str = "./data/masscan-list.txt";
const SERVER_LIST_PATH: &str = "./data/server-list.html";
const PING_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(1);

#[allow(dead_code)]
#[derive(Debug)]
enum ServerPingError {
    TimedOut,
    ConnectionError(std::io::Error),
    PingError(craftping::Error)
}

async fn ping_server(ip: &str, port: u16) -> Result<craftping::Response, ServerPingError> {
    Ok(match match tokio::time::timeout(PING_TIMEOUT, craftping::tokio::ping(&mut match match tokio::time::timeout(PING_TIMEOUT, tokio::net::TcpStream::connect((ip, port))).await {
        Err(_) => return Err(ServerPingError::TimedOut),
        Ok(tcp_stream_result) => tcp_stream_result
    } {
        Err(e) => return Err(ServerPingError::ConnectionError(e)),
        Ok(tcp_stream) => tcp_stream
    }, ip, port)).await {
        Err(_) => return Err(ServerPingError::TimedOut),
        Ok(ping_result) => ping_result
    } {
        Err(e) => return Err(ServerPingError::PingError(e)), 
        Ok(craftping) => craftping
    })
}

fn mc_text_to_string(mut text: craftping::Chat) -> String {
    for extra in text.extra {
        text.text.push_str(&mc_text_to_string(extra));
    }
    text.text
}

#[tokio::main]
async fn main() {
    let mut lines = 0;
    for _ in std::io::BufReader::new(std::fs::File::open(MASSCAN_LIST_PATH).unwrap()).lines() {
        lines += 1;
    }
    
    let mut output = std::fs::OpenOptions::new().append(true).create_new(true).open(SERVER_LIST_PATH).unwrap();

    output.write_all(b"<!DOCTYPE html><html><head><meta charset='UTF-8'><title>Minecraft Server List</title><style>table,th,td{border:1px solid black;border-collapse:collapse;}</style></head><body><table><tbody><tr><th>IP Address</th><th>Port</th><th>Status</th><th>Game Version</th><th>Protocol Version</th><th>Favicon</th><th>Description</th><th>Online Players</th><th>Max Players</th><th>Sample Player</th><th>Enforces Secure Chat?</th><th>Previews Chat?</th><th>Forge Data</th><th>Old Forge Data</th></tr>").unwrap();

    for (line_num, line) in std::io::BufReader::new(std::fs::File::open(MASSCAN_LIST_PATH).unwrap()).lines().enumerate() {
        let line = line.unwrap();
        if line.starts_with('#') {
            continue;
        }

        let mut splitted_line = line.split(' ');

        let port = splitted_line.nth(2).unwrap();
        let ip = splitted_line.next().unwrap();

        println!("Pinging {}:{}... ({}%)", ip, port, ((line_num as f32 / lines as f32) * 100.0) as u8);

        match ping_server(ip, port.parse::<u16>().unwrap()).await {
            Err(e) => output.write_all(format!("<tr><th>{}</th><th>{}</th><th>ERROR ({:?})</th></tr>", ip, port, e).as_bytes()).unwrap(),
            Ok(response) => output.write_all(format!(
                "<tr><th>{}</th><th>{}</th><th>OK</th><th>{}</th><th>{}</th><th>{}</th><th>{}</th><th>{}</th><th>{}</th><th>{}</th><th>{}</th><th>{}</th><th>{}</th><th>{}</th></tr>",
                ip,
                port,
                response.version,
                response.protocol,
                match response.favicon {
                    None => String::new(),
                    Some(favicon) => format!("<img src='data:image/png;base64,{}'>", base64::prelude::BASE64_STANDARD.encode(favicon))
                },
                mc_text_to_string(response.description),
                response.online_players,
                response.max_players,
                match response.sample {
                    None => String::new(),
                    Some(sample) => match sample.first() {
                        None => String::new(),
                        Some(player) => player.name.clone()
                    }
                },
                match response.enforces_secure_chat {
                    None => "?",
                    Some(enforces_secure_chat) => if enforces_secure_chat {
                        "Enforces"
                    } else {
                        "Doesn't enforce"
                    }
                },
                match response.previews_chat {
                    None => "?",
                    Some(previews_chat) => if previews_chat {
                        "Previews"
                    } else {
                        "Doesn't preview"
                    }
                },
                match response.forge_data {
                    None => String::new(),
                    Some(forge_data) => format!("{:?}", forge_data)
                },
                match response.mod_info {
                    None => String::new(),
                    Some(mod_info) => format!("{:?}", mod_info)
                }
            ).as_bytes()).unwrap()
        }
    }

    output.write_all(b"</tbody></table></body></html>").unwrap();
}
