// rust_functions.h
#ifndef SUI_BALANCE_H
#define SUI_BALANCE_H
#include <stdint.h>
#include <stdio.h>
#include <stddef.h>
#include <inttypes.h>
#ifdef __cplusplus
extern "C"
{
#endif

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

    extern BalanceArray get_balances(const char *address);
    extern void free_balance_array(BalanceArray balance_array);

#ifdef __cplusplus
}
#endif

#endif // SUI_BALANCE_H
