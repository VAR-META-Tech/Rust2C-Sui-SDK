#include <stdio.h>
#include <stdlib.h>
#include <assert.h>
#include "header/sui_lib.h"

// Define global constants
const char *FAUCET_ADDRESS = "0x013c740d731b06bb7447316e7b43ea6120d808d07cd0a8a0c6f391930bd449dd";
const char *SENDER_ADDRESS = "0x2107184d961804e3cbeef48106a7384d11d90f5a050fde0709da8e079450b824";
const char *SENDER_MNEMONIC = "grab away penalty eye treat embrace over hundred cave neither mix oxygen";
const char *SENDER_MNEMONIC_ALIAS = "";
const char *RECIPIENT_ADDRESS = "0xf0897c8c9dada307db3691b0ecda62107f9aaa2bc56d7978bd3f3814da75a5f2";
const char *SPONSER_ADDRESS = "0x013c740d731b06bb7447316e7b43ea6120d808d07cd0a8a0c6f391930bd449dd";
const char *PRIVATE_KEY_BASE64 = "AON/DOXYIjxYvQ5PN5v+dR0uTGedvwSI5D8NcNWKsmcXaa";
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
    ImportResult *result = import_from_mnemonic(SENDER_MNEMONIC, "ED25519", SENDER_MNEMONIC_ALIAS);
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

void test_programable_transactionbuilder()
{
    // Create a new builder
    CProgrammableTransactionBuilder *builder = create_builder();
    assert(builder != NULL);

    //
    CArguments *coin = create_arguments();
    add_argument_gas_coin(coin);

    CArguments *amount = create_arguments();
    make_pure(builder, amount, bsc_basic("u64", "1000000000"));

    add_split_coins_command(builder, coin, amount);

    // Add a transfer object command
    CArguments *agrument = create_arguments();
    add_argument_result(agrument, 0);
    CArguments *recipient = create_arguments();
    make_pure(builder, recipient, bsc_basic("address", RECIPIENT_ADDRESS));
    add_transfer_object_command(builder, agrument, recipient);

    // Execute the builder
    const char *result = execute_transaction(builder, SENDER_ADDRESS, 5000000);
    assert(result != NULL);
    printf("Result: %s\n", result);
}

void test_request_tokens_from_faucet()
{
    const char *response = request_tokens_from_faucet(FAUCET_ADDRESS);
    printf("Response from request faucet: %s\n", response);
}

int main()
{
    // test_request_tokens_from_faucet();
    // test_get_wallets();
    // test_generate_wallet();
    // test_generate_and_add_key();
    // test_import_from_private_key();
    // test_import_from_mnemonic();
    // test_get_wallet_from_address();
    // test_programmable_transaction();
    test_programmable_transaction_allow_sponser();
    // test_programable_transactionbuilder();

    return 0;
}