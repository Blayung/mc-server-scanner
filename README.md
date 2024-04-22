# Minecraft Server Scanner
## Usage:
1. Scan the internet for minecraft servers (`-p25565`) using masscan and put the results in the list format in `data/masscan-list.txt`.
2. Start the pinger (`cargo run --bin pinger`).
3. Start the http server allowing for easy server list browsing (`cargo run --bin html-gui-server`).
4. Use the gui by going to `http://localhost:8888` in your browser.
