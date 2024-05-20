#include <stdio.h>

extern int connect_sui(void);
extern int coin_read_api(void);
extern int event_api(void);
extern int sui_clients(void);

int main()
{

    int result2 = coin_read_api();
    if (result2 == 0)
    {
        printf("Task succeeded.\n");
    }
    else
    {
        printf("Task failed.\n");
    }
    return 0;
}