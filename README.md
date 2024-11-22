cargo build --release --target armv7-unknown-linux-gnueabihf
cargo build --release --target aarch64-unknown-linux-gnu

scp target/armv7-unknown-linux-gnueabihf/release/pong mrbueno@192.168.1.61:/home/mrbueno/

chmod +x <binary-name>
./<binary-name>
