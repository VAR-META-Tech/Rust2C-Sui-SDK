#include <stdio.h>
#include <stdlib.h>
#include "sui_lib.h"

int main()
{
    // const char *sender_address = "0x013c740d731b06bb7447316e7b43ea6120d808d07cd0a8a0c6f391930bd449dd";
    // const char *name = "NgocNFT";
    // const char *description = "NFT Description";
    // const char *uri = "https://letsenhance.io/static/8f5e523ee6b2479e26ecc91b9c25261e/1015f/MainAfter.jpg";

    // const char *result = mint_nft(sender_address, name, description, uri);

    const char *sender_address = "0x013c740d731b06bb7447316e7b43ea6120d808d07cd0a8a0c6f391930bd449dd";
    const char *nft_id = "0xd4ad4265be8b2055ffa81f2934213a025fd02fc226fba270ce7b83cc101ff970";
    const char *recipient_address = "0x66e350a92a4ddf98906f4ae1a208a23e40047105f470c780d2d6bec139031f75";

    const char *result = transfer_nft(sender_address, nft_id, recipient_address);

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