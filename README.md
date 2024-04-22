# Minecraft Server Scanner
## Usage:
1. Scan the internet for open 25565 ports using masscan and put the results in the list format in `data/masscan-list.txt` (you can use the script in `data/start-masscan.sh`).
2. Start the pinger (`cargo run --bin pinger`) - the server list will be put in `data/server-list.dat`.
3. Start the http server allowing for easy server list browsing (`cargo run --bin html-gui-server`).
4. View the server list by going to `http://localhost:8888` in your browser.
