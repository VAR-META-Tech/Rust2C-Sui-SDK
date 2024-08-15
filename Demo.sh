cargo build --release
gcc Demo.c -L target/release/ -lsui_rust_sdk -o Demo
./Demo
