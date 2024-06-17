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

    Wallet *wallet = generate_wallet();
    printf("address: %s\n", wallet->address);
    printf("mnemonic: %s\n", wallet->mnemonic);
    char *address = import_from_mnemonic("laundry blade senior polar hand dismiss plate cycle bar appear kitten bless");

    printf("address: %s\n", address);
    return 0;
}