# Minecraft Server Scanner
## Requirements:
- [Cargo](https://rustup.rs)
- [Masscan](https://github.com/robertdavidgraham/masscan)
- WARNING: You can get on ip blacklists by scanning the whole internet!
## Usage:
1. Scan the internet for open 25565 ports using masscan and put the results in the list format in `data/masscan-list.txt` (you can use the script in `data/start-masscan.sh`).
2. Start the pinger with `cargo run --bin pinger` (the server list will be put in `data/server-list.dat`).
3. Start the http server allowing for easy server list browsing with `cargo run --bin html-gui-server`.
4. View the server list by going to `http://localhost:8888` in your browser.
