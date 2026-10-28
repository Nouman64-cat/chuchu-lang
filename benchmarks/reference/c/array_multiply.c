#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
static inline int64_t add(int64_t a,int64_t b){int64_t r;if(__builtin_add_overflow(a,b,&r))exit(1);return r;}
static inline int64_t mul(int64_t a,int64_t b){int64_t r;if(__builtin_mul_overflow(a,b,&r))exit(1);return r;}
static inline int64_t sub(int64_t a,int64_t b){int64_t r;if(__builtin_sub_overflow(a,b,&r))exit(1);return r;}

int main(void){
double a[64]={1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0,9.0,10.0,11.0,12.0,13.0,14.0,15.0,16.0,17.0,18.0,19.0,20.0,21.0,22.0,23.0,24.0,25.0,26.0,27.0,28.0,29.0,30.0,31.0,32.0,33.0,34.0,35.0,36.0,37.0,38.0,39.0,40.0,41.0,42.0,43.0,44.0,45.0,46.0,47.0,48.0,49.0,50.0,51.0,52.0,53.0,54.0,55.0,56.0,57.0,58.0,59.0,60.0,61.0,62.0,63.0,64.0};double total=0.0;int64_t i=0;while(i<30000){double product[64];for(int j=0;j<64;j++)product[j]=a[j]*a[j];double sum=0.0;for(int j=0;j<64;j++)sum+=product[j];total+=sum;for(int j=0;j<64;j++)a[j]=a[j]+1.0;i=add(i,1);}printf("%.15g\n",total);
return 0;
}
