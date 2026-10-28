#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
static inline int64_t add(int64_t a,int64_t b){int64_t r;if(__builtin_add_overflow(a,b,&r))exit(1);return r;}
static inline int64_t mul(int64_t a,int64_t b){int64_t r;if(__builtin_mul_overflow(a,b,&r))exit(1);return r;}
static inline int64_t sub(int64_t a,int64_t b){int64_t r;if(__builtin_sub_overflow(a,b,&r))exit(1);return r;}
static int64_t fib(int64_t n){if(n<2)return n;return add(fib(sub(n,1)),fib(sub(n,2)));}
int main(void){
printf("%lld\n",(long long)fib(28));
return 0;
}
