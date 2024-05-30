#include <stdio.h>

#include "sui_lib.h"

int main()
{

    WalletList wallet_list = get_wallets();

    for (size_t i = 0; i < wallet_list.length; ++i)
    {
        Wallet wallet = wallet_list.wallets[i];
        printf("Wallet %zu:\n", i + 1);
        printf("  Address: %s\n", wallet.address);
        printf("  Mnemonic: %s\n", wallet.mnemonic);
        printf("  Public Base64 Key: %s\n", wallet.public_base64_key);
        printf("  Private Key: %s\n", wallet.private_key);
        printf("  Key Scheme: %s\n", wallet.key_scheme);
    }

    free_wallet_list(wallet_list);

    return 0;
}