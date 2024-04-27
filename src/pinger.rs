mod config;

use std::io::BufRead;

async fn ping_server(ip: &str, port: u16) -> Option<craftping::Response> {
    tokio::time::timeout(
        config::PING_TIMEOUT,
        craftping::tokio::ping(
            &mut tokio::time::timeout(
                config::PING_TIMEOUT,
                tokio::net::TcpStream::connect((ip, port))
            ).await.ok()?.ok()?,
            ip,
            port
        )
    ).await.ok()?.ok()
}

#[tokio::main]
async fn main() {
    let server_list_path = std::path::Path::new(config::SERVER_LIST_PATH);
    if !server_list_path.exists() {
        std::fs::create_dir_all(server_list_path).unwrap();
    }

    let mut lines = 0;
    for _ in std::io::BufReader::new(std::fs::File::open(config::MASSCAN_LIST_PATH).unwrap()).lines() {
        lines += 1;
    }

    let should_close = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let _should_close = should_close.clone();
    ctrlc::set_handler(move || _should_close.store(true, std::sync::atomic::Ordering::Relaxed)).unwrap();
    
    for (line_num, line) in std::io::BufReader::new(std::fs::File::open(config::MASSCAN_LIST_PATH).unwrap()).lines().enumerate() {
        let line = line.unwrap();
        if line.starts_with('#') {
            continue;
        }

        let mut splitted_line = line.split(' ');

        let port = splitted_line.nth(2).unwrap();
        let ip = splitted_line.next().unwrap();

        let server_list_entry_path = server_list_path.join(format!("{} {}", ip, port));

        let precentage_done = ((line_num as f32 / lines as f32) * 100.0) as u8;

        if server_list_entry_path.exists() {
            println!("Skipping {}:{}... ({}%)", ip, port, precentage_done);
        } else {
            println!("Pinging {}:{}... ({}%)", ip, port, precentage_done);

            std::fs::write(
                server_list_entry_path,
                bincode::serialize(&ping_server(ip, port.parse().unwrap()).await).unwrap()
            ).unwrap();
        }

        if should_close.load(std::sync::atomic::Ordering::Relaxed) {
            println!("Pausing...");
            return;
        }
    }

    println!("Finished! (100%)");
}
