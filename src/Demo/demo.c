#include <stdio.h>

#include "sui_rust_wrapper_c.h"

int main()
{

    // Call the Rust function to get the string array
    ResultCStringArray result = available_rpc_methods();

    // Check if there was an error
    if (result.error != NULL)
    {
        printf("Error: %s\n", result.error);
        free_error_string(result.error);
    }
    else
    {
        // Print the strings
        for (int i = 0; i < result.strings.len; i++)
        {
            printf("String %d: %s\n", i, result.strings.data[i]);
        }

        // Free the allocated string array memory
        free_strings(result.strings);
    }

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