#include <stdio.h>

#include "sui_lib.h"

int main()
{
    // // code here
    // int buildTestnetResult = buildTestnet();
    // if (buildTestnetResult == 0)
    // {
    //     printf("buildTestnet succeeded.\n");
    // }
    // else
    // {
    //     printf("buildTestnet failed.\n");
    // }
    // Get balances
    // BalanceArray balanceArray = get_balances("0x0cc4b15265e0a342a2822377258e3750ecea621172e580395674790b33844a6b");

    // // Iterate and print balances
    // for (size_t i = 0; i < balanceArray.length; ++i)
    // {
    //     Balance balance = balanceArray.balances[i];
    //     printf("Coin Type: %s\n", balance.coin_type);
    //     printf("Coin Object Count: %zu\n", balance.coin_object_count);
    //     printf("Total Balance Part 1: %llu\n", balance.total_balance[0]);
    //     printf("Total Balance Part 2: %llu\n", balance.total_balance[1]);
    // }

    // // Free allocated memory
    // free_balance_array(balanceArray);

    // Wallet *wallet = generate_wallet();
    // printf("address: %s\n", wallet->address);
    // printf("mnemonic: %s\n", wallet->mnemonic);
    // char *address = import_from_mnemonic(wallet->mnemonic);

   // printf("address: %s\n", address);
    request_tokens_from_faucet_("0x0ce3c2665570b4d9215eb4c36cf08f7817b2d2de6e0ab936b647bf4479f72576");
    const char* result = programmable_transaction("0x011def570767424175c98d5b47b668c28bd4b415f40c611a420ec07e73d125a9", "0x0ce3c2665570b4d9215eb4c36cf08f7817b2d2de6e0ab936b647bf4479f72576",50000);
    printf("%s\n", result);
    get_balances("0x0ce3c2665570b4d9215eb4c36cf08f7817b2d2de6e0ab936b647bf4479f72576");
      // Demo get_balance_sync 
    Balance balance = get_balance_sync("0x0ce3c2665570b4d9215eb4c36cf08f7817b2d2de6e0ab936b647bf4479f72576");
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
//   // Demo get_coins_sync
//     CCoinArray coins = get_coins_sync("0x011def570767424175c98d5b47b668c28bd4b415f40c611a420ec07e73d125a9");
//    // CCoinArray coins = get_coins_sync("0x21214e05a2bbc228e064bec68b6d21f3947a8993bf0f0c39d8ddba58335b5001");
//     // Iterate over the coins and print their details
//     for (size_t i = 0; i < coins.length; i++) {
//         CCoin coin = coins.coins[i];
//         printf("Coin %zu:\n", i);
//         printf("  Coin Type: %s\n", coin.coin_type);
//         printf("  Coin Object ID: %s\n",coin.coin_object_id); // Print the coin object ID if needed
//         // for (size_t j = 0; j < sizeof(coin.coin_object_id); j++) {
//         //     printf("%02X ", coin.coin_object_id[j]);
//         // }
//         printf("\n");
//         printf("  Version: %llu\n", coin.version);
//         printf("  Digest: %s\n,",coin.digest); // Print the coin object ID if needed
//         // for (size_t j = 0; j < sizeof(coin.digest); j++) {
//         //     printf("%02X ", coin.digest[j]);
//         // }
//         printf("\n  Balance: %llu\n", coin.balance);
        
//          printf("  Previous Transaction:  %s\n",coin.previous_transaction); // Print the coin object ID if needed
//         // for (size_t j = 0; j < sizeof(coin.previous_transaction); j++) {
//         //     printf("%02X ", coin.previous_transaction[j]);
//         // }

       
//     }
//       coins = get_coins_sync("0x0ce3c2665570b4d9215eb4c36cf08f7817b2d2de6e0ab936b647bf4479f72576");
//    // CCoinArray coins = get_coins_sync("0x21214e05a2bbc228e064bec68b6d21f3947a8993bf0f0c39d8ddba58335b5001");
//     // Iterate over the coins and print their details
//     for (size_t i = 0; i < coins.length; i++) {
//         CCoin coin = coins.coins[i];
//         printf("Coin %zu:\n", i);
//         printf("  Coin Type: %s\n", coin.coin_type);
//         printf("  Coin Object ID: %s\n",coin.coin_object_id); // Print the coin object ID if needed
//         // for (size_t j = 0; j < sizeof(coin.coin_object_id); j++) {
//         //     printf("%02X ", coin.coin_object_id[j]);
//         // }
//         printf("\n");
//         printf("  Version: %llu\n", coin.version);
//         printf("  Digest: %s\n,",coin.digest); // Print the coin object ID if needed
//         // for (size_t j = 0; j < sizeof(coin.digest); j++) {
//         //     printf("%02X ", coin.digest[j]);
//         // }
//         printf("\n  Balance: %llu\n", coin.balance);
        
//          printf("  Previous Transaction:  %s\n",coin.previous_transaction); // Print the coin object ID if needed
//         // for (size_t j = 0; j < sizeof(coin.previous_transaction); j++) {
//         //     printf("%02X ", coin.previous_transaction[j]);
//         // }

       
//     }
//     // Free the allocated memory for the coins array
//     free_coin_array(coins);
    return 0;
}