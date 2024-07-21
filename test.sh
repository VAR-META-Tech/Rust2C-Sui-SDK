cargo build --release
gcc testJson.cpp -L target/release/ -lsui_rust_sdk -o test
./test




