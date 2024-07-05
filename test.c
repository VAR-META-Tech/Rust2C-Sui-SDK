#include <stdio.h>
#include <stdlib.h>
#include "sui_lib.h"

int main()
{
    const char *addresses[] = {"0x013c740d731b06bb7447316e7b43ea6120d808d07cd0a8a0c6f391930bd449dd",
                               "0x2691bf90af73ce452f71ef081c1d8f00a9d8a3506101c5def54f6bed8c1be733",
                               "0x66e350a92a4ddf98906f4ae1a208a23e40047105f470c780d2d6bec139031f75"};
    uint8_t weights_data[] = {1, 1, 1};
    CStringArray cstring_array = {addresses, 3};
    CU8Array cu8_array = {weights_data, 3};
    uint16_t threshold = 2;
    MultiSig multisig = get_or_create_multisig(cstring_array, cu8_array, threshold);

    printf("MultiSig bytes: ");
    for (uint32_t i = 0; i < multisig.bytes.len; i++)
    {
        printf("%u ", multisig.bytes.data[i]);
    }

    const char *from_address = "0xbefabc05fd9339d24e7413db011cb9be62f852fd2ce6430c5c6852dac85e46cf";
    const char *to_address = "0x2691bf90af73ce452f71ef081c1d8f00a9d8a3506101c5def54f6bed8c1be733";
    uint64_t amount = 5400000000;

    CU8Array result = create_transaction(from_address, to_address, amount);

    if (result.error == NULL)
    {
        printf("Transaction created successfully.\n");
        printf("Data length: %u\n", result.len);
        printf("Data: ");
        for (uint32_t i = 0; i < result.len; i++)
        {
            printf("%02x", result.data[i]);
        }
        printf("\n");
    }
    else
    {
        printf("Error: %s\n", result.error);
        free((void *)result.error); // Free the error message
    }
    // Free the error message if it was allocated
    // if (result.error != NULL)
    // {
    //     free((void *)result.error);
    // }

    // CU8Array multisig_bytes = multisig.bytes;

    // uint8_t tx_data[] = {0x05, 0x06, 0x07, 0x08}; // Sample data
    // CU8Array tx = result;

    const char *addresses_data[] = {"0x013c740d731b06bb7447316e7b43ea6120d808d07cd0a8a0c6f391930bd449dd",
                                    "0x2691bf90af73ce452f71ef081c1d8f00a9d8a3506101c5def54f6bed8c1be733"};
    CStringArray addresses2 = {addresses_data, 2};

    // Call the Rust function
    const char *result2 = sign_and_execute_transaction(multisig.bytes, result, addresses2);

    if (result2 != NULL)
    {
        printf("Result: %s\n", result2);
        // Free the result when done
        free((void *)result2);
    }
    else
    {
        printf("Error occurred\n");
    }

    return 0;
}