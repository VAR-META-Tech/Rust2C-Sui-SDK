// rust_functions.h
#ifndef SUI_TRANSACTION_H
#define SUI_TRANSACTION_H
#include <stdint.h>
#include <stdio.h>
#include <stddef.h>
#include <inttypes.h>
#ifdef __cplusplus
extern "C"
{
#endif
    extern char *programmable_transaction(const char *sender_address, const char *recipient_address, uint64_t amount);
    extern char *programmable_transaction_allow_sponser(const char *sender_address, const char *recipient_address, uint64_t amount, const char *sponser_address);
    extern const char *request_tokens_from_faucet_(const char *address_str);
#ifdef __cplusplus
}
#endif

#endif // SUI_TRANSACTION_H
