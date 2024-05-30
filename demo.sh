 cargo build --release
 gcc src/Demo/demo.c -L target/release/ -lsui_rust_sdk -o src/Demo/demo
 ./src/Demo/demo