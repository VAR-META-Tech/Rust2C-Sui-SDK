#include <stdio.h>

#include "sui_rust_wrapper_c.h"

int main()
{    
    //To Demo Coid Read API Function Please change Test.sh content:
    
    /*******************************
    cargo build --release
    gcc src/Demo/sui_client.c -L target/release/ -lsui_rust_sdk -o test
    ./test  
    ********************************/

    // Demo Connet testnet 
 int connetTestNet = connect_testnet_c();
 if (connetTestNet == 0)
    {
        printf("connetTestNet succeeded.\n");
    }
    else
    {
        printf("connetTestNet failed.\n");
    }
// Demo Connet devnet 
    int connetDevNet = connect_devnet_c();
    if (connetDevNet == 0)
    {
        printf("connetDevNet succeeded.\n");
    }
    else
    {
        printf("connetDevNet failed.\n");
    }
    return 0;
}
