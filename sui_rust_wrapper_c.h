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
    typedef struct {
    char* coin_type;
    size_t coin_object_count;
    uint64_t total_balance[2];
    } Balance;


    // Define the BalanceArray struct
    typedef struct {
    Balance* balances;
    size_t length;
    } BalanceArray;


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

#ifdef __cplusplus
}
#endif

#endif // RUST_FUNCTIONS_WRAPPER_H
