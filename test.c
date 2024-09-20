#include <stdio.h>
#include <stdlib.h>
#include <assert.h>
#include "header/sui_lib.h"

// Define global constants
const char *SENDER_ADDRESS = "0x66e350a92a4ddf98906f4ae1a208a23e40047105f470c780d2d6bec139031f75";
const char *RECIPIENT_ADDRESS = "0xfee0108a2467a551f50f3f7c2dc77128406ae314ef4515030dc62accb0c15bcc";
const char *SPONSER_ADDRESS = "0xf662c23f80fbf0e613a8b2fb6c21e1eac198bb83cdeb12720b0404447ed62e3c";
const char *PRIVATE_KEY_BASE64 = "AON/DOXYIjxYvQ5PN5v+dR0uTGedvwSI5D8NcNWKsmcX";
const char *MNEMONIC = "unhappy above olympic pig brick embark chest crisp sheriff awful frown smooth";

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
    WalletList wallet_list = get_wallets();
    assert(wallet_list.wallets != NULL);

    for (size_t i = 0; i < wallet_list.length; i++)
    {
        print_wallet(&wallet_list.wallets[i]);
    }

    free_wallet_list(wallet_list);
}

void test_generate_wallet()
{
    Wallet *wallet = generate_wallet("ed25519", "word12");
    assert(wallet != NULL);
    print_wallet(wallet);
    free_wallet(wallet);
}

void test_generate_and_add_key()
{
    Wallet *wallet = generate_and_add_key();
    assert(wallet != NULL);
    print_wallet(wallet);
    free_wallet(wallet);
}

void test_import_from_private_key()
{
    ImportResult *result = import_from_private_key(PRIVATE_KEY_BASE64);
    assert(result != NULL);
    printf("Status: %d\n", result->status);
    printf("Address: %s\n", result->address);
    printf("Error: %s\n", result->error);
    free(result);
}

void test_import_from_mnemonic()
{
    ImportResult *result = import_from_mnemonic(MNEMONIC, "ED25519");
    assert(result != NULL);
    printf("Status: %d\n", result->status);
    printf("Address: %s\n", result->address);
    printf("Error: %s\n", result->error);
    free(result);
}

void test_get_wallet_from_address()
{
    Wallet *wallet = get_wallet_from_address(RECIPIENT_ADDRESS);
    assert(wallet != NULL);
    print_wallet(wallet);
    free_wallet(wallet);
}

void test_programmable_transaction()
{
    unsigned long long amount = 1000000000;
    const char *result = programmable_transaction(SENDER_ADDRESS, RECIPIENT_ADDRESS, amount);
    assert(result != NULL);
    printf("Result: %s\n", result);
    free((void *)result);
}

void test_programmable_transaction_allow_sponser()
{
    unsigned long long amount = 5400000000;
    const char *result = programmable_transaction_allow_sponser(SENDER_ADDRESS, RECIPIENT_ADDRESS, amount, SPONSER_ADDRESS);
    assert(result != NULL);
    printf("Result: %s\n", result);
    free((void *)result);
}

int main()
{
    test_get_wallets();
    test_generate_wallet();
    test_generate_and_add_key();
    test_import_from_private_key();
    test_import_from_mnemonic();
    test_get_wallet_from_address();
    test_programmable_transaction();
    test_programmable_transaction_allow_sponser();

    return 0;
}