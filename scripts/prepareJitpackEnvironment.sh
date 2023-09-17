curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"
export PATH="$PATH:$HOME/.cargo/env:$HOME/.cargo/bin"
rustup target add armv7-linux-androideabi
rustup target add i686-linux-android
rustup target add aarch64-linux-android
rustup target add x86_64-linux-android
which python3
apt-get install python3.9