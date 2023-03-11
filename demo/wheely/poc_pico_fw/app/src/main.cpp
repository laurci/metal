#include <stdio.h>
#include <inttypes.h>
#include <pico/stdlib.h>
#include "pico/multicore.h"

float f1 = 0.0f, f2 = 0.0f, f3 = 0.8f;
float f4, f5, f6;

void core1_entry(void)
{
    while (true)
    {
        printf("core1: hello! \n");
    }
}

int main()
{
    char input[100];
    char c;
    uint8_t idx;

    stdio_init_all();

    multicore_launch_core1(core1_entry);

    while (1)
    {
        if (stdio_usb_connected())
        {
            c = getchar_timeout_us(10);
            // printf("%c", c);
            // if (c != PICO_ERROR_TIMEOUT && c != 10)
            if (c == '(')
            {
                idx = 0;
                while (c != PICO_ERROR_TIMEOUT && c != 10)
                {
                    input[idx] = c;
                    ++idx;
                    c = getchar_timeout_us(10);
                }
                input[idx] = 10;
                // printf("input: %s\n", input);

                sscanf(input, "(%f %f %f)\n", &f1, &f2, &f3);
            }
        }

        // sleep_ms(100);
    }
}