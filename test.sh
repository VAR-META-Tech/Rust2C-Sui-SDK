 cargo build --release
 gcc src/Demo/test.c -L target/release/ -lsui_rust_sdk -o test
 ./test 