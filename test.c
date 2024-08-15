#include <stdio.h>
#include <stdlib.h>
#include "sui_lib.h"

void print_wallet(const Wallet *wallet)
{
    printf("Wallet Address: %s\n", wallet->address ? wallet->address : "Not set");
    printf("Mnemonic: %s\n", wallet->mnemonic ? wallet->mnemonic : "Not set");
    printf("Public Base64 Key: %s\n", wallet->public_base64_key ? wallet->public_base64_key : "Not set");
    printf("Private Key: %s\n", wallet->private_key ? wallet->private_key : "Not set");
    printf("Key Scheme: %s\n", wallet->key_scheme ? wallet->key_scheme : "Not set");
    printf("\n");
}
void test_get_wallets()
{
    // Retrieve the list of wallets
    WalletList wallet_list = get_wallets();

    // Check if the retrieval was successful
    if (wallet_list.wallets == NULL)
    {
        printf("Error retrieving wallets\n");
        return;
    }

    // Iterate over the wallets and print their details
    for (size_t i = 0; i < wallet_list.length; i++)
    {
        print_wallet(&wallet_list.wallets[i]);
    }

    // Free the memory allocated for the wallets
    free_wallet_list(wallet_list);
}

void test_generate_wallet()
{
    // Call the Rust function to generate a wallet
    Wallet *wallet = generate_wallet("ed25519", "word12");

    // Print the wallet details
    print_wallet(wallet);

    // Free the wallet when done
    if (wallet)
    {
        free_wallet(wallet); // This function should be implemented in Rust
    }
}

void test_generate_and_add_key()
{
    // Generate a new wallet and add a key
    Wallet *wallet = generate_and_add_key();

    // Check if the wallet generation was successful
    if (wallet == NULL)
    {
        printf("Failed to generate and add key\n");
        return;
    }

    // Print the wallet details
    print_wallet(wallet);

    // Free the wallet when done
    free_wallet(wallet);
}

void test_import_from_private_key(const char *key_base64)
{
    // Call the Rust function to import from a private key
    import_from_private_key(key_base64);
    printf("Private key imported successfully.\n");
}

void test_import_from_mnemonic(const char *mnemonic)
{
    // Call the Rust function to import from a mnemonic and get the address
    char *address = import_from_mnemonic(mnemonic);

    if (address == NULL)
    {
        printf("Failed to import mnemonic.\n");
    }
    else
    {
        printf("Imported Address: %s\n", address);
        // Free the memory allocated by Rust
        free(address);
    }
}

void test_get_wallet_from_address(const char *address)
{
    // Call the Rust function to get a wallet from an address
    Wallet *wallet = get_wallet_from_address(address);

    if (wallet == NULL)
    {
        printf("Failed to retrieve wallet for the given address.\n");
    }
    else
    {
        // Print the wallet details
        print_wallet(wallet);
        // Free the wallet when done
        free_wallet(wallet); // This function should be implemented in Rust
    }
}

int main()
{
    // test_get_wallets();
    // test_generate_wallet();
    // test_generate_and_add_key();
    // test_import_from_private_key("AON/DOXYIjxYvQ5PN5v+dR0uTGedvwSI5D8NcNWKsmcX");
    // test_import_from_mnemonic("urban blue h awk lecture clerk power craft episode bulk barrel venture almost");
    // test_get_wallet_from_address("0xfee0108a2467a551f50f3f7c2dc77128406ae314ef4515030dc62accb0c15bcc");

    return 0;
}