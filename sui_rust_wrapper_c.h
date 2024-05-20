// rust_functions.h
#ifndef RUST_FUNCTIONS_WRAPPER_H
#define RUST_FUNCTIONS_WRAPPER_H

#ifdef __cplusplus
extern "C" {
#endif

int async_connects_testnet_c();
int async_connects_devnet_c();
int connect_sui(void);
int coin_read_api(void);
int event_api(void);
int sui_clients(void);

#ifdef __cplusplus
}
#endif

#endif // RUST_FUNCTIONS_WRAPPER_H

