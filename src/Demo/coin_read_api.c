#include <stdio.h>

#include "sui_rust_wrapper_c.h"

int main()
{    
    //To Demo Coid Read API Function Please change Test.sh content:
    
    /*******************************
    cargo build --release
    gcc src/Demo/coin_read_api.c -L target/release/ -lsui_rust_sdk -o test
    ./test  
    ********************************/

    // Demo get_total_supply_sync 
    int64_t total_supply = get_total_supply_sync();
    printf("total_supply : %llu\n", total_supply);

    // Demo get_balance_sync 
    Balance balance = get_balance_sync();
    if (balance.coin_type == NULL) {
        printf("Failed to fetch balance.\n");
    } else {
        printf(" *** Balance ***\n");
        printf("Coin Type: %s\n", balance.coin_type);
        printf("Coin Object Count: %zu\n", balance.coin_object_count);
        __uint128_t total_balance = ((__uint128_t)balance.total_balance[1] << 64) | balance.total_balance[0];
        char total_balance_str[40]; // Enough to hold 2^128-1
        snprintf(total_balance_str, sizeof(total_balance_str), "%llu", total_balance);
        printf("Total Balance: %s\n", total_balance_str);
        printf(" *** Balance ***\n");
    }
    // Free allocated resources
    free_balance(balance);

    // Demo get_all_balances_sync
    BalanceArray balance_array = get_all_balances_sync();
    
    if (balance_array.balances == NULL) {
        printf("Failed to fetch balances.\n");
    } else {
        printf(" *** All Balances ***\n");
        for (size_t i = 0; i < balance_array.length; i++) {
            Balance balance = balance_array.balances[i];
            printf("Coin Type: %s\n", balance.coin_type);
            printf("Coin Object Count: %zu\n", balance.coin_object_count);
            __uint128_t total_balance = ((__uint128_t)balance.total_balance[1] << 64) | balance.total_balance[0];
            char total_balance_str[40]; // Enough to hold 2^128-1
            snprintf(total_balance_str, sizeof(total_balance_str), "%llu", total_balance);
            printf("Total Balance: %s\n", total_balance_str);
        }
        printf(" *** All Balances ***\n");
    }
    // Free allocated resources
    free_balance_array(balance_array);
    
    // Demo get_coins_sync
    CCoinArray coins = get_coins_sync();
    // Iterate over the coins and print their details
    for (size_t i = 0; i < coins.length; i++) {
        CCoin coin = coins.coins[i];
        printf("Coin %zu:\n", i);
        printf("  Coin Type: %s\n", coin.coin_type);
        printf("  Coin Object ID: "); // Print the coin object ID if needed
        for (size_t j = 0; j < sizeof(coin.coin_object_id); j++) {
            printf("%02X ", coin.coin_object_id[j]);
        }
        printf("\n");
        printf("  Version: %llu\n", coin.version);
        printf("  Digest: "); // Print the coin object ID if needed
        for (size_t j = 0; j < sizeof(coin.digest); j++) {
            printf("%02X ", coin.digest[j]);
        }
        printf("\n  Balance: %llu\n", coin.balance);
        
         printf("  Previous Transaction: "); // Print the coin object ID if needed
        for (size_t j = 0; j < sizeof(coin.previous_transaction); j++) {
            printf("%02X ", coin.previous_transaction[j]);
        }

       
    }
    // Free the allocated memory for the coins array
    free_coin_array(coins);
    return 0;
}