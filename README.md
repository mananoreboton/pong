cargo build --release --target armv7-unknown-linux-gnueabihf
cargo build --release --target aarch64-unknown-linux-gnu

scp target/armv7-unknown-linux-gnueabihf/release/<binary-name> pi@<raspberry-pi-ip>:/home/pi/

chmod +x <binary-name>
./<binary-name>
