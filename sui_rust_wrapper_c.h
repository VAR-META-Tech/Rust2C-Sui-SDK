// rust_functions.h
#ifndef RUST_FUNCTIONS_WRAPPER_H
#define RUST_FUNCTIONS_WRAPPER_H

#ifdef __cplusplus
extern "C" {
#endif

int connect_localnet_c(void);
int connect_devnet_c(void);
int connect_testnet_c(void);

int coin_read_api(void);
int event_api(void);
int sui_clients(void);
int test(void);

#ifdef __cplusplus
}
#endif

#endif // RUST_FUNCTIONS_WRAPPER_H

