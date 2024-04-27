# Minecraft Server Scanner
## WARNING: You can get on ip blacklists for scanning the whole internet!
## Requirements:
- [Cargo](https://rustup.rs)
- [Masscan](https://github.com/robertdavidgraham/masscan)
## Usage:
1. Scan the internet for open 25565 ports using masscan and put the results in the list format in `data/masscan-list.txt` (you can use the script in `data/start-masscan.sh`).
2. Start the pinger with `cargo run --bin pinger` (the server list will be put in the `data/server-list` directory).
3. Start the http server with the server list with `cargo run --bin server-list`.
4. View the server list by going to `http://localhost:8888` in your browser.
### You can also configure the program a bit in `src/config.rs`.
