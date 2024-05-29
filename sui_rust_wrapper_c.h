// rust_functions.h
#ifndef RUST_FUNCTIONS_WRAPPER_H
#define RUST_FUNCTIONS_WRAPPER_H
#include <stdint.h>
#include <stdio.h>
#include <stddef.h>
#include <inttypes.h>
#ifdef __cplusplus
extern "C"
{
#endif
    // Examples
    int connect_localnet_c(void);
    int connect_devnet_c(void);
    int connect_testnet_c(void);
    int coin_read_api(void);
    int event_api(void);
    int sui_clients(void);
    int test(void);

    // SuiClient
    // Setup
    // Struct to hold C-compatible string array
    typedef struct
    {
        const char **data;
        int len;
    } CStringArray;

    // Struct to hold the result, either CStringArray or error message
    typedef struct
    {
        CStringArray strings;
        const char *error;
    } ResultCStringArray;

    // Define the Balance struct
    typedef struct
    {
        char *coin_type;
        size_t coin_object_count;
        uint64_t total_balance[2];
    } Balance;

    // Define the BalanceArray struct
    typedef struct
    {
        Balance *balances;
        size_t length;
    } BalanceArray;

    // Define the C struct for Coin
    typedef struct
    {
        const char *coin_type;
        uint8_t coin_object_id[32];
        uint64_t version;
        uint8_t digest[32];
        uint64_t balance;
        uint8_t previous_transaction[32];
    } CCoin;

    // Define the C struct for an array of CCoin
    typedef struct
    {
        CCoin *coins;
        size_t length;
    } CCoinArray;
    void free_strings(CStringArray array);
    void free_error_string(const char *error);
    // SuiClient functions
    char *api_version();
    int check_api_version(void);
    ResultCStringArray available_rpc_methods();
    ResultCStringArray available_subscriptions();
    // Read Coin function
    uint64_t get_total_supply_sync();

    Balance get_balance_sync();
    void free_balance(Balance balance);

    // Declare the Rust functions
    extern BalanceArray get_all_balances_sync();
    extern void free_balance_array(BalanceArray balance_array);

    extern CCoinArray get_coins_sync();
    extern void free_coin_array(CCoinArray coins);

    // Wallet

    // Define the Wallet struct matching the Rust struct
    typedef struct
    {
        char *address;
        char *mnemonic;
        char *public_base64_key;
        char *private_key;
        char *key_scheme;
    } Wallet;

    // Declare the functions from the Rust library
    extern Wallet *generate_wallet();
    extern Wallet *generate_and_add_key();
    extern Wallet *get_wallet_from_address(const char *address);
    extern void free_wallet(Wallet *wallet);
    extern void import_from_private_key(const char *key_base64);
    extern void import_from_mnemonic(const char *mnemonic);
#ifdef __cplusplus
}
#endif

#endif // RUST_FUNCTIONS_WRAPPER_H
