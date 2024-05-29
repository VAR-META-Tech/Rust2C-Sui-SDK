#include <stdio.h>

#include "sui_rust_wrapper_c.h"

int main()
{

    // int test_result = test();
    // if (test_result == 0)
    // {
    //     printf("Demo test succeeded.\n");
    // }
    // else
    // {
    //     printf("Demo test failed.\n");
    // }

    // // Create a wallet
    // Wallet *wallet = generate_wallet();

    // const char *address = "0xfa2fa73cdb69a883dd40792dc092d7af46d5c88072835373afff26885978b2ea";

    // Call the Rust function with the C string
    // Wallet *wallet = get_wallet_from_address(address);
    Wallet *wallet = generate_and_add_key();
    // Print the wallet details
    if (wallet)
    {
        printf("Wallet Address: %s\n", wallet->address);
        printf("Mnemonic: %s\n", wallet->mnemonic);
        printf("Public Base64 Key: %s\n", wallet->public_base64_key);
        printf("Private Key: %s\n", wallet->private_key);
        printf("Key Scheme: %s\n", wallet->key_scheme);

        // Free the wallet
        free_wallet(wallet);
    }
    else
    {
        printf("Failed to create wallet.\n");
    }

    // import_from_private_key("AMO4GD+YyWCFdKqh6aiE1rvm28wbI1jS53vHTHjNHHeZ");
    // import_from_mnemonic("biology orbit beef water polar evidence faith lake load glance bomb visa");

    return 0;
}