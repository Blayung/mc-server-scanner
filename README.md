# Minecraft Server Scanner
## WARNING: You can get on ip blacklists for scanning the whole internet!
## Requirements:
- [Cargo](https://rustup.rs)
- [Masscan](https://github.com/robertdavidgraham/masscan)
## Usage:
1. Scan the internet for open 25565 ports using masscan and put the results in the list format in `data/masscan-list.txt` (you can use the `data/start-masscan.sh` and `data/resume-masscan.sh` scripts).
2. Start the pinger with `cargo run --bin pinger` (you can pause with ^C and resume by launching it again).
3. Start the http server with the server list with `cargo run --bin server-list`.
4. View the server list by going to `http://localhost:8888` in your browser.
### You can also configure the program a bit in `src/config.rs`.
### This scanner currently pings the servers synchronously, one-by-one, so it might be a little slower than an asyncronous implementation, but from my experience, it'll scan all the open 25565 ports in the ipv4 internet in ~16h.
