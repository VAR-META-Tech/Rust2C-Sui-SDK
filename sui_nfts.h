// rust_functions.h
#ifndef SUI_NFTS_H
#define SUI_NFTS_H
#include <stdint.h>
#include <stdio.h>
#include <stddef.h>
#include <inttypes.h>

#ifdef __cplusplus
extern "C"
{
#endif

    const char *mint_nft(
        const char *sender_address,
        const char *name,
        const char *description,
        const char *uri);

    const char *transfer_nft(
        const char *sender_address,
        const char *nft_id,
        const char *recipient_address);
#ifdef __cplusplus
}
#endif

#endif // SUI_NFTS_H
