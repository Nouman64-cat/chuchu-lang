#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
static inline int64_t add(int64_t a,int64_t b){int64_t r;if(__builtin_add_overflow(a,b,&r))exit(1);return r;}
static inline int64_t mul(int64_t a,int64_t b){int64_t r;if(__builtin_mul_overflow(a,b,&r))exit(1);return r;}
static inline int64_t sub(int64_t a,int64_t b){int64_t r;if(__builtin_sub_overflow(a,b,&r))exit(1);return r;}
static int64_t square_plus(int64_t x){return add(mul(x,x),3);}
int main(void){
int64_t i=0,total=0;while(i<300000){total=add(total,square_plus(i));i=add(i,1);}printf("%lld\n",(long long)total);
return 0;
}
