#include <stdio.h>

extern int perform_async_sample_task_sync(void);

int main()
{

    int result2 = perform_async_sample_task_sync();
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