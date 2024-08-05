<p align="center">
	<img src="./Resource/SuiLogo.png" alt="Unreal-Sui-SDKLogo" width="256" height="128" />
</p>


# Rust2C-Sui-SDK #

Rust2C-Sui-SDK is a package to help developers integrate Sui blockchain technology into their C++ and C# projects.

- [Project Layout](#project-layout)
- [Features](#features)
- [Requirements](#requirements)
- [Dependencies](#dependencies)
- [Installation](#installation)
- [Using Rust2C-Sui-SDK](#using-rust2c-sui-sdk)
- [Examples](#examples)
- [License](#license)

### Project Layout ###  

1. **`header/`**:: This directory contains the header files to use the functions in the library.
2. **`Resource/`**:: A place for various resources needed for the project, like images, data files, or other assets.
3. **`Src/`**: This directory contains the project's source code.

### Features ###

- [x]  Compatibility with main, dev, and test networks.
- [x]  Comprehensive Unit and Integration Test coverage.



### Requirements ###

| Platforms                              | Status       |
| -------------------------------------- | ------------ | 
| Mac / Linux                            | Fully Tested |


### Dependencies
- https://github.com/VAR-META-Tech/Unreal-Sui-SDK
- https://github.com/VAR-META-Tech/Unity-Sui-SDK
  
### Installation ###
# Installation Guide

This guide provides step-by-step instructions for installing and setting up on macOS platforms. Ensure you have the following prerequisites installed to build the project:

## Prerequisites
- **Visual Studio Code** with C++ development environment
- **Install Sui** Follow this guide to install Sui https://docs.sui.io/guides/developer/getting-started/sui-install
## Project Setup
Run follow command to setting Envỉroment befor testing:
1. Check Sui Client Environment:  
    ```sh 
    Sui client envs
    ```
 **NOTE:If you dont have DevNet Please Run CMD :**
```sh 
    sui client new-env --alias devnet --rpc https://fullnode.devnet.sui.io:443
```
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

### Using Unreal-Sui-SDK
Unreal-Sui-SDK can integrate into your own Unreal projects.
We have two options to run the project:

   1. Open the Unreal project directly through the SuisdkUnreal.uproject file and then click the Run icon to test.
    
   2. Open the project using Visual Studio Code, build the source code, and run the test.



### Examples ###

The SDK comes with several examples that show how to leverage the Rust2C-Sui-SDK to its full potential. The examples include Wallet Creation and Management, Token Transfers,  NFT Minting, Account Funding, and Multi-signature.


### License ###
This project is licensed under the Apache-2.0 License. Refer to the LICENSE.txt file for details.
