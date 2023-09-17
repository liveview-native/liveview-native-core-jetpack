curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"
export PATH="$PATH:$HOME/.cargo/env:$HOME/.cargo/bin"
rustup target add armv7-linux-androideabi
rustup target add i686-linux-android
rustup target add aarch64-linux-android
rustup target add x86_64-linux-android
rustc --version
cargo --version
export RUST_ANDROID_GRADLE_RUSTC_COMMAND="$HOME/.cargo/bin"