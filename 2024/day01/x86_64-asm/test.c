// used for testing some of the assembly functions
#include <stdint.h>
#include <stdio.h>

extern void sort(long* arr, size_t len);
extern long count_occurance(long val, long* arr, long len);

#define LEN 6

int main() {
    long arr[LEN] = {1, 5, 3, 4, 8, 5};

    sort(arr, LEN);
    long r = count_occurance(5, arr, LEN);

    printf("%ld\n", r);

    return 0;
}
