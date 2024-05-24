#include <stdio.h>

#include "sui_rust_wrapper_c.h"

int main()
{

    // Call the Rust function to get the string array
    ResultCStringArray result = available_rpc_methods();

    // Check if there was an error
    if (result.error != NULL)
    {
        printf("Error: %s\n", result.error);
        free_error_string(result.error);
    }
    else
    {
        // Print the strings
        for (int i = 0; i < result.strings.len; i++)
        {
            printf("String %d: %s\n", i, result.strings.data[i]);
        }

        // Free the allocated string array memory
        free_strings(result.strings);
    }

    const char *version = api_version();

    printf("Get api Version : %s\n", version);

    int check_api_version_result = check_api_version();
    if (check_api_version_result == 0)
    {
        printf("Api version match.\n");
    }
    else
    {
        printf("API Version not match.\n");
    }

    int test_result = test();
    if (test_result == 0)
    {
        printf("Demo test succeeded.\n");
    }
    else
    {
        printf("Demo test failed.\n");
    }
    int64_t total_supply = get_total_supply_sync();
    printf("total_supply : %llu\n", total_supply);

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
    
    return 0;
}