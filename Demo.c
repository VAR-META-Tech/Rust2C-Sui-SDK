#include <stdio.h>
#include <stdlib.h>
#include "header/sui_lib.h"

int main()
{
    const char *sender_address = "0x47d27bcd44c9d68b3dfee3827b594feb47c44fe9a2279e0cb3d19ca9f754a4da";
    const char *recipient_address = "0x4930ad3c0669150e474f2aeead1eac16919105720b92566c247044f523547441";
    const char *sponser_address = "0x6a7f8196917d9452c2331d8e186e55df604f17dfdebf042ddfd76287137bd5e2";
    int64_t amount = 500000000;

    const char *result = programmable_transaction_allow_sponser(sender_address, recipient_address, amount, sponser_address);
    if (result != NULL)
    {
        printf("Result: %s\n", result);
        // Free the result when done
        free((void *)result);
    }
    else
    {
        printf("Error occurred\n");
    }
    return 0;
}