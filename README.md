# sui_rust_sdk

# Install Rust

Follow this guide to install Rust https://www.rust-lang.org/tools/install

# Install Sui

Follow this guide to install Sui https://docs.sui.io/guides/developer/getting-started/sui-install

# Request tokens to pay for the free

Run follow command
1. Switch to devnet network: sui client switch --env devnet
2. Check current network: sui client active-env (the return should be devnet)
3. Get the active address: sui client active-address
4. Request token: sui client faucet (wait for 60s to get the tokens)
5. Check the gas coin objects for the active address: sui client gas

# Optional 

Run your local network :
https://docs.sui.io/guides/developer/getting-started/local-network

# Set up done. Run test code with sh:

Run the following command:
sh test.sh





