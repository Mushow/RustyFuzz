#include <stdio.h>
#include <string.h>

void vulnerable_function(const char *input) {
    char buffer[50];
    strcpy(buffer, input);  // Potential buffer overflow
}
