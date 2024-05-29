cargo build --release
gcc test2.c -L target/release/ -lsui_rust_sdk -o test2
./test2