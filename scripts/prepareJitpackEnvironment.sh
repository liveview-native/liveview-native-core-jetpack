curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"
rustup install nightly
rustup default nightly
rustup target add armv7-linux-androideabi
rustup target add i686-linux-android
rustup target add aarch64-linux-android
rustup target add x86_64-linux-android
export RUST_ANDROID_GRADLE_RUSTC_COMMAND="$HOME/.cargo/bin/rustc"
export RUST_ANDROID_GRADLE_CARGO_COMMAND="$HOME/.cargo/bin/cargo"
export RUST_ANDROID_GRADLE_RUSTUP_CHANNEL="nightly"