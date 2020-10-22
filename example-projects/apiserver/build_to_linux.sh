

# see: https://www.qttc.net/529-rust-cross-compile-mac-to-linux.html
CC_x86_64_unknown_linux_musl="x86_64-linux-musl-gcc" cargo build --release --target=x86_64-unknown-linux-musl
