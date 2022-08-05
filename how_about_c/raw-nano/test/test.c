#include <string.h>
#include <stdio.h>
#include <stdint.h>
#include <math.h>

#define my_macro(x)
#if (x > 2)
    printf("Hi\n");
#else 
    printf("bye\n");
#endif




int main(void) {

    int TICKS_PER_MS = 250;
    int MAX_TICKS = 65536;
    float ms = 1000.0;

    float desired_ticks = (uint32_t)(ms*TICKS_PER_MS);
    uint8_t desired_overflows = (uint8_t)(floor(desired_ticks/MAX_TICKS));
    uint32_t remaining_ticks = (uint32_t)desired_ticks % MAX_TICKS;
    uint8_t current_overflow = 0;
    unsigned short current_ticks;


    char m[100];
    // desired_ticks = (uint32_t)desired_ticks;
    sprintf(m,"des_ticks = %lu\ndes_overs = %d\nremaining_ticks = %lu\n",(uint32_t)desired_ticks,desired_overflows,remaining_ticks);
    printf(m);


}