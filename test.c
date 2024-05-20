#include <stdio.h>

#include "sui_rust_wrapper_c.h"

int main()
{
    // Demo Connet testnet
    int connetTestNet = async_connects_testnet_c();
    if (connetTestNet == 0)
    {
        printf("connetTestNet succeeded.\n");
    }
    else
    {
        printf("connetTestNet failed.\n");
    }
    // Demo Connet devnet
    int connetDevNet = async_connects_devnet_c();
    if (connetDevNet == 0)
    {
        printf("connetDevNet succeeded.\n");
    }
    else
    {
        printf("connetDevNet failed.\n");
    }
    // Demo Coin Read API
    int coin_read_api_result = coin_read_api();
    if (coin_read_api_result == 0)
    {
        printf("Demo Coin Read API succeeded.\n");
    }
    else
    {
        printf("Demo Coin Read API failed.\n");
    }
    // Demo Event API
    // int event_api_result = event_api();
    //     if (event_api_result == 0)
    //     {
    //         printf("Demo Event API succeeded.\n");
    //     }
    //     else
    //     {
    //         printf("Demo Event API failed.\n");
    //     }
    return 0;
}