# sui_rust_sdk

# Install Rust

Follow this guide to install Rust https://www.rust-lang.org/tools/install

# Install Sui

Follow this guide to install Sui https://docs.sui.io/guides/developer/getting-started/sui-install

# Request tokens to pay for the free

Run follow command to setting Envỉroment befor testing:
1. Check Sui Client Environment:  
   
    ```sh 
    Sui client envs
    ```
 **NOTE:If you dont have DevNet Please Run CMD :**

    sui client new-env --alias devnet --rpc https://fullnode.devnet.sui.io:443
    

2. Switch to devnet network: 
   
    ```sh 
    sui client switch --env devnet
    ```


3. Check current network:
   
    ```sh 
    sui client active-env
    ```
 **NOTE: The return should be devnet**
 
4. Get the active address: 

    ```sh
    sui client active-address
    ```

5. Request token:

    ```sh
    sui client faucet 
    ```

 **NOTE: Wait for 60s to get the tokens**

6. Check the gas coin objects for the active address: 
   
    ```sh
    sui client gas
    ```

# Set up done. Run test code with sh:

Run the following command to test:
 ```sh
sh test.sh
 ```




