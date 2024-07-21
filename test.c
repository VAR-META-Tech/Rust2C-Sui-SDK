#include <stdio.h>
#include <stdlib.h>
#include "sui_lib.h"

int main()
{
    const char *package_id = "0xd1efbd86210322b550a8d6017ad5113fda2bd4f486593096f83e7b9ce3cbd002";

    // const char *sender_address = "0x013c740d731b06bb7447316e7b43ea6120d808d07cd0a8a0c6f391930bd449dd";
    // const char *name = "NgocNFT2";
    // const char *description = "NFT Description";
    // const char *uri = "https://letsenhance.io/static/8f5e523ee6b2479e26ecc91b9c25261e/1015f/MainAfter.jpg";
    // const char *result = mint_nft(package_id, sender_address, name, description, uri);

    // const char *sender_address = "0x013c740d731b06bb7447316e7b43ea6120d808d07cd0a8a0c6f391930bd449dd";
    // const char *nft_id = "0x68ce773d046e42959757a800af4db34ce5a725630841824d2fd02b08c86d476e";
    // const char *recipient_address = "0x66e350a92a4ddf98906f4ae1a208a23e40047105f470c780d2d6bec139031f75";
    // const char *result = transfer_nft(package_id, sender_address, nft_id, recipient_address);

    // if (result != NULL)
    // {
    //     printf("Result: %s\n", result);
    //     // Free the result when done
    //     free((void *)result);
    // }
    // else
    // {
    //     printf("Error occurred\n");
    // }

    CSuiObjectDataArray array =
        get_wallet_objects(
            "0x013c740d731b06bb7447316e7b43ea6120d808d07cd0a8a0c6f391930bd449dd",
            "0xd1efbd86210322b550a8d6017ad5113fda2bd4f486593096f83e7b9ce3cbd002::nft::NFT");

    if (array.data == NULL)
    {
        printf("Failed to get Sui object data list\n");
        return 1;
    }

    for (size_t i = 0; i < array.len; i++)
    {
        printf("Object ID: %s\n", array.data[i].object_id);
        printf("Version: %llu\n", array.data[i].version);
        printf("Digest: %s\n", array.data[i].digest);
        printf("Type: %s\n", array.data[i].type_);
        printf("Owner: %s\n", array.data[i].owner);
        printf("Previous Transaction: %s\n", array.data[i].previous_transaction);
        printf("Storage Rebate: %llu\n", array.data[i].storage_rebate);
        printf("Display: %s\n", array.data[i].display);
        printf("Content: %s\n", array.data[i].content);
        printf("BCS: %s\n", array.data[i].bcs);

        printf("\n");
    }

    free_sui_object_data_list(array);
    return 0;
}