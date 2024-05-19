cargo build --release
gcc test.c -L target/release/ -lsui_rust_sdk -o test
./test