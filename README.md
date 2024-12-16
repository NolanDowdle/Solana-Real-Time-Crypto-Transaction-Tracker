# Solana-Real-Time-Crypto-Transaction-Tracker
Solana Real-Time Crypto Transaction Tracker


## Set Up
To get the Real-Time Crypto tracker set up,
If you do not have Homebrew installed, install it here:
```/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"```

If you do not have Rust installed, install it here:
```curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh```

1. have redis installed through```brew install redis```
2. in the home directory run ./start_redis_cluster.sh
3. execute ```cargo build```
4. execute ```cargo run```