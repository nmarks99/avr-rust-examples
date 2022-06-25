#include <stdio.h>

#define BUFF_SIZE 10

void fill_buffer(char *buff){ 
    int i;
    for (i = 0; i < BUFF_SIZE; i++) {
        buff[i] = (char)i;
    }
}


int main(void) {
    char buff[BUFF_SIZE];

    fill_buffer(buff);

    int i;
    for (i = 0; i < BUFF_SIZE; i++) {
        printf("%d\n",buff[i]);
        // should print out 0 through 9
    }
    return 0;
}