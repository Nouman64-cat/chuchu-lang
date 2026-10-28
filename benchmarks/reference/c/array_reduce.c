#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
static inline int64_t add(int64_t a,int64_t b){int64_t r;if(__builtin_add_overflow(a,b,&r))exit(1);return r;}
static inline int64_t mul(int64_t a,int64_t b){int64_t r;if(__builtin_mul_overflow(a,b,&r))exit(1);return r;}
static inline int64_t sub(int64_t a,int64_t b){int64_t r;if(__builtin_sub_overflow(a,b,&r))exit(1);return r;}

int main(void){
int64_t a[64]={1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59,60,61,62,63,64};int64_t total=0;int64_t i=0;while(i<30000){int64_t sum=0;for(int j=0;j<64;j++)sum=add(sum,a[j]);total=add(total,sum);for(int j=0;j<64;j++)a[j]=add(a[j],1);i=add(i,1);}printf("%lld\n",(long long)total);
return 0;
}
