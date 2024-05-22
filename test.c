#include <stdio.h>

#include "sui_rust_wrapper_c.h"

int main()
{

    const char *version = api_version();

    printf("Get api Version : %s\n", version);

    int check_api_version_result = check_api_version();
    if (check_api_version_result == 0)
    {
        printf("Api version match.\n");
    }
    else
    {
        printf("API Version not match.\n");
    }

    int test_result = test();
    if (test_result == 0)
    {
        printf("Demo test succeeded.\n");
    }
    else
    {
        printf("Demo test failed.\n");
    }

    return 0;
}