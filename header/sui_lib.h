#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct CBalance {
  const char *coin_type;
  uintptr_t coin_object_count;
  uint64_t total_balance[2];
} CBalance;

typedef struct CBalanceArray {
  const struct CBalance *balances;
  uintptr_t length;
} CBalanceArray;

typedef struct CCoin {
  char *coin_type;
  char *coin_object_id;
  uint64_t version;
  char *digest;
  uint64_t balance;
  char *previous_transaction;
} CCoin;

typedef struct CCoinArray {
  const struct CCoin *coins;
  uintptr_t length;
} CCoinArray;

typedef struct CU8Array {
  const unsigned char *data;
  unsigned int len;
  const char *error;
} CU8Array;

typedef struct CMultiSig {
  const char *address;
  struct CU8Array bytes;
  const char *error;
} CMultiSig;

typedef struct CStringArray {
  const char *const *data;
  int len;
} CStringArray;

typedef struct ResultCStringArray {
  struct CStringArray strings;
  const char *error;
} ResultCStringArray;

typedef struct Wallet {
  char *address;
  char *mnemonic;
  char *public_base64_key;
  char *private_key;
  char *key_scheme;
} Wallet;

typedef struct WalletList {
  struct Wallet *wallets;
  uintptr_t length;
} WalletList;

typedef struct ImportResult {
  int status;
  char *address;
  char *error;
} ImportResult;

typedef struct CSuiObjectData {
  char *object_id;
  uint64_t version;
  char *digest;
  char *type_;
  char *owner;
  char *previous_transaction;
  uint64_t storage_rebate;
  char *display;
  char *content;
  char *bcs;
} CSuiObjectData;

typedef struct CSuiObjectDataArray {
  struct CSuiObjectData *data;
  uintptr_t len;
} CSuiObjectDataArray;

int32_t coin_read_api(void);

uint64_t get_total_supply_sync(void);

struct CBalance get_balance_sync(const char *address);

void free_balance(struct CBalance balance);

void free_balance_array(struct CBalanceArray balance_array);

struct CBalanceArray get_all_balances_sync(const char *address);

struct CBalanceArray get_balances(const char *address);

void free_coin_array(struct CCoinArray coin_array);

struct CCoinArray get_coins_sync(const char *address);

void free_multisig(struct CMultiSig multisig);

struct CMultiSig get_or_create_multisig(struct CStringArray addresses,
                                        struct CU8Array weights,
                                        uint16_t threshold);

int32_t test(void);

int32_t build_testnet(void);

int32_t build_devnet(void);

struct ResultCStringArray available_rpc_methods(void);

struct ResultCStringArray available_subscriptions(void);

int32_t check_api_version(void);

const char *api_version(void);

int32_t connect_localnet_c(void);

int32_t connect_devnet_c(void);

int32_t connect_testnet_c(void);

const char *sign_and_execute_transaction(struct CU8Array multisig,
                                         struct CU8Array tx,
                                         struct CStringArray addresses);

const char *mint_nft(const char *package_id,
                     const char *sender_address,
                     const char *name,
                     const char *description,
                     const char *uri);

const char *transfer_nft(const char *package_id,
                         const char *sender_address,
                         const char *nft_id,
                         const char *recipient_address);

struct CU8Array create_transaction(const char *from_address,
                                   const char *to_address,
                                   uint64_t amount);

const char *programmable_transaction(const char *sender_address,
                                     const char *recipient_address,
                                     uint64_t amount);

const char *programmable_transaction_allow_sponser(const char *sender_address,
                                                   const char *recipient_address,
                                                   uint64_t amount,
                                                   const char *sponser_address);

const char *request_tokens_from_faucet(const char *address_str);

struct WalletList get_wallets(void);

void free_wallet_list(struct WalletList wallet_list);

void free_wallet(struct Wallet *wallet);

struct Wallet *generate_wallet(const char *key_scheme, const char *word_length);

struct Wallet *generate_and_add_key(void);

struct ImportResult *import_from_mnemonic(const char *mnemonic, const char *sig_scheme);

struct ImportResult *import_from_private_key(const char *key_base64);

struct Wallet *get_wallet_from_address(const char *address);

struct CSuiObjectDataArray get_wallet_objects(const char *address, const char *object_type);

void free_strings(struct CStringArray array);

void free_sui_object_data_list(struct CSuiObjectDataArray array);

void free_error_string(const char *error);
